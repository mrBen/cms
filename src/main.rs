use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;
use std::io::prelude::*;
use std::{
    collections::HashMap,
    io,
    path::{Path, PathBuf},
};
use tmdb_async::Client;
use walkdir::{Error, WalkDir};

lazy_static! {
    static ref NUMBERING: Regex = Regex::new(r"[Ss](\d+)[Ee](\d+)").unwrap();
}

/// Store an episode video file.
#[derive(Debug)]
struct Episode {
    season: i32,
    number: i32,
    path: PathBuf,
}
impl Episode {
    fn from_path(path: &Path) -> Episode {
        let caps = NUMBERING
            .captures(path.file_name().unwrap().to_str().unwrap())
            .unwrap();

        Episode {
            season: caps.get(1).unwrap().as_str().parse().unwrap(),
            number: caps.get(2).unwrap().as_str().parse().unwrap(),
            path: PathBuf::from(path),
        }
    }
}

/// Organize your series and movies.
#[derive(Parser)]
struct Cli {
    /// The folder to scan for videos
    folder: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Cli::parse();

    let videos = list_videos(&args.folder);

    let mut episodes = HashMap::new();
    for video in &videos {
        let file_name = video.file_name().unwrap().to_str().unwrap();
        if NUMBERING.is_match(file_name) {
            let show_name = NUMBERING
                .split(file_name)
                .collect::<Vec<&str>>()
                .first()
                .unwrap()
                .clone();

            if !episodes.contains_key(show_name) {
                episodes.insert(show_name, Vec::new());
            }

            episodes
                .get_mut(show_name)
                .unwrap()
                .push(Episode::from_path(video.as_path()));
        }
    }

    let tmdb = Client::new(env!("TMDB_API_KEY").to_string());

    for (show_name, episodes) in episodes {
        organize(show_name, episodes, &args.folder, &tmdb).await;
    }

    Ok(())
}

/// Recursively list all video files in a directory.
fn list_videos(folder: &PathBuf) -> Vec<PathBuf> {
    let mut videos = Vec::new();

    for entry in WalkDir::new(folder).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            // TODO: use `mime_classifier`
            match entry.path().extension() {
                Some(ext) => {
                    if ext == "mp4" || ext == "mkv" {
                        videos.push(entry.into_path())
                    }
                }
                None => {}
            }
        }
    }

    videos
}

/// Move a show (list of videos) to proper location.
async fn organize(show_name: &str, mut episodes: Vec<Episode>, root: &Path, tmdb: &Client) {
    println!();
    episodes.sort_by_key(|e| (e.season, e.number));
    for episode in &episodes {
        println!("{}", episode.path.strip_prefix(&root).unwrap().display());
    }
    if let Some(show) = choose_show(show_name, tmdb).await {
        for episode in episodes {
            store(episode, show, tmdb).await;
        }
    }
}

/// Mimic Python's `input(prompt)`.
fn input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .map(|x| x.trim_end().to_owned())
}

/// Ask user which show the videos belongs.
async fn choose_show(show_name: &str, tmdb: &Client) -> Option<u32> {
    let query = show_name
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { ' ' })
        .collect::<String>()
        .trim()
        .to_owned();

    println!();
    let mut shows: Vec<u32> = Vec::new();
    let result = tmdb
        .tv_search(&query, None)
        .await
        .expect("API called failed");
    for (i, show) in result.results.iter().enumerate() {
        let year = show.first_air_date;
        let poster = show.poster_path.as_ref().expect("no poster");
        println!(
            "{}. {} ({}) https://image.tmdb.org/t/p/w500{}",
            i + 1,
            show.original_name,
            year,
            poster
        );
        shows.push(show.id);
    }
    let choice = input("\nQuel Série correspond ? ").unwrap();
    if choice == "skip" {
        None
    } else {
        Some(shows[choice.parse::<usize>().unwrap() - 1])
    }
}

/// Copy an episode file to it's correct location.
async fn store(episode: Episode, show: u32, tmdb: &Client) {
    println!("{:?}", tmdb.tv_by_id(show, false, false).await.unwrap());
}

use anyhow::Result;
use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::{copy, create_dir_all};
use std::io::prelude::*;
use std::{
    collections::HashMap,
    io,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

mod tmdb;

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
    fn from_path(path: &Path) -> Option<Episode> {
        let caps = NUMBERING.captures(path.file_name()?.to_str()?)?;

        Some(Episode {
            season: caps.get(1)?.as_str().parse().unwrap(),
            number: caps.get(2)?.as_str().parse().unwrap(),
            path: PathBuf::from(path),
        })
    }
}

/// Organize your series and movies.
#[derive(Parser)]
struct Cli {
    /// The folder to scan for videos
    folder: PathBuf,

    /// Perform a trial run with no changes made
    #[arg(short, long)]
    dry_run: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
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
                .to_owned();

            if !episodes.contains_key(show_name) {
                episodes.insert(show_name, Vec::new());
            }

            if let Some(episode) = Episode::from_path(video.as_path()) {
                if let Some(show) = episodes.get_mut(show_name) {
                    show.push(episode);
                }
            }
        }
    }

    for (show_name, episodes) in episodes {
        organize(show_name, episodes, &args.folder, args.dry_run).await?;
    }

    Ok(())
}

/// Recursively list all video files in a directory.
fn list_videos(folder: &PathBuf) -> Vec<PathBuf> {
    let mut videos = Vec::new();

    for entry in WalkDir::new(folder).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            // TODO: use `mime_classifier`
            if let Some(ext) = entry.path().extension() {
                if ext == "mp4" || ext == "mkv" {
                    videos.push(entry.into_path())
                }
            }
        }
    }

    videos
}

/// Move a show (list of videos) to proper location.
async fn organize(
    show_name: &str,
    mut episodes: Vec<Episode>,
    root: &Path,
    dry_run: bool,
) -> Result<()> {
    println!();
    episodes.sort_by_key(|e| (e.season, e.number));
    for episode in &episodes {
        println!("{}", episode.path.strip_prefix(root)?.display());
    }
    if let Some((show, show_name)) = choose_show(show_name).await? {
        for episode in episodes {
            store(episode, show, &show_name, dry_run).await?;
        }
    }

    Ok(())
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
async fn choose_show(show_name: &str) -> Result<Option<(i32, String)>> {
    let query = show_name
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { ' ' })
        .collect::<String>()
        .trim()
        .to_owned();

    println!();
    let mut shows: Vec<(i32, String)> = Vec::new();
    let results = tmdb::search::search_tv_shows(&query).await?;
    for (i, show) in results.results.iter().enumerate() {
        let year = &show.first_air_date;
        let poster_path = tmdb::poster(&show.poster_path);
        println!(
            "{}. {} ({}) {}",
            i + 1,
            show.original_name,
            year,
            poster_path
        );
        shows.push((show.id, show.original_name.to_string()));
    }
    let choice = input("\nQuel Série correspond ? ")?;
    if choice == "skip" {
        Ok(None)
    } else {
        Ok(Some(shows[choice.parse::<usize>()? - 1].clone()))
    }
}

/// Copy an episode file to it's correct location.
async fn store(episode: Episode, show_id: i32, show_name: &str, dry_run: bool) -> Result<()> {
    let info = tmdb::tv_episodes::get_details(show_id, episode.season, episode.number).await?;
    let season = format!("{:02}", info.season_number);
    let number = format!("{:02}", info.episode_number);
    let name = if !info.name.is_empty() {
        format!(" - {}", info.name)
    } else {
        String::from("")
    };

    let mut dest = dirs::video_dir().unwrap();
    dest.push("Series");
    dest.push(correct_file_name(show_name));
    dest.push(format!("Season {}", season));
    create_dir_all(&dest)?;
    dest.push(correct_file_name(&format!(
        "{} - s{}e{}{}.{}",
        show_name,
        season,
        number,
        name,
        episode.path.extension().unwrap().to_str().unwrap()
    )));

    println!(
        "Copy {:?} to {:?}",
        episode.path.file_name().unwrap(),
        dest.file_name().unwrap()
    );
    if !dry_run {
        copy(episode.path, dest)?;
    }

    Ok(())
}

/// Correct file name to valid Windows name.
fn correct_file_name(name: &str) -> String {
    name.replace(['<', '>', ':', '"', '/', '\\', '|', '?', '*'], "_")
}

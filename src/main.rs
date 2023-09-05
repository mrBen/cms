#![warn(clippy::pedantic)]

mod cms;
mod tmdb;

use anyhow::Result;
use clap::Parser;
use cms::shows;
use regex::Regex;
use std::{
    fs::{copy, create_dir_all},
    io::{self, prelude::*},
    path::{Path, PathBuf},
};

/// Organize your series and movies.
#[derive(Parser)]
struct Cli {
    /// The folder to scan for videos
    folder: PathBuf,

    /// Perform a trial run with no changes made
    #[arg(short, long)]
    dry_run: bool,

    /// Only process movies
    #[arg(short, long)]
    movies: bool,

    /// Only process shows
    #[arg(short, long)]
    shows: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let videos = cms::list_videos(&args.folder);

    let (films, episodes) = cms::pre_sort(&videos);

    if args.movies || !args.shows {
        for film in films {
            if let Some(file) = film.src.file_name() {
                println!("{:?}", choose_movie(file.to_str().unwrap()).await?);
            }
        }
    }

    if args.shows || !args.movies {
        for (show_name, episodes) in episodes {
            organize(&show_name, episodes, &args.folder, args.dry_run).await?;
        }
    }

    Ok(())
}

/// Move a show (list of videos) to proper location.
async fn organize(
    show_name: &str,
    mut episodes: Vec<shows::Episode>,
    root: &Path,
    dry_run: bool,
) -> Result<()> {
    println!();
    episodes.sort_by_key(|e| (e.season, e.number));
    for episode in &episodes {
        println!("{}", episode.src.strip_prefix(root)?.display());
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
    print!("{prompt}");
    io::stdout().flush()?;
    io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .map(|x| x.trim_end().to_owned())
}

/// Ask user which show the videos belongs.
async fn choose_movie(filename: &str) -> Result<Option<(i32, String)>> {
    let year = Regex::new(r"(?:19|20)\d{2}").unwrap(); // year between 1900 and 2099

    let query = year
        .split(filename)
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .to_owned()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { ' ' })
        .collect::<String>()
        .trim()
        .to_owned();

    println!();
    println!("{filename}");
    println!();
    let mut movies: Vec<(i32, String)> = Vec::new();
    let results = tmdb::search::movies(&query).await?;
    for (i, movie) in results.results.iter().enumerate() {
        let year = &movie.release_date;
        let poster_path = match &movie.poster_path {
            Some(path) => tmdb::poster(path),
            None => String::new(),
        };
        println!(
            "{}. {} ({}) {} ({})",
            i + 1,
            movie.title,
            year,
            poster_path,
            movie.original_title
        );
        movies.push((movie.id, movie.title.to_string()));
    }
    let choice = input("\nQuel film correspond ? ")?;
    if choice == "skip" {
        Ok(None)
    } else {
        Ok(Some(movies[choice.parse::<usize>()? - 1].clone()))
    }
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
    let results = tmdb::search::tv_shows(&query).await?;
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
async fn store(
    episode: shows::Episode,
    show_id: i32,
    show_name: &str,
    dry_run: bool,
) -> Result<()> {
    let info = tmdb::tv_episodes::get_details(show_id, episode.season, episode.number).await?;
    let season = format!("{:02}", info.season_number);
    let number = format!("{:02}", info.episode_number);
    let name = if info.name.is_empty() {
        String::new()
    } else {
        format!(" - {}", info.name)
    };

    let mut dest = dirs::video_dir().unwrap();
    dest.push("Series");
    dest.push(correct_file_name(show_name));
    dest.push(format!("Season {season}"));
    create_dir_all(&dest)?;
    dest.push(correct_file_name(&format!(
        "{} - s{}e{}{}.{}",
        show_name,
        season,
        number,
        name,
        episode.src.extension().unwrap().to_str().unwrap()
    )));

    println!(
        "Copy {:?} to {:?}",
        episode.src.file_name().unwrap(),
        dest.file_name().unwrap()
    );
    if !dry_run {
        copy(episode.src, dest)?;
    }

    Ok(())
}

/// Correct file name to valid Windows name.
fn correct_file_name(name: &str) -> String {
    name.replace(['<', '>', ':', '"', '/', '\\', '|', '?', '*'], "_")
}

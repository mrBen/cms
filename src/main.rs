use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use walkdir::{Error, WalkDir};

lazy_static! {
    static ref NUMBERING: Regex = Regex::new(r"[Ss](\d+)[Ee](\d+)").unwrap();
}

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

fn main() -> Result<(), Error> {
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

    println!("{:#?}", episodes);

    Ok(())
}

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

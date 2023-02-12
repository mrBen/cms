use clap::Parser;
use std::path::PathBuf;
use walkdir::{Error, WalkDir};

/// Organize your series and movies.
#[derive(Parser)]
struct Cli {
    /// The folder to scan for videos
    folder: PathBuf,
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    let mut videos = list_videos(&args.folder);
    videos.sort();
    for video in videos {
        println!("{:?}", video.file_name().expect("no file name"));
    }

    Ok(())
}

fn list_videos(folder: &PathBuf) -> Vec<PathBuf> {
    let mut videos = Vec::new();

    for entry in WalkDir::new(folder).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            // TODO: use `mime_classifier`
            if entry.path().extension().expect("no extension") == "mp4"
                || entry.path().extension().expect("no extension") == "mkv"
            {
                videos.push(entry.into_path());
            }
        }
    }

    videos
}

mod movies;
pub mod shows;

use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, path::PathBuf};
use walkdir::WalkDir;

lazy_static! {
    static ref NUMBERING: Regex = Regex::new(r"[Ss](\d+)[Ee](\d+)").unwrap();
}

/// Recursively list all video files in a directory.
pub fn list_videos(folder: &PathBuf) -> Vec<PathBuf> {
    let mut videos = Vec::new();

    for entry in WalkDir::new(folder).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            // TODO: use `mime_classifier`
            if let Some(ext) = entry.path().extension() {
                if ext == "mp4" || ext == "mkv" {
                    videos.push(entry.into_path());
                }
            }
        }
    }

    videos
}

pub fn pre_sort(videos: &[PathBuf]) -> HashMap<String, Vec<shows::Episode>> {
    let mut episodes = HashMap::new();
    for video in videos {
        let file_name = video.file_name().unwrap().to_str().unwrap();
        if NUMBERING.is_match(file_name) {
            let show_name = NUMBERING
                .split(file_name)
                .collect::<Vec<&str>>()
                .first()
                .unwrap()
                .to_owned()
                .to_string();

            if !episodes.contains_key(&show_name) {
                episodes.insert(show_name.clone(), Vec::new());
            }

            let episode = shows::Episode::from(video.as_path());
            if let Some(show) = episodes.get_mut(&show_name) {
                show.push(episode);
            }
        }
    }
    episodes
}

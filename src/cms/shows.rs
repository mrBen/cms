use super::NUMBERING;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Episode {
    pub src: PathBuf,
    pub series: String,
    pub season: i32,
    pub number: i32,
    pub title: Option<String>,
}

impl From<&Path> for Episode {
    fn from(path: &Path) -> Self {
        let mut season = 0;
        let mut number = 0;
        if let Some(captures) = NUMBERING.captures(path.file_name().unwrap().to_str().unwrap()) {
            season = captures.get(1).unwrap().as_str().parse().unwrap();
            number = captures.get(2).unwrap().as_str().parse().unwrap();
        }

        Self {
            src: PathBuf::from(path),
            series: String::new(),
            season,
            number,
            title: None,
        }
    }
}

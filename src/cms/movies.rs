use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Film {
    pub src: PathBuf,
    pub title: String,
    pub year: i32,
}

impl From<&Path> for Film {
    fn from(path: &Path) -> Self {
        Self {
            src: PathBuf::from(path),
            title: String::new(),
            year: 0,
        }
    }
}

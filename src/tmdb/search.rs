use crate::tmdb::get;
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Movies {
    pub results: Vec<Movie>,
}

#[derive(Deserialize)]
pub struct Movie {
    pub poster_path: String,
    pub release_date: String,
    pub id: i32,
    pub original_title: String,
    pub title: String,
}

pub async fn movies(query: &str) -> Result<Movies, Error> {
    get("/search/movie", vec![("query", query)])
        .await?
        .json()
        .await
}

#[derive(Deserialize)]
pub struct Shows {
    pub results: Vec<Show>,
}

#[derive(Debug, Deserialize)]
pub struct Show {
    pub poster_path: String,
    pub id: i32,
    pub first_air_date: String,
    pub original_name: String,
}

pub async fn tv_shows(query: &str) -> Result<Shows, Error> {
    get("/search/tv", vec![("query", query)])
        .await?
        .json()
        .await
}

use crate::tmdb::{BASE_URL, TMDB_API_KEY};
use reqwest::Error;
use serde::Deserialize;

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

pub async fn search_tv_shows(query: String) -> Result<Shows, Error> {
    let endpoint = "/search/tv";
    reqwest::get(format!(
        "{BASE_URL}{endpoint}?api_key={TMDB_API_KEY}&query={query}"
    ))
    .await?
    .json()
    .await
}

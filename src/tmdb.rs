use reqwest::{Error, Response};
use serde::Deserialize;

const BASE_URL: &str = "https://api.themoviedb.org/3";
const TMDB_API_KEY: &str = env!("TMDB_API_KEY");
const IMAGE_BASE_URL: &str = "https://image.tmdb.org/t/p/";

async fn request(endpoint: String) -> Response {
    reqwest::get(format!("{BASE_URL}{endpoint}?api_key={TMDB_API_KEY}"))
        .await
        .unwrap()
}

#[derive(Debug, Deserialize)]
pub struct TvResult {
    pub poster_path: String,
    pub id: i32,
    pub first_air_date: String,
    pub original_name: String,
}

pub async fn search_tv(query: String) -> Result<Vec<TvResult>, Error> {
    let endpoint = "/search/tv";
    reqwest::get(format!(
        "{BASE_URL}{endpoint}?api_key={TMDB_API_KEY}&query={query}"
    ))
    .await?
    .json()
    .await
}

#[derive(Debug, Deserialize)]
pub struct Episode {
    pub episode_number: i32,
    pub name: String,
    pub season_number: i32,
}

pub async fn get_episode(
    tv_id: i32,
    season_number: i32,
    episode_number: i32,
) -> Result<Episode, Error> {
    request(format!(
        "/tv/{tv_id}/season/{season_number}/episode/{episode_number}"
    ))
    .await
    .json()
    .await
}

pub fn poster(poster_path: &str) -> String {
    let poster_size = "original";
    format!("{IMAGE_BASE_URL}{poster_size}{poster_path}")
}

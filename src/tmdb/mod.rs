use reqwest::{Error, Response};

pub mod search;
pub mod tv_episodes;

const BASE_URL: &str = "https://api.themoviedb.org/3";
const TMDB_API_KEY: &str = env!("TMDB_API_KEY");
const IMAGE_BASE_URL: &str = "https://image.tmdb.org/t/p/";

async fn get(endpoint: String) -> Result<Response, Error> {
    reqwest::get(format!("{BASE_URL}{endpoint}?api_key={TMDB_API_KEY}")).await
}

pub fn poster(poster_path: &str) -> String {
    let poster_size = "original";
    format!("{IMAGE_BASE_URL}{poster_size}{poster_path}")
}

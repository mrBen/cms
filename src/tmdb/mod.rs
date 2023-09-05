use lazy_static::lazy_static;
use reqwest::{Client, Error, Response};

pub mod search;
pub mod tv_episodes;

lazy_static! {
    static ref CLIENT: Client = reqwest::Client::new();
}

async fn get(endpoint: &str, params: &[(&str, &str)]) -> Result<Response, Error> {
    let token = env!("TMDB_BEARER_TOKEN");
    CLIENT
        .get(format!("https://api.themoviedb.org/3{endpoint}"))
        .header("Authorization", format!("Bearer {token}"))
        .header("Content-Type", "application/json; charset=UTF-8")
        .query(&params)
        .send()
        .await
}

pub fn poster(poster_path: &str) -> String {
    let poster_size = "original";
    format!("https://image.tmdb.org/t/p/{poster_size}{poster_path}")
}

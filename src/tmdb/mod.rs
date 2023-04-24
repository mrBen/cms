use lazy_static::lazy_static;
use reqwest::{Client, Error, Response};

pub mod search;
pub mod tv_episodes;

lazy_static! {
    static ref CLIENT: Client = reqwest::Client::new();
}

async fn get(endpoint: &str, mut params: Vec<(&str, &str)>) -> Result<Response, Error> {
    params.push(("api_key", env!("TMDB_API_KEY")));
    CLIENT
        .get(format!("https://api.themoviedb.org/3{endpoint}"))
        .header("Content-Type", "application/json; charset=UTF-8")
        .query(&params)
        .send()
        .await
}

pub fn poster(poster_path: &str) -> String {
    let poster_size = "original";
    format!("https://image.tmdb.org/t/p/{poster_size}{poster_path}")
}

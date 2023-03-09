use reqwest::Response;
use serde::Deserialize;

use crate::TMDB_API_KEY;

async fn request(endpoint: String) -> Response {
    reqwest::get(format!(
        "https://api.themoviedb.org/3{endpoint}?api_key={TMDB_API_KEY}"
    ))
    .await
    .expect("reqwest failed")
}

pub async fn get_episode(
    tv_id: u32,
    season_number: i32,
    episode_number: i32,
) -> Result<Episode, tmdb_async::Error> {
    request(format!(
        "/tv/{tv_id}/season/{season_number}/episode/{episode_number}"
    ))
    .await
    .json()
    .await
}

#[derive(Deserialize, Debug)]
pub struct Episode {
    crew: Vec<Crew>,
    episode_number: i32,
    name: String,
    id: i32,
    season_number: i32,
    still_path: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Crew {
    // id: i32,
    name: String,
    job: String,
}

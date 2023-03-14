use crate::tmdb::get;
use reqwest::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Episode {
    pub episode_number: i32,
    pub name: String,
    pub season_number: i32,
}

pub async fn get_details(
    tv_id: i32,
    season_number: i32,
    episode_number: i32,
) -> Result<Episode, Error> {
    get(
        &format!("/tv/{tv_id}/season/{season_number}/episode/{episode_number}"),
        vec![],
    )
    .await?
    .json()
    .await
}

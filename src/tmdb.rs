use crate::TMDB_API_KEY;

async fn request(endpoint: String) -> String {
    reqwest::get(format!(
        "https://api.themoviedb.org/3{endpoint}?api_key={TMDB_API_KEY}"
    ))
    .await
    .expect("reqwest error")
    .text()
    .await
    .expect("to text error")
}

pub async fn get_episode(tv_id: u32, season_number: i32, episode_number: i32) -> String {
    request(format!(
        "/tv/{tv_id}/season/{season_number}/episode/{episode_number}"
    ))
    .await
}

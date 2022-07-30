use bevy::prelude::*;
use serde::Deserialize;
use serde_json::Number;
// use ureq::serde_json::Number;

#[derive(Debug, Deserialize)]
pub struct HackerNewsStory {
    pub id: Number,
    pub title: String,
    pub url: String,
    pub points: Option<Number>,
    pub r#type: String,
    pub domain: Option<String>,
    pub time: Number,
    pub time_ago: String,
    pub comments_count: Number,
    pub user: Option<String>,
}

fn get(path: &str) -> anyhow::Result<ureq::Response> {
    info!("GET: {path}");
    Ok(ureq::get(path).set("User-Agent", "chrome").call()?)
}

pub fn get_stories(r#type: &str) -> anyhow::Result<Vec<HackerNewsStory>> {
    let response = get(&format!(
        "https://node-hnapi.herokuapp.com/{}?page={}",
        r#type, 0
    ))?;
    Ok(response.into_json()?)
}

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

#[derive(Debug, Deserialize)]
pub struct HackerNewsStoryWithComments {
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
    pub comments: Vec<HackerNewsComment>,
}

#[derive(Debug, Deserialize)]
pub struct HackerNewsComment {
    pub user: Option<String>,
    pub time_ago: String,
    pub content: Option<String>,
    pub comments: Vec<HackerNewsComment>,
}

#[derive(Debug, Deserialize)]
pub struct HackerNewsUser {
    pub created: Number,
    pub id: String,
    pub karma: Number,
}

const BASE_URL: &str = "https://api.hackerwebapp.com";

fn get(path: &str) -> anyhow::Result<ureq::Response> {
    info!("GET: {path}");
    Ok(ureq::get(path).set("User-Agent", "chrome").call()?)
}

pub fn get_stories(r#type: &str) -> anyhow::Result<Vec<HackerNewsStory>> {
    let response = get(&format!("{BASE_URL}/{}?page={}", r#type, 0))?;
    Ok(response.into_json()?)
}

pub fn get_story_comments(id: &str) -> anyhow::Result<HackerNewsStoryWithComments> {
    let response = get(&format!("{BASE_URL}/item/{id}"))?;
    Ok(response.into_json()?)
}

#[allow(unused)]
pub fn get_user(id: &str) -> anyhow::Result<HackerNewsUser> {
    let response = get(&format!(
        "https://hacker-news.firebaseio.com/v0/user/${id}.json"
    ))?;
    Ok(response.into_json()?)
}

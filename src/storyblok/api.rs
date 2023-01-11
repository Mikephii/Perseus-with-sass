use super::errors::MyCustomError;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Story<T> {
    pub name: String,
    pub slug: String,
    pub content: T,
}

#[derive(Serialize, Deserialize)]
pub struct SblokStoriesResponse<T> {
    pub cv: u32,
    pub stories: Vec<Story<T>>,
}

pub async fn get_content<T>() -> Result<Vec<T>, MyCustomError>
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    let url = "https://api.storyblok.com/v2/cdn/stories?token=T1lVJToB5V7fQxD0f4nRPQtt&version=draft&starts_with=portfolio-items";
    let response: SblokStoriesResponse<T> = reqwest::get(url).await?.json().await?;

    let content_items: Vec<T> = response
        .stories
        .into_iter()
        .map(|story| story.content)
        .collect();

    Ok(content_items)
}

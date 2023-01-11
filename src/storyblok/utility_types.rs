use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SbImage {
    pub id: u32,
    pub alt: String,
    pub name: String,
    pub focus: String,
    pub title: String,
    pub filename: String,
    pub copyright: String,
    pub fieldtype: String,
    pub is_external_url: bool,
}

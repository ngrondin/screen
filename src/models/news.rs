use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RssData {
    pub channels: Vec<ChannelData>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChannelData {
    pub title: String,
    pub image: ImageData,
    pub items: Vec<ArticleData>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ImageData {
    pub url: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ArticleData {
    pub title: String,
    pub description: String,
    pub image: ImageData
}
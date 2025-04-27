
use std::io::Read;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::data::DataStore;

use super::Provider;


#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct RssData {
    channel: Vec<ChannelData>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct ChannelData {
    title: String,
    image: ImageData,
    item: Vec<ArticleData>
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
struct ImageData {
    url: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct ArticleData {
    title: String,
    #[serde(default)]
    description: String,
    #[serde(default, rename(deserialize = "thumbnail"))]
    image: ImageData
}

pub struct RSSProvider {
    name: String,
    url: String,
}

impl RSSProvider {
    pub fn new(name: &str, value: &Value) -> Self {
        let url = match value["url"].as_str() {
            Some(url_str) => url_str,
            None => "https://feeds.bbci.co.uk/news/world/rss.xml"
        };
        RSSProvider { name: name.to_string(), url: url.to_string() }
    }
}

impl Provider for RSSProvider {
    fn provide(&mut self, data_store: &mut DataStore) {
        let mut res = reqwest::blocking::get(&self.url).unwrap();
        let mut body_str = String::new();
        res.read_to_string(&mut body_str).unwrap();
        let rss: RssData = serde_xml_rs::from_str(&body_str).unwrap();
        let json_str = serde_json::to_string_pretty(&rss).unwrap();
        data_store.store(&self.name, &json_str); 
    }
}
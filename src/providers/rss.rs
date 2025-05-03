
use std::io::Read;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{data::DataStore, models::news::{ArticleData, ChannelData, ImageData, RssData}};

use super::Provider;

 
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct XmlRssData {
    channel: Vec<XmlChannelData>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct XmlChannelData {
    title: String,
    image: XmlImageData,
    item: Vec<XmlArticleData>
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
struct XmlImageData {
    url: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct XmlArticleData {
    title: String,
    #[serde(default)]
    description: String,
    #[serde(rename(deserialize = "thumbnail"))]
    image1: Option<XmlImageData>,
    #[serde(rename(deserialize = "enclosure"))]
    image2: Option<XmlImageData>,
    #[serde(rename(deserialize = "group"))]
    image3: Option<XmlMediaGroupData>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct XmlMediaGroupData {
    #[serde(rename(deserialize = "content"))]
    image: Vec<XmlImageData>
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
        if let Ok(mut res) =  reqwest::blocking::get(&self.url) {
            let mut body_str = String::new();
            res.read_to_string(&mut body_str).unwrap();
            let xml_rss: XmlRssData = serde_xml_rs::from_str(&body_str).unwrap();
            let rss = RssData {
                channels: xml_rss.channel.iter().map(|xml_chan| {
                    ChannelData {
                        title: xml_chan.title.clone(),
                        image: ImageData { 
                            url: xml_chan.image.url.clone() 
                        },
                        items: xml_chan.item.iter().map(|item| {
                            ArticleData { 
                                title: item.title.clone(), 
                                description: item.description.clone(),
                                image: ImageData { 
                                    url: if let Some(img) = &item.image1 {
                                        img.url.clone()
                                    } else if let Some(img) = &item.image2 {
                                        img.url.clone()
                                    } else if let Some(img) = &item.image3 {
                                        img.image[0].url.clone()
                                    } else {
                                        "".to_owned()
                                    }
                                } 
                            }
                        }).collect()
                    }
                }).collect()
            };

            let json_str = serde_json::to_string_pretty(&rss).unwrap();
            data_store.store(&self.name, &json_str); 
        }        
    }
}

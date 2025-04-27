use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct WeatherData {
    pub title: String,
    pub description: String,
    pub icon: String,
    pub temp: f64,
    pub pressure: u64, 
    pub humidity: u64,
    pub wind_speed: f64,
    pub wind_dir: u64,
    pub cloud: u64,
    pub sunrise: u64,
    pub sunset: u64
} 

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct WeatherForecastData {
    pub list: Vec<WeatherForecastItemData>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct WeatherForecastItemData {
    pub ts: i64,
    pub title: String,
    pub icon: String,
    pub temp: f64,
    pub wind_speed: f64,
    pub wind_dir: u64,
}
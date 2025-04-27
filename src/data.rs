use std::{fs::{self, File}, io::BufReader};

use serde::de::DeserializeOwned;

pub struct DataStore {
    folder: String
}

impl DataStore {
    pub fn new(f: &str) -> Self {
        DataStore { folder: f.to_string() }
    }
    
    pub fn store(&self, name: &str, json_str: &str) {
        let path = format!("{}/{}.json", self.folder, name);
        fs::write(path, json_str).unwrap();
    }

    pub fn load<T: DeserializeOwned>(&self, name: &str) -> T {
        let path = format!("{}/{}.json", self.folder, name);
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let data: T = serde_json::from_reader(reader).unwrap();
        data
    }
}
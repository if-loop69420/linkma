use std::collections::HashMap;

use config::{Config, File};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CreateConfig {
    pub files: HashMap<String, ConfigFile>,
}

#[derive(Deserialize, Clone)]
pub struct ConfigFile {
    pub contents: Contents,
}

#[derive(Deserialize, Clone)]
pub enum Contents {
    internal(String),
    external(String),
}

impl From<String> for CreateConfig {
    fn from(value: String) -> Self {
        let s = Config::builder()
            .add_source(File::with_name(&value))
            .build()
            .unwrap();

        s.try_deserialize().unwrap()
    }
}

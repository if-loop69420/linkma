use config::{Config, File};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CreateConfig {
    pub files: Vec<ConfigFile>,
}

#[derive(Deserialize, Clone)]
pub struct ConfigFile {
    pub output_path: String,
    pub contents: Contents,
}

#[derive(Deserialize, Clone)]
pub enum Contents {
    InFileContents(String),
    OutOfFileContents(String),
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

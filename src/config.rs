use config::{Config, File};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateConfig {
    files: Vec<ConfigFile>,
}

#[derive(Deserialize)]
pub struct ConfigFile {
    output_path: String,
    contents: Contents,
}

#[derive(Deserialize)]
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

use std::{
    ascii::AsciiExt,
    cell::LazyCell,
    collections::{HashMap, HashSet},
    fs::{File, create_dir_all, exists},
    io::{Read, Write},
    time::SystemTime,
};

use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256, Sha512};

use crate::config::{ConfigFile, CreateConfig};

pub const TIMESTAMP: LazyCell<DateTime<Utc>> = LazyCell::new(|| SystemTime::now().into());

pub fn create_directory_if_not_exists() -> Result<(), String> {
    let timestamp_string = format!("{}", TIMESTAMP.clone().format("%d_%m_%Y-%H_%M"));
    let path = format!("/linkma/{}", timestamp_string);
    let exists = match exists(path.clone()) {
        Ok(x) => x,
        Err(e) => {
            return Err(format!(
                "Failed to check if the directory /linkma exists with error:\n{}",
                e
            ));
        }
    };
    if !exists {
        match create_dir_all(path.clone()) {
            Ok(()) => (),
            Err(e) => {
                return Err(format!(
                    "Failed to create directory /linkma with error\n{}",
                    e
                ));
            }
        }
    }

    Ok(())
}

pub fn check_unique_filenames(config: CreateConfig) -> bool {
    let mut unique = HashSet::new();
    config
        .files
        .iter()
        .all(|x| unique.insert(x.output_path.clone()))
}

pub fn create_dir(config: CreateConfig) -> Result<(), String> {
    create_directory_if_not_exists()?;
    if !check_unique_filenames(config.clone()) {
        return Err(String::from(
            "The supplied config contains duplicate output paths",
        ));
    }

    Ok(())
}

fn get_file_contents(file: ConfigFile) -> Result<String, String> {
    match file.contents {
        crate::config::Contents::InFileContents(x) => Ok(x),
        crate::config::Contents::OutOfFileContents(x) => {
            let mut file = match File::open(x) {
                Ok(x) => x,
                Err(e) => {
                    return Err(format!("{}", e));
                }
            };
            let mut return_string = String::new();
            match File::read_to_string(&mut file, &mut return_string) {
                Ok(_size) => Ok(return_string),
                Err(e) => {
                    return Err(format!("{}", e));
                }
            }
        }
    }
}

pub fn create_files(config: CreateConfig) -> Result<(), String> {
    create_dir(config.clone())?;

    for file in config.files {
        let mut hasher = Sha256::new();
        hasher.update(file.output_path.clone());
        let output_name = format!("{:X}", hasher.finalize());
        let timestamp_string = format!("{}", TIMESTAMP.clone().format("%d_%m_%Y-%H_%M"));
        let path = format!("/linkma/{}/{}.balls", timestamp_string, output_name);
        let contents = get_file_contents(file)?;
        let mut out_file = match File::create(path) {
            Ok(x) => x,
            Err(e) => {
                return Err(format!("Failed to create file with: {}", e));
            }
        };
        match out_file.write_all(contents.as_bytes()) {
            Ok(_x) => {}
            Err(e) => {
                return Err(format!("{}", e));
            }
        }
    }

    Ok(())
}

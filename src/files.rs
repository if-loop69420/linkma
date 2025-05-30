use std::{
    cell::LazyCell,
    collections::{HashMap, HashSet},
    fs::{
        File, create_dir_all, exists, read_dir, read_link, remove_dir_all, remove_file,
        set_permissions,
    },
    io::{Read, Write},
    os::unix::fs::symlink,
    time::SystemTime,
};

use chrono::{DateTime, Utc};
use regex::Regex;
use sha2::{Digest, Sha256};

use crate::config::{ConfigFile, CreateConfig};

pub const TIMESTAMP: LazyCell<DateTime<Utc>> = LazyCell::new(|| SystemTime::now().into());

pub const DIRECTORY_REGEX: LazyCell<Regex> = LazyCell::new(|| {
    Regex::new(r"\d+_\d+_\d+-\d+_\d+").expect("Couldn't create regex for dir name")
});

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
    config.files.iter().all(|x| unique.insert(x.0.clone()))
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
        crate::config::Contents::internal(x) => Ok(String::from(x.trim())),
        crate::config::Contents::external(x) => {
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

fn link_files(mappings: HashMap<String, (String, String)>) -> Result<(), String> {
    // Go through all mappings, check if a (non-symlink) file for the output path exists
    // If it exists move it to {path}.bak. If it doesn't exists create everything needed to put
    // the link where it needs to go (directories, if they don't exist). Link the files to their
    // output_path.

    for (_output_name, (output_path, path)) in mappings.clone() {
        // If the file exists move it to output_path.bak
        if exists(output_path.clone())
            .expect(format!("Couldn't check that file exists!\n{}", output_path).as_str())
        {
            // Read current contents and write it to output_path.bak
            let mut old_file = File::open(output_path.clone())
                .expect(format!("Couldn't open file for reading!\n{}", output_path).as_str());

            if !read_link(output_path.clone()).is_ok() {
                println!(
                    "{} is not a symlink. It is a {:?}",
                    output_path.clone(),
                    old_file.metadata().expect("Couldn't open").file_type()
                );
                let mut new_file = File::create(format!("{}.bak", output_path)).expect(
                    format!(
                        "Couldn't open file for backing up a file!\n{}.bak",
                        output_path
                    )
                    .as_str(),
                );
                let mut old_file_contents = String::new();
                File::read_to_string(&mut old_file, &mut old_file_contents).expect(
                    format!(
                        "Couldn't read contents from old file{}",
                        output_path.clone(),
                    )
                    .as_str(),
                );

                new_file
                    .write_all(old_file_contents.as_bytes())
                    .expect("Couldn't write backup file");
            }
            remove_file(output_path.clone())
                .expect("Couldn't remove file which is to be replaced with a symlink");
        }
        // Now we can link the file
        symlink(path, output_path).expect("Couldn't create symlink");
    }

    let mappings_as_json =
        serde_json::to_string_pretty(&mappings).expect("Couldn't turn mapping to json.");

    let mut mappings_file = File::create(format!(
        "/linkma/{}/mappings.json",
        TIMESTAMP.clone().format("%d_%m_%Y-%H_%M")
    ))
    .expect("Couldn't open mappings file");
    mappings_file
        .write_all(mappings_as_json.as_bytes())
        .expect("Couldn't write json to mappings file");

    Ok(())
}

pub fn create_files(config: CreateConfig) -> Result<(), String> {
    create_dir(config.clone())?;

    let mut mappings = HashMap::new();

    for file in config.files {
        let mut hasher = Sha256::new();
        hasher.update(file.0.clone());
        let output_name = format!("{:X}", hasher.finalize());
        let timestamp_string = format!("{}", TIMESTAMP.clone().format("%d_%m_%Y-%H_%M"));
        let path = format!("/linkma/{}/{}.balls", timestamp_string, output_name);
        mappings.insert(output_name.clone(), (file.0.clone(), path.clone()));
        let contents = get_file_contents(file.1)?;
        let mut out_file = match File::create(path.clone()) {
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
        let mut perms = out_file
            .metadata()
            .expect("Couldn't get permissions for file")
            .permissions();

        perms.set_readonly(true);
        set_permissions(path, perms).expect("Couldn't set permissions to RO for files");
    }

    // Link the created files where they need to go
    // Backup the mapping between the path in /linkma and the output_path, so we can rollback.
    link_files(mappings)?;

    let directory_path = format!("/linkma/{}", TIMESTAMP.clone().format("%d_%m_%Y-%H_%M"));

    let directory = File::open(directory_path.clone()).expect("Couldn't open directory");
    let mut perms = directory
        .metadata()
        .expect("Couldn't open metadata for directory")
        .permissions();
    perms.set_readonly(true);

    set_permissions(directory_path.clone(), perms)
        .expect("Couldn't set permissions to RO for files");

    let _ = remove_file("/linkma/current_system");
    symlink(directory_path, "/linkma/current_system").expect("Couldn't link new current_system");

    Ok(())
}

pub fn list_generations() {
    let directory_path = String::from("/linkma/");
    let directories = read_dir(directory_path).expect("Couldn't read contents of /linkma");
    let generations: Vec<String> = directories
        .map(|x| {
            x.expect("Couldn't unwrap content")
                .file_name()
                .into_string()
                .expect("Couldn't convert to string")
        })
        .filter(|x| DIRECTORY_REGEX.is_match(x))
        .collect();

    println!("A list of the generations (newest first)");

    generations
        .iter()
        .rev()
        .enumerate()
        .for_each(|(i, x)| println!("[{}] {}", i, x));
}

pub fn delete_nth_generation(n: usize) {
    let directory_path = String::from("/linkma/");
    let directories = read_dir(directory_path).expect("Couldn't read contents of /linkma");
    let generations: Vec<String> = directories
        .map(|x| {
            x.expect("Couldn't unwrap content")
                .file_name()
                .into_string()
                .expect("Couldn't convert to string")
        })
        .filter(|x| DIRECTORY_REGEX.is_match(x))
        .collect();

    println!("A list of the generations (newest first)");

    generations
        .iter()
        .rev()
        .enumerate()
        .filter(|(i, _)| *i == n)
        .map(|(_, x)| x)
        .for_each(|x| {
            remove_dir_all(format!("/linkma/{}", x)).expect("Couldn't remove the file");
        });
}

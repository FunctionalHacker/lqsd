extern crate xdg;

use super::fs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use xdg::BaseDirectories;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub resume_file: PathBuf,
    pub idle_level: i32,
    pub dim_speed: u64,
    pub resume_speed: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            resume_file: PathBuf::from("/tmp/lqsd-resume"),
            idle_level: 0,
            dim_speed: 50,
            resume_speed: 25,
        }
    }
}

fn read_parse(path: PathBuf) -> Config {
    let mut toml = String::from("");

    match fs::read(&path) {
        Ok(result) => toml = result,
        Err(err) => {
            eprintln!("Failed to read config: {}", err);
            println!("Using default config");
            return Config::default();
        }
    }

    match toml::from_str(&toml) {
        Ok(result) => return result,
        Err(err) => {
            eprintln!("Failed to read config: {}", err);
            println!("Using default config");
            return Config::default();
        }
    }
}

fn xdg_path() -> PathBuf {
    BaseDirectories::with_prefix("lqsd")
        .expect("cannot create configuration directory")
        .place_config_file("config.toml")
        .unwrap()
}

pub fn copy_config() {
    let path = xdg_path();
    match fs::write(&path, toml::to_string(&Config::default()).unwrap()) {
        Ok(()) => println!("Default config saved to {}", path.display()),
        Err(err) => eprintln!("Failed to write default config: {}", err),
    };
}

pub fn load_xdg() -> Config {
    let path = xdg_path();

    if !path.exists() {
        Config::default()
    } else {
        read_parse(xdg_path())
    }
}

pub fn load_user(path: PathBuf) -> Config {
    read_parse(path)
}

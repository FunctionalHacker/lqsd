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

fn xdg_config() -> PathBuf {
    BaseDirectories::with_prefix("lqsd")
        .expect("cannot create configuration directory")
        .place_config_file("config.toml")
        .unwrap()
}

pub fn load_xdg() -> Config {
    let path = xdg_config();

    if !path.exists() {
        println!(
            "{} does not exist, writing default configuration",
            path.display()
        );

        match fs::write(&path, toml::to_string(&Config::default()).unwrap()) {
            Ok(()) => println!("Default config saved to {}", path.display()),
            Err(err) => eprintln!("Failed to write default config: {}", err),
        };

        Config::default()
    } else {
        let toml: String;

        match fs::read(&path) {
            Ok(string) => toml = string,
            Err(err) => {
                eprintln!("Failed to read config: {}", err);
                eprintln!("Using default config");
                return Config::default();
            }
        }

        match toml::from_str<Config, _>(&toml) {
            Ok(parsed_conf) = parsed_conf,
            Err(err) => {
                eprintln!("Failed to parse config: {}", err);
                eprintln!("Using default config");
                config
            }
        }
    }
}

pub fn load_user(path: PathBuf) -> Config {
    if !path.exists() {
        panic!("{} does not exist", path.display());
    } else {
        let toml = fs::read(&path).unwrap();
        match toml::from_str(&toml) {
            Ok(parsed_conf) ->
        }
        config
    }
}

use super::fs;
use super::Config;
use std::path::PathBuf;
use xdg::BaseDirectories;

extern crate xdg;

fn default_config() -> Config {
    Config {
        resume_file: PathBuf::from("/tmp/lqsd-resume"),
        idle_level: 0,
        dim_speed: 50,
        resume_speed: 25,
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
        match fs::write(&path, toml::to_string(&default_config()).unwrap()) {
            Ok(()) => println!("Default config saved to {}", path.display()),
            Err(err) => eprintln!("Failed to write default config: {}", err),
        };
        default_config()
    } else {
        let toml = fs::read(&path).unwrap();
        let config: Config = toml::from_str(&toml).unwrap();
        config
    }
}

pub fn load_user(path: PathBuf) -> Config {
    if !path.exists() {
        panic!("{} does not exist", path.display());
    } else {
        let toml = fs::read(&path).unwrap();
        let config: Config = toml::from_str(&toml).unwrap();
        config
    }
}

extern crate clap;

use config::Config;
use std::path::PathBuf;
use std::process::Command;
use std::{thread, time};

mod cli;
mod config;
mod fs;

fn transition(w_brightness: i32, speed: time::Duration) {
    let c_brightness = get_brightness();
    println!("Transitioning from {}% to {}%", c_brightness, w_brightness);

    if c_brightness < w_brightness {
        for n in c_brightness..w_brightness {
            if n != c_brightness {
                set_brightness(n);
                thread::sleep(speed);
            }
        }
    } else {
        for n in (w_brightness..c_brightness).rev() {
            if n != c_brightness {
                set_brightness(n);
                thread::sleep(speed);
            }
        }
    }
}

fn set_brightness(brightness: i32) {
    Command::new("light")
        .args(&["-S", &brightness.to_string()])
        .spawn()
        .expect("Failed to execute command 'light'");
}

fn get_brightness() -> i32 {
    let output = Command::new("light")
        .arg("-G")
        .output()
        .expect("Failed to execute command 'light'");

    let string = String::from_utf8_lossy(&output.stdout);
    let float: f32 = string.trim().parse().unwrap();
    float.round() as i32
}

fn main() {
    let args = cli::app().get_matches();
    let conf: Config;

    if let Some(config_path) = args.get_one::<String>("config") {
        let config_path_buf = PathBuf::from(config_path);
        conf = config::load_user(config_path_buf);
    } else {
        conf = config::load_xdg();
    }

    let dim_speed = time::Duration::from_millis(conf.dim_speed);
    let resume_speed = time::Duration::from_millis(conf.resume_speed);

    if args.get_flag("dim") {
        let current_brightness = get_brightness().to_string();
        match fs::write(&conf.resume_file, current_brightness) {
            Ok(()) => println!(
                "Current brightness written to resume file at {}",
                conf.resume_file.display()
            ),
            Err(err) => eprintln!("Error writing brightness to resume file: {}", err),
        }
        transition(conf.idle_level, dim_speed);
    }

    if args.get_flag("resume") {
        let old_brightness: i32;

        match fs::read(&conf.resume_file) {
            Ok(result) => old_brightness = result.parse().unwrap(),
            Err(err) => {
                eprintln!("Failed to read resume_file: {}", err);
                old_brightness = 100;
            }
        }

        transition(old_brightness, resume_speed);

        match fs::remove(&conf.resume_file) {
            Ok(()) => println!("Resume file removed"),
            Err(err) => eprintln!("Failed to remove resume file: {}", err),
        }
    }

    if args.get_flag("copy-config") {
        config::copy_config();
    }
}

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
    let args = cli::get_args();
    let conf: Config;

    if args.is_present("config") {
        let config_path = PathBuf::from(args.value_of("config").unwrap());
        conf = config::load_user(config_path);
    } else {
        conf = config::load_xdg();
    }
    let dim_speed = time::Duration::from_millis(conf.dim_speed);
    let resume_speed = time::Duration::from_millis(conf.resume_speed);

    if args.is_present("dim") {
        let current_brightness = get_brightness().to_string();
        match fs::write(&conf.resume_file, current_brightness) {
            Ok(()) => println!("Current brightness written to resume file"),
            Err(err) => eprintln!("Error writing brightness to resume file: {}", err),
        }
        transition(conf.idle_level, dim_speed);
    }
    if args.is_present("resume") {
        let old_brightness: i32 = fs::read(&conf.resume_file).unwrap().parse().unwrap();
        transition(old_brightness, resume_speed);
        match fs::remove(&conf.resume_file) {
            Ok(()) => println!("Resume file removed"),
            Err(err) => eprintln!("Failed to remove resume file: {}", err),
        }
    }
}

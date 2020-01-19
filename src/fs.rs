use std::fs::{remove_file, File};
use std::io::prelude::*;
use std::path::PathBuf;

pub fn write(path: &PathBuf, contents: String) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;
    file.sync_data()?;
    Ok(())
}

pub fn read(path: &PathBuf) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn remove(path: &PathBuf) -> std::io::Result<()> {
    remove_file(path)?;
    Ok(())
}

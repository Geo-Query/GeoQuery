use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};

pub fn read_path(configFile: File) -> Result<PathBuf ,Box<dyn Error>> {
    let mut path_str = String::new();


    BufReader::new(configFile).read_line(&mut path_str)?;

    if path_str.ends_with('\n') {
        path_str.pop();
        if path_str.ends_with('\r') {
            path_str.pop();
        }
    }
    return Ok(PathBuf::from(path_str));
}
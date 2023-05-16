use std::{fs, str::FromStr};

use anyhow::Result;

const DATA_DIR_PATH: &'static str = "data/";

pub fn read_line_per_line<T: FromStr>(file: &str) -> Result<Vec<T>> {
    Ok(fs::read_to_string(format!("{}{}", DATA_DIR_PATH, file))?
        .lines()
        .filter_map(|l| l.parse::<T>().ok())
        .collect())
}

pub fn read_vec_by_sep<T: FromStr>(file: &str, sep: &str) -> Result<Vec<Vec<T>>> {
    Ok(fs::read_to_string(format!("{}{}", DATA_DIR_PATH, file))?
        .split(sep)
        .map(|s| s.lines().filter_map(|v| v.parse::<T>().ok()).collect())
        .collect())
}

pub fn read_parts(file: &str) -> Result<Vec<u8>> {
    Ok(fs::read(format!("{}{}", DATA_DIR_PATH, file))?)
}

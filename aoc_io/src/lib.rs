use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use anyhow::Result;

pub fn lines(file_name: impl AsRef<Path>) -> Result<Vec<String>> {
    let file = File::open(file_name)?;
    let v = io::BufReader::new(file).lines().collect::<Result<_, _>>()?;
    Ok(v)
}
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines(path: impl AsRef<Path>) -> impl Iterator<Item = String> {
    let file = File::open(path).unwrap();
    io::BufReader::new(file).lines().map(|line| line.unwrap())
}

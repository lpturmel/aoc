use std::io;
use std::str::FromStr;

pub fn read_file_to_lines<T>(path: &str) -> io::Result<Vec<T>>
where
    T: FromStr,
{
    Ok(std::fs::read_to_string(path)?
        .lines()
        .filter_map(|line| line.parse::<T>().ok())
        .collect())
}

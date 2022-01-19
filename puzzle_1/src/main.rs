use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let input_file: &str = "./input.txt";
    let mut nb_measurements: i32 = 0;

    if let Ok(depths) = read_file(input_file) {
        println!("Number of depth measures: {}", depths.len());
        depths.iter().enumerate().for_each(|(i, current_depth)| {
            if i != 0 {
                if current_depth > &depths[i - 1] {
                    nb_measurements += 1;
                }
            }
        })
    }
    println!(
        "Number of larger than previous measurements: {}",
        nb_measurements
    );
    println!("Duration: {}ms", now.elapsed().as_millis());
}

fn read_file(filename: &str) -> io::Result<Vec<i32>> {
    let file = File::open(filename)?;
    let buf_reader = BufReader::new(file);
    let collection = buf_reader
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect();

    Ok(collection)
}

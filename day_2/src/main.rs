mod direction;
mod pos;

use direction::Direction;
use pos::Pos;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() {
    let input: &str = "./input.txt";

    let mut pos = Pos::new(0, 0, 0);

    if let Ok(instructions) = read_file_to_lines(input) {
        for instruction in instructions {
            let direction = convert_input_to_pos(&instruction);
            match direction {
                Direction::Up(x) => {
                    pos.set_aim(pos.aim - x);
                }
                Direction::Down(x) => {
                    pos.set_aim(pos.aim + x);
                }
                Direction::Forward(x) => {
                    pos.set_x(pos.x + x);
                    pos.set_depth(pos.depth + pos.aim * x);
                }
            }
        }
    }
    let result = pos.result();
    println!("Result of the multiplication: {}", result);
}
fn convert_input_to_pos(s: &String) -> Direction {
    let split: Vec<&str> = s.split(" ").collect();

    let direction = split[0];
    let unit = split[1];

    match direction {
        "up" => Direction::Up(unit.parse::<i32>().unwrap()),
        "down" => Direction::Down(unit.parse::<i32>().unwrap()),
        "forward" => Direction::Forward(unit.parse::<i32>().unwrap()),
        _ => panic!("Unknown direction"),
    }
}
fn read_file_to_lines(filename: &str) -> io::Result<Vec<String>> {
    match File::open(filename) {
        Ok(file) => {
            let buf_reader = BufReader::new(file);
            let collection = buf_reader.lines().map(|l| l.unwrap()).collect();
            Ok(collection)
        }
        Err(e) => panic!("Error reading file: {}", e),
    }
}

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() {
    let input: &str = "./input.txt";
    let mut oxygen_rating = String::from("");
    let mut co2_rating = String::from("");

    if let Ok(lines) = read_file_to_lines(input) {
        oxygen_rating = process_rating(&lines, '1');
        co2_rating = process_rating(&lines, '0');
    }
    println!("Binary O2 rating: {}", oxygen_rating);
    println!("Binary Co2 rating: {}", co2_rating);

    let decimal_oxygen_rating = binary_to_decimal(&oxygen_rating);
    let decimal_co2_rating = binary_to_decimal(&co2_rating);

    println!("Result: {}", decimal_oxygen_rating * decimal_co2_rating);
}
fn process_rating(input: &Vec<String>, bit: char) -> String {
    let mut map = input.clone();
    let mut column: usize = 0;
    const BIT_SIZE: usize = 12;

    while column < BIT_SIZE && map.len() > 1 {
        let mut ones = 0;
        let mut zeros = 0;

        for row in 0..map.len() {
            let current_char = map[row].chars().nth(column).unwrap();
            match current_char {
                '1' => ones += 1,
                '0' => zeros += 1,
                _ => panic!("Invalid character"),
            }
        }
        if ones > zeros {
            match bit {
                '0' => {
                    map = map
                        .into_iter()
                        .filter(|line| line.chars().nth(column).unwrap() == '0')
                        .collect();
                }
                '1' => {
                    map = map
                        .into_iter()
                        .filter(|line| line.chars().nth(column).unwrap() == '1')
                        .collect();
                }
                _ => {}
            }
        } else if ones < zeros {
            match bit {
                '0' => {
                    map = map
                        .into_iter()
                        .filter(|line| line.chars().nth(column).unwrap() == '1')
                        .collect();
                }
                '1' => {
                    map = map
                        .into_iter()
                        .filter(|line| line.chars().nth(column).unwrap() == '0')
                        .collect();
                }
                _ => {}
            }
        } else {
            // There is an equal amount of ones and zeros
            map = map
                .into_iter()
                .filter(|line| line.chars().nth(column).unwrap() == bit)
                .collect();
        }
        column += 1;
    }

    map[0].clone()
}
fn binary_to_decimal(s: &String) -> isize {
    let decimal_value = isize::from_str_radix(s, 2).unwrap();
    decimal_value
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
#[cfg(test)]
mod tests {
    #[test]
    fn binary_to_decimal() {
        let binary_value = "10110";
        let decimal_value = super::binary_to_decimal(&String::from(binary_value));
        assert_eq!(decimal_value, 22);
    }
}

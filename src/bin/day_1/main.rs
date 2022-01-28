use aoc::read_file_to_lines;

fn main() {
    let input_file: &str = "data/day1.txt";
    let mut nb_measurements: i32 = 0;

    if let Ok(depths) = read_file_to_lines::<i32>(input_file) {
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
}

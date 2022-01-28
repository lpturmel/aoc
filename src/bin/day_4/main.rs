mod bingo;
mod board;

use aoc::read_file_to_lines;
use bingo::Bingo;
use board::Board;

fn main() {
    part_one();
    part_two();
}
fn part_one() {
    let input_lines = read_file_to_lines::<String>("data/day4.txt").unwrap();
    let bingo_numbers = &input_lines[0];
    let bingo = Bingo::from(bingo_numbers);

    let mut boards_str: Vec<Vec<String>> = vec![vec![]];
    let mut board_index = 0;

    // Build boards as array of strings
    for line in input_lines.into_iter().skip(2) {
        if line != "" {
            if boards_str[board_index].len() < 5 {
                boards_str[board_index].push(line);
            } else {
                board_index += 1;
                boards_str.push(vec![]);
                boards_str[board_index].push(line);
            }
        }
    }

    let mut boards: Vec<Board> = vec![];

    for board_str in boards_str {
        boards.push(Board::from(board_str));
    }

    'bingo: for bingo_number in bingo.bingo_numbers {
        // Check for boards that have the bingo number
        for board in boards.iter_mut() {
            board.mark_number(bingo_number);

            let winner = board.has_won();

            if winner {
                let winning_score = board.calculate_score(bingo_number);
                println!("Part one result: {}", winning_score);
                break 'bingo;
            }
        }
    }
}
fn part_two() {
    let input_lines = read_file_to_lines("data/day4.txt").unwrap();
    let bingo_numbers = &input_lines[0];
    let bingo = Bingo::from(bingo_numbers);

    let mut boards_str: Vec<Vec<String>> = vec![vec![]];
    let mut board_index = 0;

    // Build boards as array of strings
    for line in input_lines.into_iter().skip(2) {
        if line != "" {
            if boards_str[board_index].len() < 5 {
                boards_str[board_index].push(line);
            } else {
                board_index += 1;
                boards_str.push(vec![]);
                boards_str[board_index].push(line);
            }
        }
    }

    let mut boards: Vec<Board> = vec![];

    for board_str in boards_str {
        boards.push(Board::from(board_str));
    }

    let mut last_winner_index = 0;
    let mut last_winner_drawn_number = 0;
    for bingo_number in bingo.bingo_numbers {
        // Check for boards that have the bingo number
        for (i, board) in boards.iter_mut().enumerate() {
            if !board.won {
                board.mark_number(bingo_number);

                let winner = board.has_won();
                if winner {
                    last_winner_index = i;
                    last_winner_drawn_number = bingo_number;
                }
            }
        }
    }

    println!(
        "Part two score: {}",
        boards[last_winner_index].calculate_score(last_winner_drawn_number)
    );
}
#[cfg(test)]
mod tests {

    #[test]
    fn test_bingo_from_list_numbers() {
        let bingo = super::Bingo::from(&"1,2,3,4,5".to_string());

        assert_eq!(bingo.bingo_numbers, vec![1, 2, 3, 4, 5]);
    }
}

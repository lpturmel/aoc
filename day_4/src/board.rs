use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Board {
    rows: Vec<Vec<i32>>,
    pub won: bool,
}

impl Board {
    pub fn mark_number(&mut self, number: i32) -> () {
        for row in self.rows.iter_mut() {
            for cell in row {
                if *cell == number {
                    *cell = -1;
                }
            }
        }
    }
    pub fn has_won(&mut self) -> bool {
        let mut won = false;
        // Do rows first
        for row in &self.rows {
            if row.iter().all(|&x| x == -1) {
                won = true;
            }
        }

        // Do columns
        for i in 0..self.rows[0].len() {
            let mut mark_count = 0;
            for row in &self.rows {
                if row[i] == -1 {
                    mark_count += 1;
                }
            }
            if mark_count == self.rows.len() {
                won = true;
            }
        }
        self.won = won;
        won
    }
    pub fn calculate_score(&self, bingo_number: i32) -> i32 {
        let mut score = 0;
        for row in &self.rows {
            for cell in row {
                if *cell != -1 {
                    score += *cell;
                }
            }
        }
        score * bingo_number
    }
}
impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for row in &self.rows {
            for cell in row {
                write!(f, "{} ", cell)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
impl From<Vec<String>> for Board {
    fn from(item: Vec<String>) -> Self {
        let mut rows = Vec::new();

        for line in item {
            let mut row = Vec::new();
            for c in line.split(" ").into_iter() {
                if c != "" {
                    row.push(c.parse::<i32>().unwrap());
                }
            }
            rows.push(row);
        }
        Board { rows, won: false }
    }
}

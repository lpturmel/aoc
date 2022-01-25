pub struct Bingo {
    pub bingo_numbers: Vec<i32>,
}

impl From<&String> for Bingo {
    fn from(item: &String) -> Self {
        let bingo_numbers = item
            .split(",")
            .map(|line| line.parse::<i32>().unwrap())
            .collect();

        Bingo { bingo_numbers }
    }
}

use std::fs;

const BOARD_SIZE: usize = 5;

#[derive(Debug, Clone, Copy)]
struct Board {
    values: [[usize; BOARD_SIZE]; BOARD_SIZE],
    marked: [[bool; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    fn build(input: &[&str]) -> Self {
        let mut values = [[0; BOARD_SIZE]; BOARD_SIZE];
        for (i, row) in input.iter().enumerate() {
            let cols: Vec<usize> = row
                .split_whitespace()
                .map(|number| number.parse::<usize>().unwrap())
                .collect();
            for (j, val) in cols.iter().enumerate() {
                values[i][j] = *val;
            }
        }
        Board {
            values: values,
            marked: [[false; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    fn play_number(&self, number: usize) -> Self {
        let mut new_marked = self.marked.clone();
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let board_value: usize = self.values[row][col];
                if board_value == number {
                    new_marked[row][col] = true;

                    // assume there is only one possible play
                    break;
                }
            }
        }
        Board {
            values: self.values.clone(),
            marked: new_marked,
        }
    }

    fn is_winner(&self) -> bool {
        // check rows
        for row in 0..BOARD_SIZE {
            if self.marked[row].iter().filter(|&m| *m).count() == BOARD_SIZE {
                return true;
            }
        }

        // check columns
        for col in 0..BOARD_SIZE {
            if self.marked.iter().filter(|&row| row[col]).count() == BOARD_SIZE {
                return true;
            }
        }
        false
    }

    fn score(&self) -> usize {
        let mut total_score: usize = 0;
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if self.marked[row][col] == false {
                    total_score += self.values[row][col];
                }
            }
        }
        total_score
    }
}

fn get_input(file: &str) -> (Vec<usize>, Vec<Board>) {
    let contents = fs::read_to_string(file).unwrap();
    let input: Vec<&str> = contents.lines().filter(|l| l != &"").collect();
    let numbers: Vec<usize> = input[0]
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let board_input: Vec<Board> = input[1..].chunks(BOARD_SIZE).map(|c| Board::build(c)).collect();

    (numbers, board_input)
}

fn win(numbers: Vec<usize>, mut boards: Vec<Board>) -> usize {
    let mut score = 0;

    'outer: for num in numbers {
        let new_boards: Vec<Board> = boards.iter().map(|board| board.play_number(num)).collect();
        for board in &new_boards {
            if board.is_winner() {
                score = board.score() * num;
                break 'outer;
            }
        }
        boards = new_boards;
    }
    score
}

fn lose(numbers: Vec<usize>, mut boards: Vec<Board>) -> usize {
    let mut score = 0;

    for num in numbers {
        let played_boards: Vec<Board> = boards.iter().map(|board| board.play_number(num)).collect();

        if played_boards.len() == 1 {
            let last_card = played_boards[0];
            if last_card.is_winner() {
                score = last_card.score() * num;
                break;
            }
        }
        let mut non_winners: Vec<Board> = Vec::new();
        for board in &played_boards {
            if !board.is_winner() {
                non_winners.push(*board);
            }
        }
        boards = non_winners;
    }
    score
}

fn main() {
    let (numbers, boards) = get_input(&"input.txt");
    let score = win(numbers.clone(), boards.clone());
    println!("Winner with score {}", score);
    let score = lose(numbers, boards);
    println!("Loser with score {}", score);
}

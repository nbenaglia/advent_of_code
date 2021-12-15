use core::panic;
use ndarray::prelude::*;

static INPUT_FILE: &'static str = include_str!("input.txt");

struct bingo_number {
    number: u16,
    marked: bool
}

struct bingo_card {
    card: Array5<>
}

fn get_numbers() -> Vec<u8> {
    return INPUT_FILE
        .lines()
        .nth(0)
        .unwrap()
        .split(",")
        .map(|number| number.parse().unwrap())
        .collect();
}

fn get_boards() {
    return INPUT_FILE
    .lines()
    .nth(2)
    .unwrap()
    .split_ascii_whitespace()
    .map(|number| number.parse().unwrap())
    .collect();
}

fn main() {
    println!("{:?}", get_numbers());
}

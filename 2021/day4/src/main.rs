use core::panic;

static INPUT_FILE: &'static str = include_str!("input.txt");

fn get_numbers() -> Vec<u8> {
    return INPUT_FILE
        .lines()
        .nth(0)
        .unwrap()
        .split(",")
        .map(|number| number.parse().unwrap())
        .collect();
}

fn get_boards() {}

fn main() {
    println!("{:?}", get_numbers());
}

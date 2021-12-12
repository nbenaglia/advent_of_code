static INPUT_FILE: &'static str = include_str!("input.txt");

const BIT_LENGTH: usize = 12;

fn get_decimal_numbers() -> Vec<u16> {
    return INPUT_FILE
        .split_ascii_whitespace()
        .map(|str| u16::from_str_radix(str, 2).unwrap())
        .collect::<Vec<u16>>();
}

fn bit_matching(number: u16, position: usize, expected_bit: u16) -> bool {
    let mask = 1 << position;
    return ((number & mask) >> position) == (expected_bit as u16);
}

fn main() {
    let numbers = get_decimal_numbers();
    let mask: u16 = 0x0FFF;
    let half_total = numbers.len() / 2;
    let mut counts: [usize; BIT_LENGTH] = [0; BIT_LENGTH];

    for number in numbers {
        for position in 0..BIT_LENGTH {
            if bit_matching(number, position, 1) {
                counts[position] += 1;
            }
        }
    }

    // assemble back to a number
    let gamma: u16 = counts
        .iter()
        .enumerate()
        .fold(0, |accum, (position, count)| {
            let most_common_bit: u16 = if count > &half_total { 1 } else { 0 };
            return accum | (most_common_bit << position);
        });

    let epsilon = !gamma & mask;
    let multiplied = gamma as u64 * epsilon as u64;

    println!(
        "[Part 1] Gamma: {}, Epsilon: {}, Multiplied: {}",
        gamma, epsilon, multiplied
    );
}
static INPUT_FILE: &'static str = include_str!("input.txt");

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
    enum BitSearch {
        LeastCommon,
        MostCommon,
    }

    fn compute_rating(search: BitSearch) -> u16 {
        let mut numbers = get_decimal_numbers();
        let mut counts: [usize; 12] = [0; 12];
        let mut position = 12;

        while numbers.len() > 1 {
            position -= 1;
            let mut current_total = numbers.len();
            let half_total = current_total as f32 / 2.0;

            for number in &numbers {
                if bit_matching(*number, position, 1) {
                    counts[position] += 1;
                }
            }

            let count = counts[position];
            let common_bit = match search {
                BitSearch::MostCommon => {
                    if (count as f32) >= half_total {
                        1
                    } else {
                        0
                    }
                }
                BitSearch::LeastCommon => {
                    if (count as f32) < half_total {
                        1
                    } else {
                        0
                    }
                }
            };

            numbers.retain(|number| {
                let matched = bit_matching(*number, position, common_bit);

                if !matched {
                    current_total -= 1;
                }

                return matched || current_total == 0;
            });
        }

        assert_eq!(numbers.len(), 1);
        return numbers[0];
    }

    let oxygen_generator_rating = compute_rating(BitSearch::MostCommon);
    let co2_scrubber_rating = compute_rating(BitSearch::LeastCommon);

    println!("[Part 2]");
    println!("Oxygen generator rating: {}", oxygen_generator_rating);
    println!("CO2 scrubber rating: {}", co2_scrubber_rating);
    println!(
        "Multiplied together: {}",
        oxygen_generator_rating as u64 * co2_scrubber_rating as u64
    );
}

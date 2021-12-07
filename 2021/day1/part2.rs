use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");

    let mut previous: Option<u32> = None;
    let mut current: u32;
    let mut increments: u32 = 0;

    let lines: Vec<u32> = input.lines().map(|x| x.parse::<u32>().unwrap()).collect();
    for item in lines.windows(3) {
        current = item.iter().sum::<u32>();
        if let Some(value) = previous {
            if current > value {
                increments += 1;
            }
        }
        previous = Some(current);
    }

    println!("Part 2 (sliding windows): {}", increments);
}

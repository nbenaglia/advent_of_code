use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    
    let mut previous: Option<u32> = None;
    let mut current: Option<u32>;
    let mut increments: u32 = 0;

    for line in input.lines() {
        current = Some(line.parse::<u32>().unwrap());
        if let Some(value) = previous {
            if current.unwrap() > value {
                increments += 1;
            }
        } 
        previous = current;
    }
    
    println!("Part 1 (increments): {}", increments);
}

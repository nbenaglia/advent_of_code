use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");

    let mut depth: u32 = 0;
    let mut horizontal_position: u32 = 0;
    let mut aim: u32 = 0;

    for line in input.lines() {
        let mut instruction = line.split(" ");

        let command = instruction.next().unwrap();
        let value: u32 = instruction.next().unwrap().parse::<u32>().unwrap();

        match command {
            "up" => {
                aim -= value;
            }
            "down" => {
                aim += value;
            }
            "forward" => {
                horizontal_position += value;
                depth += aim * value;
                println!("Depth: {:?}", depth);
                println!("Horizontal position: {:?}", horizontal_position);
            }
            _ => (),
        }
    }
       
    println!("Result: {:?}", depth * horizontal_position);
}

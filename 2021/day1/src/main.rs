use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

const SLIDING_WINDOW_SIZE: usize = 3;

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn count_increments(values: &[i64]) -> i32 {
    let mut first_value = values[0];
    let mut increments = 0;
    for read in &values[1..] {
        if read > &first_value {
            increments += 1;
        };
        first_value = *read;
    }
    return increments;
}

fn count_increments_three_measurement_sliding_window(values: &[i64]) -> i32 {
    let mut increments: &i32 = &0;

    // stack containing the 2 windows to compare
    let mut window_stack: VecDeque<Vec<i64>> = VecDeque::with_capacity(2);

    let mut window1: Vec<i64> = Vec::with_capacity(SLIDING_WINDOW_SIZE);
    let mut window2: Vec<i64> = Vec::with_capacity(SLIDING_WINDOW_SIZE);
    let mut window3: Vec<i64> = Vec::with_capacity(SLIDING_WINDOW_SIZE);

    for (i, &read) in values.iter().enumerate() {
        match i % (SLIDING_WINDOW_SIZE - 1) {
            0 => {
                window1.push(read);
            }
            1 => {
                window1.push(read);
                window2.push(read);
            }
            2 => {
                window1.push(read);
                window2.push(read);
                window3.push(read);
            }
            _ => panic!("Undetermined result!"),
        }
        check_window(window1, &mut window_stack, &increments)
    }

    return *increments;
}

fn check_window(window: &mut Vec<i64>, window_stack: &mut VecDeque<Vec<i64>>, increments: i32) {
    if window.len() == SLIDING_WINDOW_SIZE {
        // push into the queue
        window_stack.push_back(window.clone());
        window.clear();

        if has_increment(&mut window_stack) {
            increments += 1;
        };
    }
}

fn has_increment(window_stack: &mut VecDeque<Vec<i64>>) -> bool {
    // check if there are two windows to compare
    if (window_stack.get(0)).is_some() && (window_stack.get(1)).is_some() {
        println!("0 {:?}", window_stack.get(0).unwrap().to_vec());
        println!("1 {:?}", window_stack.get(1).unwrap().to_vec());
        // compare window values
        let is_increment: bool = get_window_sum(window_stack.get(0).unwrap().to_vec())
            > get_window_sum(window_stack.get(1).unwrap().to_vec());

        // remove window from the front queue
        window_stack.pop_front();

        return is_increment;
    }
    return false;
}

fn get_window_sum(window: Vec<i64>) -> i64 {
    let mut sum = 0;
    for element in window {
        sum += element;
    }
    return sum;
}

fn main() -> Result<(), Error> {
    let data = read(File::open("input.txt")?)?;
    println!("Part 1 (increments): {}", count_increments(&data));
    println!(
        "Part 2 (sliding windows): {}",
        count_increments_three_measurement_sliding_window(&data)
    );
    Ok(())
}

use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

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
    let mut window: [i64; 3] = [values[0], values[1], values[2]];
    let mut increments = 0;

    let mut counter = 0;
    let mut current_window: [i64; 3] = [0, 0, 0];

    println!("{:?}", window);

    for read in &values[1..] {
        current_window[counter] = *read;
        if counter == 2 {
            if compare_window(&current_window, &window) {
                increments += 1;
            }
            counter = 0;
            println!("previous window {:?}", window);
            println!("current window {:?}", current_window);
            window = current_window;
            current_window = [0, 0, 0];
        } else {
            counter += 1;
        }
    }

    return increments;
}

fn compare_window(current_window: &[i64], previous_window: &[i64]) -> bool {
    return (current_window[0] + current_window[1] + current_window[2])
        > (previous_window[0] + previous_window[1] + previous_window[2]);
}

fn main() -> Result<(), Error> {
    let data = read(File::open("input.txt")?)?;
    println!("{}", count_increments(&data));
    println!(
        "{}",
        count_increments_three_measurement_sliding_window(&data)
    );
    Ok(())
}

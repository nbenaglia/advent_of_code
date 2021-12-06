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
    const SLIDING_WINDOW: i32 = 3;
    let mut window: [i64; 3] = values[0..SLIDING_WINDOW];
    let window_sum = &values[0] + &values[1] + &values[2];

    let mut increments = 0;
    
    for read in &values[3..] {
        let counter = 0;
        let current_window =
        if counter == 3 {
            if read > &window_sum {
                increments += 1;
            };
            window = *read;
        }
    }
    
    return increments;
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

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

fn main() -> Result<(), Error> {
    let data = read(File::open("input.txt")?)?;
    println!("{}", count_increments(&data));
    Ok(())
}

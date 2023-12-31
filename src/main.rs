use std::fs::File;
use std::io::{self, BufRead, BufReader};

mod days;

fn main() -> io::Result<()> {
    let mut values: Vec<String> = Vec::new();
    let file_path = "day1.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                values.push(days::day1::get_calibration_value(line));
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }
    let mut res = 0;
    for value in values {
        res += value.parse::<i32>().unwrap();
    }
    println!("Result: {}", res);

    Ok(())
}

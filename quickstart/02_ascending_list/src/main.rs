use std::io::{self, BufRead};

fn main() {
    let result = io::stdin()
        .lock()
        .lines()
        .next()
        .expect("expected one line input")
        .expect("expected to parse as a string")
        .split_ascii_whitespace()
        .map(|maybe_number| maybe_number.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()
        .expect("expected to parse all values as numbers")
        .windows(2)
        .all(|two_numbers| two_numbers[0] < two_numbers[1]);

    println!(
        "{}",
        match result {
            true => "YES",
            false => "NO",
        }
    );
}

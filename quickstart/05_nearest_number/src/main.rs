use std::io::{self, BufRead};

fn main() {
    let mut stdin_lines = io::stdin().lock().lines().take(3);
    let capacity = stdin_lines
        .next()
        .unwrap()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut numbers = Vec::<isize>::with_capacity(capacity);
    stdin_lines
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<isize>().unwrap())
        .for_each(|n| numbers.push(n));
    let target_number = stdin_lines
        .next()
        .unwrap()
        .unwrap()
        .parse::<isize>()
        .unwrap();

    let result = numbers
        .iter()
        .min_by_key(|&&num| (num - target_number).abs())
        .unwrap();

    println!("{}", result);
}

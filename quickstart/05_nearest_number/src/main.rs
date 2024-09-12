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

    // numbers.sort();

    let mut delta = std::isize::MAX;
    let mut nearest_number = std::isize::MAX;

    for number in &numbers {
        let current_delta = (number - target_number).abs();
        if current_delta < delta {
            delta = current_delta;
            nearest_number = *number;
        }
        // else {
        //     break;
        // }
    }

    println!("{}", nearest_number);

    // let result = numbers
    //     .iter()
    //     .min_by_key(|&&num| (num - target_number).abs())
    //     .unwrap();

    // println!("{}", result);
}

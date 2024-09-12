use std::io::{self, BufRead};

fn main() {
    let mut stdin_lines = io::stdin().lock().lines();

    let mut buttons = stdin_lines
        .next()
        .expect("expected first line input")
        .expect("expected to parse as a string")
        .split_ascii_whitespace()
        .take(3)
        .map(|s| s.chars().next().unwrap().to_digit(10))
        .collect::<Option<Vec<_>>>()
        .expect("expected 3 digits separated by space on the first line");
    buttons.sort();

    let number_digits = stdin_lines
        .next()
        .expect("expected second line input")
        .expect("expected to parse as a string")
        .chars()
        .map(|c| c.to_digit(10))
        .collect::<Option<Vec<_>>>()
        .unwrap();

    let mut buttons_to_add: Vec<u32> = Vec::new();

    let result = number_digits
        .iter()
        .filter(
            |&digit| match !buttons.contains(&digit) && !buttons_to_add.contains(&digit) {
                true => {
                    buttons_to_add.push(*digit);
                    true
                }
                false => false,
            },
        )
        .count();

    println!("{}", result);
}

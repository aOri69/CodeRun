use std::io::{self, BufRead};

fn main() {
    let mut stdin_iter = io::stdin().lock().lines().take(2);
    let element_count = match stdin_iter
        .next()
        .unwrap()
        .unwrap()
        .parse::<usize>()
        .unwrap()
    {
        n @ 1..=100 => n,
        _ => panic!("expected number in the range 0..100"),
    };
    let _numbers = stdin_iter
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .take(element_count)
        .map(|maybe_number| maybe_number.parse::<usize>())
        .map(|maybe_digit| match maybe_digit.ok() {
            Some(d) if d >= 1 && d <= 9 => Some(d),
            Some(_) => None,
            None => None,
        })
        .collect::<Option<Vec<_>>>()
        .expect("expected valid set of digits");
}

use std::io::{self, BufRead};

fn main() {
    let mut stdin_iter = io::stdin().lock().lines();

    let first_line = stdin_iter
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let _rows = *first_line.first().unwrap();
    let _cols = *first_line.get(1).unwrap();
    let mines = *first_line.get(2).unwrap();

    while let Some(Ok(mine_coord)) = stdin_iter.take(mines).next() {
        todo!();
    }
}

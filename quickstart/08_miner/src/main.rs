use std::io::{self, BufRead};

#[derive(Debug)]
struct MineCoord {
    row: usize,
    col: usize,
}

impl MineCoord {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Default)]
struct Field {
    rows: usize,
    cols: usize,
    mines: Vec<MineCoord>,
}

impl Field {
    fn set_rows(&mut self, rows: usize) {
        self.rows = rows;
    }

    fn set_cols(&mut self, cols: usize) {
        self.cols = cols;
    }

    fn add_mine(&mut self, mine: MineCoord) {
        self.mines.push(mine);
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

fn parse_input_field() -> Field {
    let mut stdin_iter = io::stdin().lock().lines();

    let first_line = stdin_iter
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut field = Field::default();

    field.set_rows(*first_line.first().unwrap());
    field.set_cols(*first_line.get(1).unwrap());

    let mines = *first_line.get(2).unwrap();
    let mut mines_iter = stdin_iter.take(mines);

    while let Some(Ok(mine_coord_str)) = mines_iter.next() {
        let (row, col) = mine_coord_str.split_once(" ").unwrap();
        field.add_mine(MineCoord::new(
            row.parse::<usize>().unwrap(),
            col.parse::<usize>().unwrap(),
        ));
    }

    field
}

fn main() {
    let field = parse_input_field();

    dbg!(field);
}

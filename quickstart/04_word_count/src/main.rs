use std::{
    collections::HashSet,
    io::{self, BufRead},
};

fn main() {
    let mut unique_words: HashSet<String> = HashSet::new();

    io::stdin().lock().lines().for_each(|line| {
        line.expect("expected valid string line")
            .split_ascii_whitespace()
            .for_each(|word| {
                unique_words.insert(word.trim().to_string());
            });
    });

    println!("{}", unique_words.len());
}

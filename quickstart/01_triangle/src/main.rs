use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines().take(3);

    let a = iter.next().unwrap().unwrap().parse::<u32>().unwrap();
    let b = iter.next().unwrap().unwrap().parse::<u32>().unwrap();
    let c = iter.next().unwrap().unwrap().parse::<u32>().unwrap();

    let result = if a + b > c && a + c > b && b + c > a {
        "YES"
    } else {
        "NO"
    };

    println!("{}", result);
}

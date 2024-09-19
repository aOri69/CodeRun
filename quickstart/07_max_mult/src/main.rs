use std::io::{self, BufRead};

fn main() {
    let mut _stdin_iter = io::stdin().lock().lines();
    todo!();
}

#[cfg(test)]
mod tests {
    // Bring outer functions into scope
    #[allow(unused_imports)]
    use crate::*;

    #[test]
    fn test_1() {
        assert_eq!(1, 0);
    }

    #[test]
    fn test_2() {
        assert_eq!(1, 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(1, 0);
    }
}

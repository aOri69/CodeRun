use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
struct MissedElements {
    count: usize,
    elements: Vec<usize>,
}

fn get_missed_elements(numbers: &[usize]) -> MissedElements {
    let len = numbers.len();
    for i in 0..len {
        let mut j = len - i - 1;
        debug(len, i, j, numbers);

        if numbers[i] != numbers[j] {
            continue;
        }

        // for j in (i..len).rev() {
        //     debug(len, i, j, numbers);
        //     let lhs = numbers[i];
        //     let rhs = numbers[j];
        //     if lhs != rhs {
        //         break;
        //     }
        //     if j == i {
        //         let result = numbers[..i].iter().cloned().rev().collect::<Vec<_>>();
        //         return MissedElements {
        //             count: result.len(),
        //             elements: result,
        //         };
        //     }
        // }
    }
    MissedElements {
        count: 0,
        elements: vec![],
    }
}

fn debug(len: usize, i: usize, j: usize, numbers: &[usize]) {
    let mut sel = " ".repeat(len * 2);
    sel.replace_range(i..i, "v");
    println!("{0:-<width$}", sel, width = len * 2);
    println!(
        "{0:-<width$}",
        numbers.iter().map(ToString::to_string).collect::<String>(),
        width = len * 2
    );
    sel = " ".repeat(len * 2);
    sel.replace_range(j..j, "^");
    println!("{0:-<width$}", sel, width = len * 2);
}

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
    let numbers = stdin_iter
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

    let result = get_missed_elements(&numbers);

    println!("{}", result.count);
    println!(
        "{}",
        result
            .elements
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    );
}

#[cfg(test)]
mod tests {
    use crate::MissedElements;

    // Bring outer functions into scope
    use super::get_missed_elements;

    #[test]
    fn test_1() {
        assert_eq!(
            get_missed_elements(&vec![1, 2, 3, 4, 5, 4, 3, 2, 1]),
            MissedElements {
                count: 0,
                elements: vec![]
            }
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            get_missed_elements(&vec![1, 2, 1, 2, 2]),
            MissedElements {
                count: 3,
                elements: vec![1, 2, 1]
            }
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            get_missed_elements(&vec![1, 2, 3, 4, 5]),
            MissedElements {
                count: 4,
                elements: vec![4, 3, 2, 1]
            }
        );
    }
}

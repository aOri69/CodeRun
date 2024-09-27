use std::cmp;
use std::io::{self, BufRead};

fn max_product_of_three_elements(numbers: Vec<i128>) -> [i128; 3] {
    // [0,1,2]
    // indexes 0,1 will elements of highest or lowest product of 2(highest pair/lowest pair)
    // index 2 will store next highest or lowest element
    let mut result: [i128; 3];
    let mut highest_pair: [i128; 2];
    let mut lowest_pair: [i128; 2];

    // numbers.sort();

    let mut highest = cmp::max(numbers[0], numbers[1]);
    let mut lowest = cmp::min(numbers[0], numbers[1]);
    let mut highest_product_of_2 = numbers[0] * numbers[1];
    let mut lowest_product_of_2 = numbers[0] * numbers[1];
    let mut highest_product_of_3 = numbers[0] * numbers[1] * numbers[2];

    result = *numbers.first_chunk::<3>().unwrap();
    highest_pair = *numbers.first_chunk::<2>().unwrap();
    lowest_pair = *numbers.first_chunk::<2>().unwrap();

    // println!("Start:   {numbers:?}");

    for &current_num in &numbers[2..] {
        println!("--------------------------------------------------");
        highest_product_of_3 = match cmp::max(
            highest_product_of_3,
            cmp::max(
                current_num * highest_product_of_2,
                current_num * lowest_product_of_2,
            ),
        ) {
            h if h == highest_product_of_3 => highest_product_of_3,
            h if h == current_num * highest_product_of_2 => {
                result[0] = highest_pair[0];
                result[1] = highest_pair[1];
                result[2] = current_num;
                current_num * highest_product_of_2
            }
            h if h == current_num * lowest_product_of_2 => {
                result[0] = lowest_pair[0];
                result[1] = lowest_pair[1];
                result[2] = current_num;
                current_num * lowest_product_of_2
            }
            _ => unreachable!(),
        };

        highest_product_of_2 = match cmp::max(
            highest_product_of_2,
            cmp::max(current_num * highest, current_num * lowest),
        ) {
            h if h == highest_product_of_2 => highest_product_of_2, // do not change elements
            h if h == current_num * highest => {
                highest_pair[0] = current_num;
                highest_pair[1] = highest;
                current_num * highest
            }
            h if h == current_num * lowest => {
                highest_pair[0] = current_num;
                highest_pair[1] = lowest;
                current_num * lowest
            }
            _ => unreachable!(),
        };
        lowest_product_of_2 = match cmp::min(
            lowest_product_of_2,
            cmp::min(current_num * highest, current_num * lowest),
        ) {
            h if h == lowest_product_of_2 => lowest_product_of_2, // do not change elements
            h if h == current_num * highest => {
                lowest_pair[0] = current_num;
                lowest_pair[1] = highest;
                current_num * highest
            }
            h if h == current_num * lowest => {
                lowest_pair[0] = current_num;
                lowest_pair[1] = lowest;
                current_num * lowest
            }
            _ => unreachable!(),
        };

        highest = cmp::max(current_num, highest);
        lowest = cmp::min(current_num, lowest);

        println!("current: [{current_num:?}]");
        println!("result:  {result:?}");
        println!("highest: {highest_pair:?}");
        println!("lowest:  {lowest_pair:?}");
    }
    result
}

fn main() {
    let mut stdin_iter = io::stdin().lock().lines().take(1);
    let numbers = stdin_iter
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .map(|num| num.parse::<i128>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    println!(
        "{}",
        max_product_of_three_elements(numbers)
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn _max_product_of_three(numbers: Vec<i64>) -> i64 {
    let mut highest = cmp::max(numbers[0], numbers[1]);
    let mut lowest = cmp::max(numbers[0], numbers[1]);
    let mut highest_product_of_2 = numbers[0] * numbers[1];
    let mut lowest_product_of_2 = numbers[0] * numbers[1];
    let mut highest_product_of_3 = numbers[0] * numbers[1] * numbers[2];

    println!("{numbers:?}");

    for &current_num in &numbers[2..] {
        highest_product_of_3 = cmp::max(
            highest_product_of_3,
            cmp::max(
                current_num * highest_product_of_2,
                current_num * lowest_product_of_2,
            ),
        );

        highest_product_of_2 = cmp::max(
            highest_product_of_2,
            cmp::max(current_num * highest, current_num * lowest),
        );

        lowest_product_of_2 = cmp::min(
            lowest_product_of_2,
            cmp::min(current_num * highest, current_num * lowest),
        );

        highest = cmp::max(current_num, highest);
        lowest = cmp::max(current_num, lowest);
    }
    highest_product_of_3
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    // Bring outer functions into scope
    #[allow(unused_imports)]
    use crate::*;

    #[rstest]
    // CodeRun variants
    #[case("3 5 1 7 9 0 9 -3 10", "10 9 9")]
    #[case("-5 -30000 -12", "-5 -12 -30000")]
    #[case("1 2 3", "3 2 1")]
    #[case("1 2 3 4 5 6", "4 5 6")] // all positive
    #[case("-1 -4 -5 -6 -2 -3", "-1 -2 -3")] // all negative
    #[case("1 2 3 0 5 0", "2 3 5")] // zeros
    #[case("-1 -2 -3 0 -5 0", "0 -2 -3")] // zeros
    #[case("0 0 0 0 0 0 0 0 0 0", "0 0 0")] // zeros
    #[case("9 -2 9 0 -5 9", "9 9 9")] // duplicates
    #[case(
        "999999999 -2 999999999 0 -5 999999999",
        "999999999 999999999 999999999"
    )] // large numbers
    #[case("1 0 0 0 1", "1 0 0")]
    #[case("-10 -10 1 3 2", "-10 -10 3")]
    #[case("-10 1 3 2 -10", "-10 -10 3")]

    fn test_fn(#[case] input: &str, #[case] expected: &str) {
        let numbers_input = input
            .split_ascii_whitespace()
            .map(|num| num.parse::<i128>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let mut numbers_expected = expected
            .split_ascii_whitespace()
            .map(|num| num.parse::<i128>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let mut numbers_result = max_product_of_three_elements(numbers_input).to_vec();

        numbers_expected.sort();
        numbers_result.sort();

        assert_eq!(numbers_result, numbers_expected);
    }
}

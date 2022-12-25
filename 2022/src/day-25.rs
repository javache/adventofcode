use std::io::{self, BufRead};

const LOOKUP: [char; 5] = ['=', '-', '0', '1', '2'];

fn snafu_to_num(input: String) -> i64 {
    input.chars().fold(0, |acc, c| {
        5 * acc + LOOKUP.iter().position(|&val| val == c).unwrap() as i64 - 2
    })
}

fn num_to_snafu(mut input: i64) -> String {
    let mut output = vec![];
    while input > 0 {
        output.push(LOOKUP[((input + 2) % 5) as usize]);
        input = (input + 2) / 5
    }
    output.iter().rev().collect()
}

fn main() {
    let sum: i64 = io::stdin().lock().lines().flatten().map(snafu_to_num).sum();
    println!("(1) {} is {} in SNAFU", sum, num_to_snafu(sum));
}

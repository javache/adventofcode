use itertools::Itertools;
use std::io::{self, BufRead};

fn solve(input: &Vec<i32>) -> (i32, i32) {
    let diffs = input
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect::<Vec<_>>();
    let delta = if diffs.iter().all(|&d| d == 0) {
        (0, 0)
    } else {
        solve(&diffs)
    };
    (input[0] - delta.0, input[input.len() - 1] + delta.1)
}

fn main() {
    let (left, right) = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| {
            let nums = line
                .split(" ")
                .flat_map(|x| x.parse::<i32>())
                .collect::<Vec<_>>();
            solve(&nums)
        })
        .fold((0, 0), |(l, r), (ll, rr)| (l + ll, r + rr));

    println!("(1) Extrapolating right gives {}", right);
    println!("(2) Extrapolating left gives {}", left);
}

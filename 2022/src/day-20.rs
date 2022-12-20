use std::collections::VecDeque;
use std::io::{self, BufRead};

const DECRYPTION_KEY: i64 = 811589153;

fn mix(mut vec: VecDeque<(i64, usize)>) -> VecDeque<(i64, usize)> {
    for i in 0..vec.len() {
        let idx = vec.iter().position(|&(_, pos)| pos == i).unwrap();
        let value = vec[idx].0;

        if value == 0 {
            continue;
        }

        let mut target_idx = idx as i64 + value;
        target_idx = target_idx.rem_euclid(vec.len() as i64 - 1);

        vec.remove(idx as usize);
        vec.insert(target_idx as usize, (value, i));
    }
    vec
}

fn calculate_sum(output: &VecDeque<(i64, usize)>) -> i64 {
    let zero_pos = output.iter().position(|&num| num.0 == 0).unwrap();
    [1_000, 2_000, 3_000]
        .iter()
        .map(|offset| output[(zero_pos + offset) % output.len()].0)
        .sum()
}

fn main() {
    let input: Vec<i64> = io::stdin()
        .lock()
        .lines()
        .flatten()
        .flat_map(|line| line.parse())
        .collect();

    let mixed = mix(input
        .iter()
        .enumerate()
        .map(|(idx, num)| (*num, idx))
        .collect());
    println!("(1) Sum = {}", calculate_sum(&mixed));

    let multiplied = input
        .iter()
        .enumerate()
        .map(|(idx, num)| (num * DECRYPTION_KEY, idx))
        .collect();
    let mixed = (0..10).fold(multiplied, |prev, _| mix(prev));
    println!("(2) Sum = {}", calculate_sum(&mixed));
}

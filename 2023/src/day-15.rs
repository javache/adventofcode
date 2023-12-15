use std::io::{self, BufRead};

fn hash(input: &[u8]) -> usize {
    input
        .iter()
        .fold(0, |acc, x| ((acc + *x as u32) * 17) % 256) as usize
}

const EMPTY_VEC: Vec<(Vec<u8>, u8)> = Vec::new();

fn main() {
    let mut boxes = [EMPTY_VEC; 256];
    let mut sum = 0;
    for step in io::stdin().lock().split(b',').flatten() {
        let step = if step.contains(&b'\n') {
            &step[..(step.len() - 1)]
        } else {
            &step[..]
        };
        sum += hash(step);

        if let Some(action_idx) = step.iter().position(|&c| c == b'=' || c == b'-') {
            let key = &step[0..action_idx];
            let box_ = &mut boxes[hash(key)];
            match step[action_idx] {
                b'-' => {
                    if let Some(value_idx) = box_.iter().position(|entry| entry.0 == key) {
                        box_.remove(value_idx);
                    }
                }
                b'=' => {
                    let value = (step[action_idx + 1] as char).to_digit(10).unwrap() as u8;
                    if let Some(value_idx) = box_.iter().position(|entry| entry.0 == key) {
                        box_[value_idx].1 = value;
                    } else {
                        box_.push((key.to_vec(), value));
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    println!("(1) Sum of hashes is {}", sum);

    let power = boxes
        .iter()
        .enumerate()
        .map(|(idx, entries)| {
            entries
                .iter()
                .enumerate()
                .map(|(entry_idx, entry)| (entry_idx + 1) * entry.1 as usize)
                .sum::<usize>()
                * (idx + 1)
        })
        .sum::<usize>();
    println!("(2) Total focusing power is {}", power);
}

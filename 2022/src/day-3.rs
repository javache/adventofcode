use std::collections::HashSet;
use std::io::{self, BufRead};

fn priority(c: &char) -> u32 {
    if c.is_lowercase() {
        (*c as u8 - b'a' + 1) as u32
    } else {
        (*c as u8 - b'A' + 27) as u32
    }
}

fn main() {
    let mut sum_of_priorities = 0;
    let mut sum_of_badge_priorities = 0;
    let mut curr_group: HashSet<char> = HashSet::new();

    for (idx, line) in io::stdin().lock().lines().flatten().enumerate() {
        let len = line.len() / 2;
        line.chars()
            .take(len)
            .collect::<HashSet<char>>()
            .intersection(&line.chars().skip(len).collect())
            .next()
            .map(|item| sum_of_priorities += priority(item));

        let backpack = line.chars().collect::<HashSet<char>>();
        curr_group = if curr_group.is_empty() {
            backpack
        } else {
            curr_group.intersection(&backpack).cloned().collect()
        };
        if idx % 3 == 2 {
            curr_group
                .iter()
                .next()
                .map(|item| sum_of_badge_priorities += priority(item));
            curr_group.clear();
        }
    }

    println!("(1) Sum of priorities is {}", sum_of_priorities);
    println!(
        "(2) Sum of priorities of badges is {}",
        sum_of_badge_priorities
    );
}

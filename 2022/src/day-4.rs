use std::io::{self, BufRead};

fn parse_pair(input: &str) -> (i32, i32) {
    if let [first, last] = input.split('-').flat_map(str::parse).collect::<Vec<i32>>()[..] {
        (first, last)
    } else {
        panic!("Unexpected input {}", input);
    }
}

fn main() {
    let mut contained_pairs: usize = 0;
    let mut overlapping_pairs: usize = 0;

    for line in io::stdin().lock().lines().flatten() {
        let pairs: Vec<(i32, i32)> = line.split(',').map(parse_pair).collect();
        contained_pairs += ((pairs[0].0 >= pairs[1].0 && pairs[0].1 <= pairs[1].1)
            || (pairs[1].0 >= pairs[0].0 && pairs[1].1 <= pairs[0].1))
            as usize;
        overlapping_pairs += (pairs[0].0 <= pairs[1].1 && pairs[0].1 >= pairs[1].0) as usize;
    }

    println!("(1) There are {} contained pairs", contained_pairs);
    println!("(2) There are {} overlapping pairs", overlapping_pairs);
}

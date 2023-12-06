use std::io::{self, BufRead};

fn solve(time: u64, distance: u64) -> u64 {
    // solve x * (time - x) > distance
    let root = ((time * time - 4 * distance) as f64).sqrt();
    ((0.5 * (time as f64 + root)).ceil() - (0.5 * (time as f64 - root)).floor() - 1.0) as u64
}

fn main() {
    let input = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| {
            line.split(' ')
                .map(|col| col.trim().parse())
                .flatten()
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<_>>();

    let option_count: u64 = (0..input[0].len())
        .map(|i| solve(input[0][i], input[1][i]))
        .product();
    println!(
        "(1) The product of the number of options is {}",
        option_count
    );

    let combined_time = input[0].iter().map(&u64::to_string).collect::<String>();
    let combined_distance = input[1].iter().map(&u64::to_string).collect::<String>();
    println!(
        "(2) The number of options for the combined value is {}",
        solve(
            combined_time.parse().unwrap(),
            combined_distance.parse().unwrap()
        )
    );
}

use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line)?;

    let mut positions: Vec<i32> = line.split(',').filter_map(|n| n.parse().ok()).collect();
    positions.sort();

    let median = positions[positions.len() / 2];
    let cost: i32 = positions.iter().map(|p| i32::abs(median - p)).sum();
    println!("(1) Median of positions is {}, cost is {}", median, cost);

    let mean = (positions.iter().sum::<i32>() as f32 / positions.len() as f32).floor() as i32;
    let cost2: i32 = positions
        .iter()
        .map(|p| {
            let delta = i32::abs(mean - p);
            delta * (delta + 1) / 2
        })
        .sum();
    println!("(2) Mean of positions is {}, cost is {}", mean, cost2);

    Ok(())
}

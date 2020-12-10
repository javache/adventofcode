use std::collections::HashSet;
use std::env;
use std::io::{self, BufRead};

fn read_numbers() -> io::Result<Vec<u64>> {
    let mut result = Vec::new();
    for line in io::stdin().lock().lines() {
        if let Ok(number) = line?.parse::<u64>() {
            result.push(number);
        }
    }
    Ok(result)
}

fn main() -> io::Result<()> {
    let window_size = env::args()
        .nth(1)
        .and_then(|arg| arg.parse::<usize>().ok())
        .unwrap();
    let input = read_numbers()?;

    let mut window: HashSet<_> = input[0..window_size].into_iter().collect();
    if let Some((_, target_sum)) = input
        .iter()
        .enumerate()
        .skip(window_size)
        .find(|(i, target)| {
            let has_sum = input[(i - window_size)..*i]
                .iter()
                .any(|elem| *target > elem && window.contains(&(*target - elem)));
            window.remove(&input[i - window_size]);
            window.insert(&input[*i]);
            !has_sum
        })
    {
        println!("(1) Did not find sum in input for {}", target_sum);

        let mut start_idx = 0;
        let mut sum = input[0];
        if let Some((min, max)) = input.iter().enumerate().skip(1).find_map(|(idx, elem)| {
            sum += elem;
            while start_idx < idx && sum > *target_sum {
                sum -= input[start_idx];
                start_idx += 1;
            }
            if sum == *target_sum {
                let range = &input[start_idx..=idx];
                range.iter().min().zip(range.iter().max())
            } else {
                None
            }
        }) {
            println!("(2) Found sum from {} to {} = {}", min, max, min + max);
        }
    }

    Ok(())
}

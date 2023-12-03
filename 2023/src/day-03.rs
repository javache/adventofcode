use itertools::iproduct;
use itertools::Itertools;
use std::collections::HashMap;
use std::io::{self, BufRead};

type Grid = Vec<Vec<char>>;

fn find_adjacent_symbol(
    grid: &Grid,
    min_x: usize,
    max_x: usize,
    y: usize,
) -> Option<(usize, usize)> {
    let min_y = y.saturating_sub(1);
    let max_y = if y + 1 < grid.len() { y + 1 } else { y };

    let min_x = min_x.saturating_sub(1);
    let max_x = if max_x + 1 < grid[0].len() {
        max_x + 1
    } else {
        max_x
    };

    iproduct!(min_y..=max_y, min_x..=max_x)
        .find(|&(y, x)| !grid[y][x].is_digit(10) && grid[y][x] != '.')
}

fn main() {
    let input: Grid = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| line.chars().collect())
        .collect();

    let mut sum_of_parts = 0;
    let mut symbols_matched: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for y in 0..input.len() {
        for (is_digit, group) in &input[y]
            .iter()
            .enumerate()
            .group_by(|(_, sym)| sym.is_digit(10))
        {
            if !is_digit {
                continue;
            }

            let (x, num): (Vec<usize>, String) = group.unzip();
            if let Some(symbol_pos) = find_adjacent_symbol(&input, x[0], x[x.len() - 1], y) {
                let num = num.parse::<u32>().unwrap();
                sum_of_parts += num;

                if input[symbol_pos.0][symbol_pos.1] == '*' {
                    symbols_matched.entry(symbol_pos).or_default().push(num);
                }
            }
        }
    }

    println!("(1) Sum of all part numbers is {}", sum_of_parts);

    let mut sum_of_gears = 0;
    for (_, vals) in symbols_matched {
        if vals.len() == 2 {
            sum_of_gears += vals[0] * vals[1];
        }
    }

    println!(
        "(2) The sum of the product all of gears is {}",
        sum_of_gears
    )
}

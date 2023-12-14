use std::collections::HashMap;
use std::io::{self, BufRead};

type Grid = Vec<Vec<char>>;

fn calculate_load(grid: &Grid) -> usize {
    grid.iter()
        .enumerate()
        .map(|(idx, row)| row.iter().filter(|&c| *c == 'O').count() * (grid.len() - idx))
        .sum()
}

fn transpose(v: &Grid) -> Grid {
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i]).collect::<Vec<_>>())
        .collect()
}

fn reverse(v: &Grid) -> Grid {
    v.iter()
        .map(|row| row.iter().rev().cloned().collect::<Vec<_>>())
        .collect()
}

fn tilt_west(mut grid: Grid) -> Grid {
    for row in &mut grid {
        let mut next_free = 0;
        for i in 0..row.len() {
            if row[i] == '#' {
                next_free = i + 1;
            } else if row[i] == 'O' {
                while next_free < i && row[next_free] != '.' {
                    next_free += 1;
                }
                if i != next_free {
                    row[next_free] = 'O';
                    row[i] = '.';
                    next_free += 1;
                }
            }
        }
    }
    grid
}

fn tilt_north(grid: Grid) -> Grid {
    transpose(&tilt_west(transpose(&grid)))
}

fn tilt_south(grid: Grid) -> Grid {
    transpose(&reverse(&tilt_west(reverse(&transpose(&grid)))))
}

fn tilt_east(grid: Grid) -> Grid {
    reverse(&tilt_west(reverse(&grid)))
}

fn main() {
    let input: Grid = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let result = tilt_north(input.clone());
    println!(
        "(1) Load after tilting north is {}",
        calculate_load(&result)
    );

    let mut result = input;
    let mut seen = HashMap::new();
    let mut remaining_cycles = 0;

    for i in 0..1_000_000_000 {
        result = tilt_east(tilt_south(tilt_west(tilt_north(result))));
        if seen.contains_key(&result) {
            let cycle_length = i - seen[&result];
            remaining_cycles = (1_000_000_000 - i) % cycle_length;
            break;
        } else {
            seen.insert(result.clone(), i);
        }
    }

    for _ in 1..remaining_cycles {
        result = tilt_east(tilt_south(tilt_west(tilt_north(result))));
    }
    println!(
        "(2) Load after tilting a million times is {}",
        calculate_load(&result)
    );
}

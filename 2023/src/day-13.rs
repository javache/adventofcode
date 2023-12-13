use std::io::{self, Read};

type Grid = Vec<Vec<char>>;

fn find_reflection(grid: &Grid, expected_delta: usize) -> Option<usize> {
    (1..grid.len()).find(|&row| {
        let delta = (0..(grid.len()))
            .map(|i| {
                if i + 1 > row || row + i > grid.len() - 1 {
                    0
                } else {
                    grid[row - i - 1]
                        .iter()
                        .zip(grid[row + i].iter())
                        .filter(|(a, b)| (a != b))
                        .count()
                }
            })
            .sum::<usize>();
        delta == expected_delta
    })
}

fn transpose(v: &Grid) -> Grid {
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i]).collect::<Vec<_>>())
        .collect()
}

fn solve(grid: &Grid, expected_delta: usize) -> usize {
    find_reflection(grid, expected_delta)
        .map(|n| n * 100)
        .or_else(|| {
            let transposed = transpose(grid);
            find_reflection(&transposed, expected_delta)
        })
        .unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();

    let (mut sum_a, mut sum_b) = (0, 0);
    for pattern in input.split("\n\n") {
        let grid: Grid = pattern.lines().map(|line| line.chars().collect()).collect();
        sum_a += solve(&grid, 0);
        sum_b += solve(&grid, 1);
    }
    println!("(1) Sum is {}", sum_a);
    println!("(2) Sum is {}", sum_b);
}

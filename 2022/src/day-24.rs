use itertools::iproduct;
use std::collections::HashSet;
use std::io::{self, BufRead};

type Grid = Vec<Vec<u8>>;
type Point = (i32, i32);

fn wrap(value: usize, delta: i32, len: usize) -> usize {
    ((value as i32 + delta - 1).rem_euclid(len as i32 - 2) + 1) as usize
}

fn grid_at_time(starting_grid: &Grid, time: usize) -> Grid {
    let grid_size = (starting_grid[0].len(), starting_grid.len());
    let mut grid: Grid = vec![vec![0; grid_size.0]; grid_size.1];
    for (y, x) in iproduct!(0..grid_size.1, 0..grid_size.0) {
        let value = starting_grid[y][x];
        if value == 1 {
            grid[y][x] = 1;
        } else if value > 1 {
            if value & (1 << 1) != 0 {
                grid[y][wrap(x, time as i32, grid_size.0)] |= 1 << 0;
            }
            if value & (1 << 2) != 0 {
                grid[wrap(y, time as i32, grid_size.1)][x] |= 1 << 1;
            }
            if value & (1 << 3) != 0 {
                grid[y][wrap(x, -(time as i32), grid_size.0)] |= 1 << 2;
            }
            if value & (1 << 4) != 0 {
                grid[wrap(y, -(time as i32), grid_size.1)][x] |= 1 << 3;
            }
        }
    }
    grid
}

fn find_path(starting_grid: &Grid, start: Point, end: Point, start_time: usize) -> usize {
    let mut positions = HashSet::from([start]);
    for time in start_time.. {
        let grid = grid_at_time(starting_grid, time);
        positions = positions
            .into_iter()
            .flat_map(|pos| {
                [(0, 1), (1, 0), (-1, 0), (0, -1), (0, 0)]
                    .iter()
                    .flat_map(|(dx, dy)| {
                        let next_pos = (pos.0 as i32 + dx, pos.1 as i32 + dy);
                        (next_pos.0 >= 0
                            && next_pos.0 < grid[0].len() as i32
                            && next_pos.1 >= 0
                            && next_pos.1 < grid.len() as i32
                            && grid[next_pos.1 as usize][next_pos.0 as usize] == 0)
                            .then(|| next_pos)
                    })
                    .collect::<Vec<Point>>()
            })
            .collect();
        if positions.contains(&end) {
            return time;
        }
    }
    panic!("Did not return from loop");
}

fn main() {
    let input: Grid = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => 0,
                    '#' => 1 << 0,
                    '>' => 1 << 1,
                    'v' => 1 << 2,
                    '<' => 1 << 3,
                    '^' => 1 << 4,
                    _ => panic!("Invalid char {}", c),
                })
                .collect()
        })
        .collect();

    let start = (1, 0);
    let end = (input[0].len() as i32 - 2, input.len() as i32 - 1);
    let trip_1_time = find_path(&input, start, end, 1);
    println!("(1) Found a path in {} minutes", trip_1_time);

    let trip_2_time = find_path(&input, end, start, trip_1_time);
    println!(
        "(2) Found a path in {} minutes",
        find_path(&input, start, end, trip_2_time)
    );
}

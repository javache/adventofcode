use itertools::Itertools;
use std::collections::HashSet;
use std::io::{self, BufRead};

type Grid = Vec<Vec<u8>>;
type Point = (usize, usize);

const DIRECTIONS: &'static [(i32, i32)] = &[(0, -1), (0, 1), (-1, 0), (1, 0)];

fn get_neighbours_in_dir<'a>(
    base: &'a Point,
    dir: &'a (i32, i32),
    grid_size: usize,
) -> impl Iterator<Item = Point> + 'a {
    (1..grid_size as i32)
        .map(|n| {
            (
                (base.0 as i32 + dir.0 * n) as usize,
                (base.1 as i32 + dir.1 * n) as usize,
            )
        })
        .filter(move |p| p.0 < grid_size && p.1 < grid_size)
}

fn is_border_point(p: &Point, grid_size: usize) -> bool {
    p.0 == 0 || p.0 == grid_size || p.1 == 0 || p.1 == grid_size
}

fn is_visible(grid: &Grid, base: &Point) -> bool {
    let grid_size = grid.len();
    if is_border_point(base, grid_size) {
        return true;
    }

    let base_val = grid[base.1][base.0];
    DIRECTIONS
        .iter()
        .any(|dir| get_neighbours_in_dir(base, dir, grid_size).all(|p| grid[p.1][p.0] < base_val))
}

fn scenic_score(grid: &Grid, base: &Point) -> usize {
    let base_val = grid[base.1][base.0];
    DIRECTIONS
        .iter()
        .map(|dir| {
            let mut distance = 0;
            for p in get_neighbours_in_dir(base, dir, grid.len()) {
                distance += 1;
                if grid[p.1][p.0] >= base_val {
                    break;
                }
            }
            distance
        })
        .product()
}

fn main() {
    let grid: Grid = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| line.chars().map(|c| (c as u8) - b'0').collect())
        .collect();
    let grid_size = grid.len();
    let visible_points: HashSet<Point> = (0..grid_size)
        .cartesian_product(0..grid_size)
        .filter(|p| is_visible(&grid, &p))
        .collect();
    println!(
        "(1) There are {} trees visible from outside the grid",
        visible_points.len()
    );

    let best_score = (0..grid_size)
        .cartesian_product(0..grid_size)
        .map(|point| scenic_score(&grid, &point))
        .max();
    println!("(2) Highest scenic score is {}", best_score.unwrap());
}

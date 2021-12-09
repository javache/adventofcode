use itertools::iproduct;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::io::{self, BufRead};

type Grid = Vec<Vec<u8>>;
type Point = (usize, usize);

const NEIGHBOURS: [(i32, i32); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

fn get_neighbours<'a>(grid: &'a Grid, point: &'a Point) -> impl Iterator<Item = Point> + 'a {
    NEIGHBOURS.iter().flat_map(|(dx, dy)| {
        let neighbour = (point.0 as i32 + dx, point.1 as i32 + dy);
        ((0..grid.len() as i32).contains(&neighbour.0)
            && (0..grid[0].len() as i32).contains(&neighbour.1))
        .then(|| (neighbour.0 as usize, neighbour.1 as usize))
    })
}

fn find_low_points(grid: &Grid) -> Vec<Point> {
    iproduct!(0..grid.len(), 0..grid[0].len())
        .flat_map(|point| {
            get_neighbours(grid, &point)
                .all(|n| grid[n.0][n.1] > grid[point.0][point.1])
                .then(|| point)
        })
        .collect()
}

fn explore_basin(grid: &Grid, curr: &Point, seen: &mut HashSet<Point>) {
    let curr_value = grid[curr.0][curr.1];
    for neighbour in get_neighbours(grid, curr) {
        if ((curr_value + 1)..9).contains(&grid[neighbour.0][neighbour.1])
            && !seen.contains(&neighbour)
        {
            seen.insert(neighbour);
            explore_basin(grid, &neighbour, seen);
        }
    }
}

fn main() -> io::Result<()> {
    let grid: Grid = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| line.chars().map(|c| (c as u8) - b'0').collect())
        .collect();

    let low_points = find_low_points(&grid);
    let risk_level: u32 = low_points
        .iter()
        .map(|(x, y)| 1 + (grid[*x as usize][*y as usize] as u32))
        .sum();
    println!("(1) The risk level of all low points is {}", risk_level);

    let mut basin_sizes: BinaryHeap<usize> = low_points
        .iter()
        .map(|point| {
            let mut basin_set = HashSet::from([*point]);
            explore_basin(&grid, point, &mut basin_set);
            basin_set.len()
        })
        .collect();
    let largest_basins_product: usize =
        vec![basin_sizes.pop(), basin_sizes.pop(), basin_sizes.pop()]
            .iter()
            .flatten()
            .product();
    println!(
        "(2) The product of three largest basisns is {}",
        largest_basins_product
    );

    Ok(())
}

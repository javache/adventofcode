use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::io::{self, BufRead};

type Point = (i8, i8, i8);
type Grid = HashSet<Point>;

fn neighbours(&(x, y, z): &Point) -> [Point; 6] {
    [
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y - 1, z),
        (x, y + 1, z),
        (x, y, z - 1),
        (x, y, z + 1),
    ]
}

fn count_unconnected_sides(grid: &Grid) -> usize {
    grid.iter()
        .map(|point| {
            neighbours(point)
                .iter()
                .filter(|neighbour| !grid.contains(neighbour))
                .count()
        })
        .sum()
}

fn flood_fill(grid: &Grid, bounds: &[(i8, i8); 3]) -> HashSet<Point> {
    // Outset the bounds to control for droplets in the corners
    let bounds = bounds.map(|bound| (bound.0 - 1)..=(bound.1 + 1));

    // Start floodfill from the top-left-front corner of the cube
    let mut queue = VecDeque::from([(*bounds[0].start(), *bounds[1].start(), *bounds[2].start())]);

    let mut flood = HashSet::new();
    while let Some(elem) = queue.pop_front() {
        if grid.contains(&elem) || flood.contains(&elem) {
            continue;
        }
        flood.insert(elem);

        for neighbour in neighbours(&elem) {
            if bounds[0].contains(&neighbour.0)
                && bounds[1].contains(&neighbour.1)
                && bounds[2].contains(&neighbour.2)
            {
                queue.push_back(neighbour);
            }
        }
    }
    flood
}

fn count_reachable_sides(grid: &Grid) -> usize {
    let bounds = [
        grid.iter().map(|c| c.0).minmax(),
        grid.iter().map(|c| c.1).minmax(),
        grid.iter().map(|c| c.2).minmax(),
    ]
    .map(|minmax| minmax.into_option().unwrap());

    let flood = flood_fill(grid, &bounds);
    grid.iter()
        .map(|point| {
            neighbours(point)
                .iter()
                .filter(|point| flood.contains(point))
                .count()
        })
        .sum()
}

fn main() {
    let grid: Grid = io::stdin()
        .lock()
        .lines()
        .flatten()
        .flat_map(|line| {
            line.split(',')
                .map(str::parse::<i8>)
                .flatten()
                .collect_tuple()
        })
        .collect();

    println!(
        "(1) There are {} unconnected sides to all cubes",
        count_unconnected_sides(&grid)
    );
    println!(
        "(2) There are {} sides reachable from the outside",
        count_reachable_sides(&grid)
    );
}

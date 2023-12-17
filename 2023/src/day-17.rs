use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

type Grid = Vec<Vec<u8>>;
type Point = (usize, usize);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Eq, PartialEq)]
struct Vertex {
    cost: u32,
    position: Point,
    orientation: Orientation,
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const DIRECTIONS: &'static [&[(i32, i32)]; 2] = &[&[(0, -1), (0, 1)], &[(-1, 0), (1, 0)]];

fn find_valid_steps<'a>(
    grid: &'a Grid,
    point: Point,
    orientation: Orientation,
    movement_range: (i32, i32),
) -> impl Iterator<Item = (Point, u32)> + 'a {
    let grid_size = (0..grid.len() as i32, 0..grid[0].len() as i32);
    DIRECTIONS[orientation as usize]
        .iter()
        .flat_map(move |(dx, dy)| {
            let mut cost = 0;
            (1..=(movement_range.1))
                .filter_map(|n| {
                    let position = (point.0 as i32 + dx * n, point.1 as i32 + dy * n);
                    if !grid_size.0.contains(&position.0) || !grid_size.1.contains(&position.1) {
                        return None;
                    }
                    cost += grid[position.0 as usize][position.1 as usize] as u32;
                    (n >= movement_range.0)
                        .then(|| ((position.0 as usize, position.1 as usize), cost))
                })
                .collect::<Vec<_>>()
        })
}

fn solve_dijkstra(grid: &Grid, start: &Point, end: &Point, movement_range: (i32, i32)) -> u32 {
    let mut distances = vec![vec![[u32::MAX, u32::MAX]; grid[0].len()]; grid.len()];
    distances[start.0][start.0] = [0, 0];

    let mut heap = [Orientation::Horizontal, Orientation::Vertical]
        .iter()
        .map(|&orientation| Vertex {
            cost: 0,
            position: *start,
            orientation,
        })
        .collect::<BinaryHeap<_>>();

    while let Some(Vertex {
        cost,
        position,
        orientation,
    }) = heap.pop()
    {
        if position == *end {
            return cost;
        }
        if cost > distances[position.0][position.1][orientation as usize] {
            continue;
        }
        let next_orientation = if orientation == Orientation::Horizontal {
            Orientation::Vertical
        } else {
            Orientation::Horizontal
        };
        for (neighbour, mut next_cost) in
            find_valid_steps(grid, position, orientation, movement_range)
        {
            next_cost += cost;
            let distance_ref = &mut distances[neighbour.0][neighbour.1][next_orientation as usize];
            if next_cost < *distance_ref {
                *distance_ref = next_cost;
                heap.push(Vertex {
                    cost: next_cost,
                    position: neighbour,
                    orientation: next_orientation,
                });
            }
        }
    }

    panic!("No path found");
}

fn main() {
    let input: Grid = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect();

    let start = (0, 0);
    let end = (input.len() - 1, input[0].len() - 1);

    println!(
        "(1) The shortest path when going 1-3 blocks at a time has cost {}",
        solve_dijkstra(&input, &start, &end, (1, 3))
    );
    println!(
        "(2) The shortest path when going 4-10 blocks at a time has cost {}",
        solve_dijkstra(&input, &start, &end, (4, 10))
    );
}

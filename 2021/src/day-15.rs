use itertools::iproduct;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

type Grid = Vec<Vec<u8>>;
type Point = (usize, usize);

#[derive(Copy, Clone, Eq, PartialEq)]
struct Vertex {
    cost: u32,
    position: Point,
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_neighbours<'a>(grid: &'a Grid, point: Point) -> impl Iterator<Item = Point> + 'a {
    [(0, -1), (-1, 0), (0, 1), (1, 0)]
        .into_iter()
        .flat_map(move |(dx, dy)| {
            let neighbour = (point.0 as i32 + dx, point.1 as i32 + dy);
            ((0..grid.len() as i32).contains(&neighbour.0)
                && (0..grid[0].len() as i32).contains(&neighbour.1))
            .then(|| (neighbour.0 as usize, neighbour.1 as usize))
        })
}

fn find_cost_of_shortest_path(grid: &Grid, start: Point, goal: Point) -> u32 {
    let mut distances = vec![vec![u32::MAX; grid[0].len()]; grid.len()];
    distances[start.0][start.1] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(Vertex {
        cost: 0,
        position: start,
    });

    while let Some(Vertex { cost, position }) = heap.pop() {
        if position == goal {
            return cost;
        }
        if cost > distances[position.0][position.1] {
            continue;
        }
        for edge in get_neighbours(grid, position) {
            let next = Vertex {
                cost: cost + grid[edge.0][edge.1] as u32,
                position: edge,
            };
            if next.cost < distances[edge.0][edge.1] {
                heap.push(next);
                distances[edge.0][edge.1] = next.cost;
            }
        }
    }
    panic!("No path found");
}

fn expand_grid(input: &Grid) -> Grid {
    let (input_width, input_height) = (input[0].len(), input.len());

    let mut expanded_grid = input.clone();
    expanded_grid.resize(input_height * 5, vec![]);
    expanded_grid
        .iter_mut()
        .for_each(|row| row.resize(input_width * 5, 0));

    for (j, i) in iproduct!(0..5, 0..5) {
        if i == 0 && j == 0 {
            continue;
        }
        for (y, x) in iproduct!(0..input_height, 0..input_width) {
            let value = input[y][x] + (j + i) as u8;
            expanded_grid[input_height * j + y][input_width * i + x] = value.min(value % 10 + 1);
        }
    }
    expanded_grid
}

fn main() -> io::Result<()> {
    let grid: Grid = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| line.chars().map(|c| (c as u8) - b'0').collect())
        .collect();

    let target = (grid[0].len() - 1, grid.len() - 1);
    println!(
        "(1) Shortest path from (0, 0) to {:?} costs {}",
        target,
        find_cost_of_shortest_path(&grid, (0, 0), target)
    );

    let expanded_grid = expand_grid(&grid);
    let expanded_grid_target = (expanded_grid[0].len() - 1, expanded_grid.len() - 1);
    println!(
        "(2) Shortest path from (0, 0) to {:?} costs {}",
        expanded_grid_target,
        find_cost_of_shortest_path(&expanded_grid, (0, 0), expanded_grid_target),
    );

    Ok(())
}

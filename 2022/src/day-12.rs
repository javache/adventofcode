use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

type Grid = Vec<Vec<char>>;
type Point = (usize, usize);

#[derive(Eq, PartialEq)]
struct Vertex {
    cost: u32,
    position: Point,
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

fn find_valid_steps<'a>(point: Point, grid: &'a Grid) -> impl Iterator<Item = Point> + 'a {
    let curr_elevation = grid[point.1][point.0] as i8;
    let grid_size = (0..grid[0].len() as i32, 0..grid.len() as i32);
    [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .into_iter()
        .filter_map(move |(dx, dy)| {
            let neighbour = (point.0 as i32 + dx, point.1 as i32 + dy);
            (grid_size.0.contains(&neighbour.0) && grid_size.1.contains(&neighbour.1))
                .then(|| ((neighbour.0 as usize), (neighbour.1 as usize)))
        })
        .filter(move |neighbour| (grid[neighbour.1][neighbour.0] as i8) - curr_elevation <= 1)
}

fn solve_dijkstra(start: &Point, end: &Point, grid: &Grid) -> u32 {
    let mut distances = vec![vec![u32::MAX; grid[0].len()]; grid.len()];
    distances[start.1][start.0] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(Vertex {
        cost: 0,
        position: *start,
    });

    while let Some(Vertex { cost, position }) = heap.pop() {
        if position == *end {
            return cost;
        }
        if cost > distances[position.1][position.0] {
            continue;
        }
        for neighbour in find_valid_steps(position, grid) {
            let next = Vertex {
                cost: cost + 1,
                position: neighbour,
            };
            if next.cost < distances[neighbour.1][neighbour.0] {
                distances[neighbour.1][neighbour.0] = next.cost;
                heap.push(next);
            }
        }
    }
    panic!("No path found");
}

fn find_position_with_value(grid: &Grid, value: &char) -> Vec<Point> {
    grid.iter()
        .enumerate()
        .filter_map(|(idx, row)| row.iter().position(|c| c == value).map(|col| (col, idx)))
        .collect()
}

fn main() -> io::Result<()> {
    let mut input: Grid = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| line.chars().collect())
        .collect();

    let start = find_position_with_value(&input, &'S')[0];
    let end = find_position_with_value(&input, &'E')[0];
    input[start.1][start.0] = 'a';
    input[end.1][end.0] = 'z';

    println!(
        "(1) The minimal cost for a solution is {}",
        solve_dijkstra(&start, &end, &input)
    );

    println!(
        "(2) The minimal cost for a solution is {}",
        find_position_with_value(&input, &'a')
            .iter()
            .map(|start| { solve_dijkstra(&start, &end, &input) })
            .min()
            .unwrap()
    );

    Ok(())
}

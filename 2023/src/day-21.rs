use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::{self, BufRead};

type Grid = Vec<Vec<char>>;
type Point = (i32, i32);

const DIRECTIONS: &'static [(i32, i32)] = &[(0, -1), (0, 1), (-1, 0), (1, 0)];

fn walk(grid: &Grid, steps: &[usize], start: Point) -> Vec<usize> {
    let grid_size = grid.len() as i32;

    let mut visited: HashMap<Point, usize> = [(start, 0)].into();
    let mut q: VecDeque<Point> = [start].into();
    let mut output = vec![];

    let max_steps = *steps.iter().max().unwrap();
    for i in 1..=max_steps {
        let mut q2 = VecDeque::new();
        while let Some((y, x)) = q.pop_front() {
            for (dy, dx) in DIRECTIONS {
                let p = (y + dy, x + dx);

                if grid[p.0.rem_euclid(grid_size) as usize][p.1.rem_euclid(grid_size) as usize]
                    == '#'
                {
                    continue;
                }

                if !visited.contains_key(&p) {
                    visited.insert(p, i);
                    q2.push_back(p);
                }
            }
        }

        if i == steps[output.len()] {
            let reachable = visited.values().filter(|&v| v % 2 == i % 2).count();
            output.push(reachable);
        }

        q = q2;
    }
    output
}

// Lagrange's Interpolation formula for ax^2 + bx + c with x=[0,1,2] and y=[y0,y1,y2]
fn lagrange_interpolation(steps: &[i64]) -> [i64; 3] {
    [
        (steps[0] - 2 * steps[1] + steps[2]) / 2,
        (-3 * steps[0] + 4 * steps[1] - steps[2]) / 2,
        steps[0],
    ]
}

fn main() {
    let input: Grid = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let start = input
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .position(|c| c == &'S')
                .map(|x| (y as i32, x as i32))
        })
        .unwrap();
    println!(
        "(1) After 64 steps {} positions are reachable",
        walk(&input, &[64], start)[0]
    );

    // 26501365 = 202300 * 131 + 65
    let n = 202300;
    let steps = [65, 196, 327];
    let distances = walk(&input, &steps, start)
        .iter()
        .map(|&d| d as i64)
        .collect::<Vec<_>>();
    let polynomial_factors = lagrange_interpolation(&distances[0..3]);

    println!(
        "(2) After 26501365 steps {} positions are reachable",
        polynomial_factors[0] * n * n + polynomial_factors[1] * n + polynomial_factors[2]
    );
}

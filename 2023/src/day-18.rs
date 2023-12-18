use itertools::Itertools;
use std::collections::HashMap;
use std::io::{self, BufRead};

type Grid = HashMap<i64, HashMap<i64, bool>>;
type Position = (i64, i64);

fn is_border(grid: &Grid, pos: Position) -> bool {
    *grid
        .get(&pos.0)
        .map(|row| row.get(&pos.1).unwrap_or(&false))
        .unwrap_or(&false)
}

fn intersection_fill(grid: &mut Grid) -> usize {
    grid.iter()
        .map(|(&row_idx, row)| {
            let mut intersections = row.keys().copied().collect::<Vec<_>>();
            intersections.sort();

            let mut paint_start = None;
            let mut painted = intersections.len();
            let mut num_intersections = 0;

            let mut iter = intersections.iter();
            while let Some(&(mut point)) = iter.next() {
                if let Some(prev_point) = paint_start {
                    painted += (point - prev_point - 1) as usize;
                    paint_start = None;
                }

                let start = point;
                while is_border(grid, (row_idx, point + 1)) {
                    iter.next();
                    point += 1;
                }

                if start == point
                    || is_border(grid, (row_idx - 1, start))
                        != is_border(grid, (row_idx - 1, point))
                {
                    num_intersections += 1;
                }

                // Use the even-odd rule to determine which points are painted
                if num_intersections % 2 == 1 {
                    paint_start = Some(point);
                }
            }
            assert!(num_intersections % 2 == 0);
            painted
        })
        .sum::<usize>()
}

fn calc_area(vertices: &Vec<Position>) -> i64 {
    let mut area = 0;
    let mut perimeter = 0.0;

    // Shoelace formula
    for (first, second) in vertices.iter().tuple_windows() {
        area += (first.1 * second.0) - (first.0 * second.1);
        let dx = second.1 - first.1;
        let dy = second.0 - first.0;
        perimeter += f64::sqrt(((dx * dx) + (dy * dy)) as f64);
    }
    let area = i64::abs(area) / 2;

    // Pick's theorem
    let interior = area - (perimeter as i64 / 2) + 1;
    interior + perimeter as i64
}

fn main() {
    let input = io::stdin().lock().lines().flatten().collect::<Vec<_>>();

    let mut grid: Grid = HashMap::new();
    let mut curr = (0, 0);

    for line in &input {
        if let [direction, count, _] = &line.split(' ').collect::<Vec<_>>()[..] {
            let direction = direction.chars().next().unwrap();
            let count = count.parse::<i32>().unwrap();

            for _ in 0..count {
                let next = match direction {
                    'L' => (curr.0, curr.1 - 1),
                    'R' => (curr.0, curr.1 + 1),
                    'U' => (curr.0 - 1, curr.1),
                    'D' => (curr.0 + 1, curr.1),
                    _ => panic!("Invalid direction"),
                };

                grid.entry(next.0).or_default().insert(next.1, true);
                curr = next;
            }
        }
    }
    println!(
        "(1) Area filled (using intersection fill): {}",
        intersection_fill(&mut grid)
    );

    curr = (0, 0);
    let mut vertices = vec![curr];

    for line in &input {
        if let [_, _, color] = &line.split(' ').collect::<Vec<_>>()[..] {
            let count = i64::from_str_radix(&color[2..color.len() - 2], 16).unwrap();
            let direction = color.chars().nth_back(1).unwrap();

            curr = match direction {
                '0' => (curr.0, curr.1 + count),
                '1' => (curr.0 + count, curr.1),
                '2' => (curr.0, curr.1 - count),
                '3' => (curr.0 - count, curr.1),
                _ => panic!("Invalid direction"),
            };
            vertices.push(curr);
        }
    }
    println!(
        "(2) Area filled (using shoelace formula): {}",
        calc_area(&vertices)
    );
}

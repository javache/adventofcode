use std::collections::HashMap;
use std::io::{self, BufRead};

type Grid = HashMap<Point, u32>;
type Point = (i32, i32);

fn walk_x(grid: &mut Grid, idx: u32, start_pos: Point, distance: i32) -> Point {
    (1..=distance.abs())
        .map(|dx| dx * distance.signum())
        .for_each(|dx| {
            let pos = (start_pos.0 + dx, start_pos.1);
            grid.entry(pos).or_insert(idx + dx.abs() as u32);
        });
    (start_pos.0 + distance, start_pos.1)
}

fn walk_y(grid: &mut Grid, idx: u32, start_pos: Point, distance: i32) -> Point {
    (1..=distance.abs())
        .map(|dy| dy * distance.signum())
        .for_each(|dy| {
            let pos = (start_pos.0, start_pos.1 + dy);
            grid.entry(pos).or_insert(idx + dy.abs() as u32);
        });
    (start_pos.0, start_pos.1 + distance)
}

fn manhattan_distance(point: &Point) -> i32 {
    point.0.abs() + point.1.abs()
}

fn main() -> io::Result<()> {
    let mut grids: Vec<Grid> = vec![];
    io::stdin().lock().lines().flatten().for_each(|line| {
        let mut grid: Grid = HashMap::new();
        let mut pos = (0, 0);
        let mut idx = 0;
        line.split(',').for_each(|step| {
            let distance: i32 = step[1..].parse().unwrap();
            pos = match &step[..1] {
                "L" => walk_x(&mut grid, idx, pos, -distance),
                "R" => walk_x(&mut grid, idx, pos, distance),
                "U" => walk_y(&mut grid, idx, pos, -distance),
                "D" => walk_y(&mut grid, idx, pos, distance),
                _ => panic!("Unexpected input {}", step),
            };
            idx += distance as u32;
        });
        grids.push(grid);
    });

    // Intersect grids
    let mut crossings: Vec<Point> = grids[0]
        .keys()
        .filter(|point| grids[1].contains_key(point))
        .cloned()
        .collect();

    crossings.sort_by(|a, b| manhattan_distance(a).cmp(&manhattan_distance(b)));
    println!(
        "(1) The closest crossing on the grid is at {:?} = {}",
        crossings[0],
        manhattan_distance(&crossings[0])
    );

    crossings.sort_by(|a, b| (grids[0][a] + grids[1][a]).cmp(&(grids[0][b] + grids[1][b])));
    println!(
        "(2) The closest crossing in wire distance is at {:?} = {}",
        crossings[0],
        grids[0][&crossings[0]] + grids[1][&crossings[0]]
    );

    Ok(())
}

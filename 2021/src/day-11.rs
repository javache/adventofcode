use itertools::iproduct;
use std::io::{self, BufRead};

type Grid = Vec<Vec<u8>>;
type Point = (usize, usize);

fn run_iteration(grid: &mut Grid) -> usize {
    iproduct!(0..grid.len(), 0..grid[0].len()).for_each(|(x,y)| {
        grid[x][y] += 1;
    });

    let mut total_flashes = 0;
    loop {
        let flashing: Vec<Point> = grid
            .iter()
            .enumerate()
            .flat_map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(y, val)| (*val > 9).then(|| (x, y)))
                    .collect::<Vec<Point>>()
            })
            .collect();
        flashing.iter().for_each(|p| spread_flash(grid, p));
        total_flashes += flashing.len();

        if flashing.len() == 0 {
            break;
        }
    }

    total_flashes
}

fn spread_flash(grid: &mut Grid, point: &Point) {
    grid[point.0][point.1] = 0;        
    iproduct!([-1, 0, 1], [-1, 0, 1]).for_each(|(dx, dy)| {
        let neighbour = (point.0 as i32 + dx, point.1 as i32 + dy);
        if (dx, dy) != (0, 0)
            && (0..grid.len() as i32).contains(&neighbour.0)
            && (0..grid[0].len() as i32).contains(&neighbour.1)
        {
            let value = &mut grid[neighbour.0 as usize][neighbour.1 as usize];
            if *value > 0 {
                *value += 1;
            }
        }
    });
}

fn main() -> io::Result<()> {
    let mut grid: Grid = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| line.chars().map(|c| (c as u8) - b'0').collect())
        .collect();

    let mut total_flashes = 0;
    let mut synchronized_step = 0;

    for step in 0.. {
        let flashes = run_iteration(&mut grid);
        if step < 100 {
            total_flashes += flashes;
        }
        if grid.iter().all(|row| row.iter().all(|val| *val == 0)) {
            synchronized_step = step + 1;
            break;
        }
    }

    println!(
        "(1) After 100 iterations there were {} flashes",
        total_flashes
    );
    println!(
        "(2) The octupuses flashed synchronusly in step {}",
        synchronized_step
    );

    Ok(())
}

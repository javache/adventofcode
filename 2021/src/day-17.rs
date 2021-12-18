use itertools::iproduct;
use regex::Regex;
use std::io::{self, BufRead};
use std::ops::RangeInclusive;

type Point = (i32, i32);
type Target = (RangeInclusive<i32>, RangeInclusive<i32>);

fn is_valid_solution(mut velocity: Point, target: &Target) -> bool {
    let mut position = (0, 0);
    while &position.0 <= target.0.end() && &position.1 >= target.1.start() {
        position = (position.0 + velocity.0, position.1 + velocity.1);
        velocity = (
            velocity.0.signum() * (velocity.0.abs() - 1).max(0),
            velocity.1 - 1,
        );
        if target.0.contains(&position.0) && target.1.contains(&position.1) {
            return true;
        }
    }
    false
}

fn main() -> io::Result<()> {
    let mut line: String = String::new();
    io::stdin().lock().read_line(&mut line)?;

    let input_re = Regex::new(r"^target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)$").unwrap();
    let solutions = input_re
        .captures(&line)
        .map(|captures| {
            if let [min_x, max_x, min_y, max_y] =
                [&captures[1], &captures[2], &captures[3], &captures[4]]
                    .into_iter()
                    .flat_map(str::parse)
                    .collect::<Vec<i32>>()[..]
            {
                let target = (min_x..=max_x, min_y..=max_y);
                iproduct!(0..=max_x, min_y..200)
                    .filter(|v| is_valid_solution(*v, &target))
                    .collect::<Vec<Point>>()
            } else {
                vec![]
            }
        })
        .unwrap();

    if let Some(max_solution) = solutions.iter().max_by_key(|(_, y)| *y) {
        println!(
            "(1) Solution with max Y is {:?} at {}",
            max_solution,
            max_solution.1 * (1 + max_solution.1) / 2
        );
    }
    println!("(2) There are {} distinct solutions", solutions.len());

    Ok(())
}

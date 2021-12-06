use itertools::Itertools;
use std::collections::HashMap;
use std::io::{self, BufRead};

type Point = (i32, i32);
type Line = (Point, Point);

struct LineIterator {
    next: Option<Point>,
    end: Point,
    delta: (i32, i32),
}

impl LineIterator {
    fn new(start: Point, end: Point, allow_diagonal: bool) -> LineIterator {
        if allow_diagonal || start.0 == end.0 || start.1 == end.1 {
            LineIterator {
                next: Some(start),
                end,
                delta: (i32::signum(end.0 - start.0), i32::signum(end.1 - start.1)),
            }
        } else {
            LineIterator {
                next: None,
                end,
                delta: (0, 0),
            }
        }
    }
}

impl Iterator for LineIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.next;
        if let Some(result) = result {
            if result == self.end {
                self.next = None
            } else {
                self.next = Some((result.0 + self.delta.0, result.1 + self.delta.1));
            }
        }
        result
    }
}

fn parse_line(input: &str) -> Option<Line> {
    input
        .split(" -> ")
        .filter_map(|p| {
            if let [x, y] = input.split(',').collect::<Vec<&str>>()[..] {
                Some((x.parse().ok()?, y.parse().ok()?))
            } else {
                None
            }
        })
        .collect_tuple()
}

fn count_overlapping(lines: &Vec<Line>, allow_diagonal: bool) -> usize {
    let mut grid = HashMap::<Point, u8>::new();
    for line in lines.iter() {
        for (x, y) in LineIterator::new(line.0, line.1, allow_diagonal) {
            let count = grid.entry((x, y)).or_insert(0);
            if *count < u8::MAX {
                *count += 1;
            }
        }
    }
    grid.iter().filter(|(_, count)| **count > 1).count()
}

fn main() -> io::Result<()> {
    let lines: Vec<Line> = io::stdin()
        .lock()
        .lines()
        .flatten()
        .filter_map(|line| parse_line(&line))
        .collect();

    println!(
        "(1) There are {} points where lines overlap horizontally/vertically",
        count_overlapping(&lines, false)
    );
    println!(
        "(2) There are {} points where lines overlap in all directions",
        count_overlapping(&lines, true)
    );

    Ok(())
}

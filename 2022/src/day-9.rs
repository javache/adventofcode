use itertools::Itertools;
use std::collections::HashSet;
use std::io::{self, BufRead};

type Point = (i32, i32);

fn is_neighbour(point: &Point, reference: &Point) -> bool {
    ((reference.0 - 1)..=(reference.0 + 1)).contains(&point.0)
        && ((reference.1 - 1)..=(reference.1 + 1)).contains(&point.1)
}

fn simulate_rope(steps: &Vec<(char, usize)>, rope_length: usize) -> usize {
    let mut rope: Vec<Point> = vec![(0, 0); rope_length];
    let mut tail_positions: HashSet<Point> = HashSet::from([(0, 0)]);

    for (direction, n) in steps {
        for _ in 0..*n {
            let head = &mut rope[0];
            *head = match direction {
                'U' => (head.0, head.1 - 1),
                'D' => (head.0, head.1 + 1),
                'L' => (head.0 - 1, head.1),
                'R' => (head.0 + 1, head.1),
                _ => panic!("Unexpected char {}", direction),
            };
            for i in 1..rope_length {
                let prev = &rope[i - 1];
                let curr = &rope[i];
                if !is_neighbour(&curr, &prev) {
                    rope[i] = (
                        curr.0 + i32::signum(prev.0 - curr.0),
                        curr.1 + i32::signum(prev.1 - curr.1),
                    );
                }
            }
            tail_positions.insert(rope[rope_length - 1]);
        }
    }
    tail_positions.len()
}

fn main() {
    let input: Vec<(char, usize)> = io::stdin()
        .lock()
        .lines()
        .flatten()
        .flat_map(|line| {
            line.split(' ')
                .collect_tuple()
                .map(|(c, n)| Some((c.chars().next()?, n.parse().ok()?)))
                .flatten()
        })
        .collect();

    println!(
        "(1) The tail had {} different positions",
        simulate_rope(&input, 2)
    );
    println!(
        "(2) The tail had {} different positions",
        simulate_rope(&input, 10)
    );
}

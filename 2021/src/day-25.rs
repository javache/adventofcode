use itertools::iproduct;
use std::io::{self, BufRead};

#[derive(Debug, Copy, Clone, PartialEq)]
enum Node {
    East,
    South,
    Empty,
}
type Grid = Vec<Vec<Node>>;

fn step(input: &Grid) -> Grid {
    let mut output = input.clone();
    for (x, y) in iproduct!(0..input[0].len(), 0..input.len()) {
        if matches!(input[y][x], Node::East) {
            let dest = ((x + 1) % input[0].len(), y);
            if matches!(input[dest.1][dest.0], Node::Empty) {
                output[dest.1][dest.0] = Node::East;
                output[y][x] = Node::Empty;
            }
        }
    }
    let output_snapshot = output.clone();
    for (x, y) in iproduct!(0..input[0].len(), 0..input.len()) {
        if matches!(input[y][x], Node::South) {
            let dest = (x, (y + 1) % input.len());
            if matches!(output_snapshot[dest.1][dest.0], Node::Empty) {
                output[dest.1][dest.0] = Node::South;
                output[y][x] = Node::Empty;
            }
        }
    }
    output
}

fn main() -> io::Result<()> {
    let input: Grid = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '>' => Node::East,
                    'v' => Node::South,
                    _ => Node::Empty,
                })
                .collect()
        })
        .collect();

    let mut state = input;
    for i in 1.. {
        let next_state = step(&state);
        if state == next_state {
            println!("(1) The sea cucumbers stopped moving in step {}", i);
            break;
        }
        state = next_state;
    }

    Ok(())
}

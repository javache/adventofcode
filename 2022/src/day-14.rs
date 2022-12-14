use itertools::Itertools;
use std::io::{self, BufRead};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum State {
    Free,
    Rock,
    Sand,
}

type Grid = Vec<Vec<State>>;
type Point = (usize, usize);
type Segment = Vec<Point>;

fn coord_range(start: usize, end: usize) -> impl Iterator<Item = usize> + Clone {
    if start < end {
        start..=end
    } else {
        end..=start
    }
}

fn insert_rock_segments(state: &mut Grid, segments: &Vec<Segment>) {
    for segment in segments {
        for (start, end) in segment.iter().tuple_windows() {
            for point in coord_range(start.0, end.0).cartesian_product(coord_range(start.1, end.1))
            {
                state[point.1][point.0] = State::Rock;
            }
        }
    }
}

const SAND_START: Point = (500, 0);
const DIRECTIONS: &'static [(i32, i32)] = &[(0, 1), (-1, 1), (1, 1)];

fn simulate_sand(state: &mut Grid) -> usize {
    for turn in 1.. {
        let mut pos = SAND_START;
        loop {
            let mut next_pos = None;
            for (dx, dy) in DIRECTIONS {
                let candidate = ((pos.0 as i32 + dx) as usize, (pos.1 as i32 + dy) as usize);
                if candidate.1 >= state.len() {
                    return turn - 1;
                }
                if state[candidate.1][candidate.0] == State::Free {
                    next_pos = Some(candidate);
                    break;
                }
            }
            if let Some(next_pos) = next_pos {
                pos = next_pos;
            } else {
                if pos == SAND_START {
                    return turn;
                } else {
                    state[pos.1][pos.0] = State::Sand;
                    break;
                }
            }
        }
    }
    panic!("Unreachable");
}

fn main() -> io::Result<()> {
    let input: Vec<Segment> = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    point
                        .split(',')
                        .flat_map(str::parse)
                        .collect_tuple()
                        .unwrap()
                })
                .collect()
        })
        .collect();

    let (max_x, max_y) = input.iter().fold((0, 0), |acc, segment| {
        let segment_max = segment
            .iter()
            .fold((0, 0), |acc, p| (acc.0.max(p.0), acc.1.max(p.1)));
        (acc.0.max(segment_max.0), acc.1.max(segment_max.1))
    });

    let mut state: Grid = vec![vec![State::Free; 3 * max_x / 2]; max_y + 1];
    insert_rock_segments(&mut state, &input);

    let turns = simulate_sand(&mut state.clone());
    println!("(1) Sand started freefalling at turn {}", turns);

    state.push(vec![State::Free; state[0].len()]);
    state.push(vec![State::Rock; state[0].len()]);
    let turns = simulate_sand(&mut state.clone());
    println!("(2) Source was blocked at turn {}", turns);

    Ok(())
}

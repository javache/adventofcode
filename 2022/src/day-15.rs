use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use std::io::{self, BufRead};
use std::ops::RangeInclusive;

type Point = (i32, i32);
type Input = Vec<(Point, Point)>;
type State = Vec<Vec<RangeInclusive<i32>>>;

static INSTRUCTION_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$")
        .unwrap()
});

fn manhattan_distance(sensor: &Point, beacon: &Point) -> i32 {
    (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs()
}

fn union_of_ranges(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> RangeInclusive<i32> {
    *(a.start().min(b.start()))..=*(a.end().max(b.end()))
}

fn merge_reading(state: &mut State, y: i32, mut x: RangeInclusive<i32>) {
    // Ignore values out of bounds for our later search
    if y < 0 || y as usize >= state.len() {
        return;
    }

    let ranges = &mut state[y as usize];

    let mut insertion_idx = None;
    let mut indices_to_remove = 0..0;
    for (i, range) in ranges.iter().enumerate() {
        // Range already fully contained, nothing to do here
        if range.start() <= x.start() && range.end() >= x.end() {
            return;
        }

        // Partial overlap, merge ranges
        if range.start() <= x.end() && (range.end() + 1) >= *x.start() {
            x = union_of_ranges(range, &x);
            if insertion_idx == None {
                insertion_idx = Some(i);
                indices_to_remove = i..(i + 1);
            } else {
                indices_to_remove = indices_to_remove.start..(i + 1);
            }
        // Completed overlap, can break now
        } else if insertion_idx != None {
            break;
        // Found insertion point
        } else if x.end() < range.start() {
            insertion_idx = Some(i);
            break;
        }
    }

    if let Some(insertion_idx) = insertion_idx {
        if indices_to_remove.len() == 1 {
            ranges[insertion_idx] = x;
        } else {
            ranges.drain(indices_to_remove);
            ranges.insert(insertion_idx, x);
        }
    } else {
        ranges.push(x);
    }
}

fn sum_ranges_length(input: &Vec<RangeInclusive<i32>>) -> usize {
    input
        .iter()
        .map(|range| (range.end() - range.start()))
        .sum::<i32>() as usize
}

fn find_empty_position_in_range(
    state: &State,
    search_space: &RangeInclusive<i32>,
) -> Option<Point> {
    for y in search_space.clone() {
        let mut search_space_x = search_space.clone();
        for range in &state[y as usize] {
            if range.start() <= search_space_x.start() && range.end() >= search_space_x.end() {
                break;
            }
            if range.start() - 1 == *search_space_x.start() {
                return Some((*search_space_x.start(), y));
            } else {
                search_space_x = (range.end() + 1)..=*search_space_x.end();
            }
        }
    }
    None
}

fn main() -> io::Result<()> {
    let input: Input = io::stdin()
        .lock()
        .lines()
        .flatten()
        .flat_map(|line| {
            INSTRUCTION_RE
                .captures(&line)
                .unwrap()
                .iter()
                .flat_map(|n| n.unwrap().as_str().parse())
                .tuples::<(_, _)>()
                .collect_tuple::<(_, _)>()
        })
        .collect();

    let mut state: State = vec![vec![]; 4_000_000];
    for (sensor, beacon) in input {
        let distance = manhattan_distance(&sensor, &beacon);
        for y in (-distance)..=distance {
            let width = distance - y.abs();
            merge_reading(
                &mut state,
                sensor.1 + y,
                (sensor.0 - width)..=(sensor.0 + width),
            );
        }
    }

    let output_reading = 2_000_000;
    println!(
        "(1) There are {} observed positions on row {}",
        sum_ranges_length(&state[output_reading]),
        output_reading
    );

    let search_space = 0..=4_000_000;
    if let Some((x, y)) = find_empty_position_in_range(&state, &search_space) {
        println!(
            "(2) Found match at {:?} => {}",
            (x, y),
            x as usize * 4_000_000 + y as usize
        );
    }

    Ok(())
}

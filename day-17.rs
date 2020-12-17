use itertools::Itertools;
use std::io::{self, Read};
use std::iter;
use std::ops::Range;

#[derive(Clone)]
struct State {
    dimensions: Vec<Range<i32>>,
    data: Vec<bool>,
}

impl State {
    fn new(input: Vec<Vec<bool>>) -> State {
        State {
            dimensions: vec![0..(input[0].len() as i32), 0..(input.len() as i32)],
            data: input.into_iter().flatten().collect(),
        }
    }

    fn new_with_state(prev_state: &State) -> State {
        let extended_dimensions: Vec<Range<i32>> = prev_state
            .get_minimal_dimensions()
            .iter()
            .map(|r| (r.start - 1)..(r.end + 1))
            .collect();
        let extended_len = extended_dimensions
            .iter()
            .map(|r| (r.end - r.start) as usize)
            .product();
        State {
            dimensions: extended_dimensions,
            data: vec![false; extended_len],
        }
    }

    fn set_dimensionality(&mut self, dimensions: usize) {
        self.dimensions.reserve(dimensions);
        for _ in self.dimensions.len()..dimensions {
            self.dimensions.push(0..1);
        }
    }

    fn get_minimal_dimensions(&self) -> Vec<Range<i32>> {
        let mut min_max: Option<Vec<Range<i32>>> = None;
        for point in self
            .dimensions
            .clone()
            .into_iter()
            .multi_cartesian_product()
        {
            if self.get(&point) {
                if let Some(curr_min_max) = &min_max {
                    min_max = Some(
                        curr_min_max
                            .iter()
                            .zip(point)
                            .map(|(range, c)| (range.start.min(c)..range.end.max(c + 1)))
                            .collect(),
                    );
                } else {
                    min_max = Some(point.iter().map(|&c| c..(c + 1)).collect());
                }
            }
        }
        min_max.unwrap()
    }

    fn get_offset(&self, point: &Vec<i32>) -> usize {
        assert!(point.len() == self.dimensions.len());
        let mut multiplier = 1;
        let mut offset = 0;
        for i in 0..point.len() {
            offset += (point[i] - self.dimensions[i].start) * multiplier;
            multiplier *= self.dimensions[i].end - self.dimensions[i].start;
        }
        offset as usize
    }

    fn get(&self, point: &Vec<i32>) -> bool {
        if point
            .iter()
            .enumerate()
            .all(|(idx, c)| self.dimensions[idx].contains(c))
        {
            self.data[self.get_offset(point)]
        } else {
            false
        }
    }

    fn set(&mut self, point: &Vec<i32>, value: bool) -> () {
        let offset = self.get_offset(point);
        self.data[offset] = value;
    }

    fn step(&self) -> State {
        let mut next = State::new_with_state(&self);
        for point in next
            .dimensions
            .clone()
            .into_iter()
            .multi_cartesian_product()
        {
            let neighbours = self.count_active_neighbours(&point);
            if self.get(&point) {
                next.set(&point, neighbours == 2 || neighbours == 3);
            } else {
                next.set(&point, neighbours == 3);
            }
        }
        next
    }

    fn count_active_neighbours(&self, point: &Vec<i32>) -> usize {
        iter::repeat(vec![-1, 0, 1])
            .take(point.len())
            .multi_cartesian_product()
            .map(|delta| {
                if delta == vec![0; point.len()] {
                    0
                } else {
                    let neighbour: Vec<i32> = point.iter().zip(delta).map(|(a, b)| a + b).collect();
                    self.get(&neighbour) as usize
                }
            })
            .sum()
    }

    fn count_active(&self) -> usize {
        self.data.iter().map(|b| *b as usize).sum()
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;

    let initial_state = State::new(
        input
            .split("\n")
            .map(|l| l.chars().map(|c| c == '#').collect())
            .collect(),
    );

    let mut state = initial_state.clone();
    state.set_dimensionality(3);
    for _ in 0..6 {
        state = state.step()
    }
    println!(
        "(1) After 6 steps in 3 dimensions, there's {} cells",
        state.count_active()
    );

    let mut state = initial_state.clone();
    state.set_dimensionality(4);
    for _ in 0..6 {
        state = state.step();
    }
    println!(
        "(2) After 6 steps in 4 dimensions, there's {} cells",
        state.count_active()
    );

    Ok(())
}

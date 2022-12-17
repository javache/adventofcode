use std::io::{self, Read};

type Chamber = Vec<u8>;

const CHAMBER_WIDTH: i8 = 7;

#[derive(Clone, PartialEq, Eq)]
struct Shape {
    elems: [u8; 4],
    width: i8,
}

fn get_shapes() -> Vec<Shape> {
    vec![
        Shape {
            elems: [0b1111, 0b0, 0b0, 0b0],
            width: 4,
        },
        Shape {
            elems: [0b010, 0b111, 0b010, 0b0],
            width: 3,
        },
        Shape {
            elems: [0b111, 0b1, 0b1, 0b0],
            width: 3,
        },
        Shape {
            elems: [0b1, 0b1, 0b1, 0b1],
            width: 1,
        },
        Shape {
            elems: [0b11, 0b11, 0b0, 0b0],
            width: 2,
        },
    ]
}

fn find_first_empty_row(chamber: &Chamber) -> usize {
    chamber.len()
        - chamber
            .iter()
            .rev()
            .position(|row| *row > 0)
            .map(|pos| pos)
            .unwrap_or(chamber.len())
}

impl Shape {
    fn get_initial_position(self: &Self, chamber: &mut Chamber) -> (i8, isize) {
        let max_y = find_first_empty_row(&chamber) + 3 + self.elems.len();
        if max_y > chamber.len() {
            chamber.append(&mut vec![0; max_y - chamber.len()]);
        }
        (
            CHAMBER_WIDTH - self.width - 2,
            (max_y - self.elems.len()) as isize,
        )
    }

    fn overlaps_chamber(self: &Self, position: (i8, isize), chamber: &Chamber) -> bool {
        self.elems
            .iter()
            .enumerate()
            .any(|(idx, row)| chamber[idx + position.1 as usize] & (row << position.0) != 0)
    }

    fn add_to_chamber(
        self: &Self,
        chamber: &mut Chamber,
        instructions: &mut impl Iterator<Item = u8>,
    ) {
        let mut position = self.get_initial_position(chamber);
        loop {
            let next_pos_x = (match instructions.next() {
                Some(b'>') => -1,
                _ => 1,
            } + position.0)
                .clamp(0, CHAMBER_WIDTH - self.width);
            if !self.overlaps_chamber((next_pos_x, position.1), chamber) {
                position.0 = next_pos_x;
            }

            if position.1 > 0 && !self.overlaps_chamber((position.0, position.1 - 1), chamber) {
                position.1 -= 1;
            } else {
                break;
            }
        }

        for (idx, row) in self.elems.iter().enumerate() {
            chamber[idx + position.1 as usize] |= row << position.0;
        }
    }
}

fn main() {
    let pattern: Vec<u8> = io::stdin().lock().bytes().flatten().collect();

    let mut chamber = vec![0; 4];
    let mut shape_it = get_shapes().into_iter().cycle();
    let mut pattern_it = pattern.into_iter().cycle().clone();

    let mut heights = vec![];
    let mut last_height = 0;
    for count in 1usize..=10_000 {
        let shape = shape_it.next().unwrap();
        shape.add_to_chamber(&mut chamber, &mut pattern_it);

        let height = find_first_empty_row(&chamber);
        heights.push(height - last_height);
        last_height = height;

        if count == 2022 {
            println!("(1) Tower is {} rows high after block 2022", height);
        }
    }

    let target: usize = 1_000_000_000_000;
    for pattern_len in 4..(heights.len() / 2) {
        if heights
            .iter()
            .rev()
            .zip(heights.iter().rev().skip(pattern_len))
            .take(pattern_len)
            .all(|(row_a, row_b)| row_a == row_b)
        {
            println!("Pattern found of length {}", pattern_len);
            let height_increase: usize = heights.iter().rev().take(pattern_len).sum();
            let repeating = (target - 10_000) as usize / pattern_len;
            let remainder = (target - 10_000) as usize % pattern_len;

            for _ in 0..remainder {
                let shape = shape_it.next().unwrap();
                shape.add_to_chamber(&mut chamber, &mut pattern_it);
            }

            let height = find_first_empty_row(&chamber);
            println!(
                "(2) Tower is {} rows high after block {}",
                height + repeating * height_increase,
                repeating * pattern_len + 10_000 + remainder
            );
            break;
        }
    }
}

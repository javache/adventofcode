use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use std::collections::HashMap;
use std::io::{self, BufRead};

type State = HashMap<(i32, i32), bool>;

// Using axial coordinates (https://www.redblobgames.com/grids/hexagons/)
fn follow_directions(input: &str) -> (i32, i32) {
    let mut pos = (0, 0);
    let mut input_it = input.chars();
    while let Some(c) = input_it.next() {
        pos = match c {
            'e' => (pos.0 + 1, pos.1),
            'w' => (pos.0 - 1, pos.1),
            'n' => {
                if input_it.next() == Some('e') {
                    (pos.0 + 1, pos.1 - 1)
                } else {
                    (pos.0, pos.1 - 1)
                }
            }
            's' => {
                if input_it.next() == Some('e') {
                    (pos.0, pos.1 + 1)
                } else {
                    (pos.0 - 1, pos.1 + 1)
                }
            }
            _ => unreachable!(),
        }
    }
    pos
}

const HEXAGON_NEIGBHOURS: &'static [(i32, i32)] =
    &[(1, 0), (1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1)];

fn neighbour_count(state: &State, pos: &(i32, i32)) -> usize {
    HEXAGON_NEIGBHOURS
        .iter()
        .filter(|delta| {
            *state
                .get(&(pos.0 + delta.0, pos.1 + delta.1))
                .unwrap_or(&false)
        })
        .count()
}

fn flip_tiles(state: &State) -> State {
    let mut output = HashMap::new();
    let bounds = (
        state.keys().map(|c| c.0).minmax(),
        state.keys().map(|c| c.1).minmax(),
    );
    if let (MinMax(min_q, max_q), MinMax(min_r, max_r)) = bounds {
        for pos in ((min_q - 1)..=(max_q + 1)).cartesian_product((min_r - 1)..=(max_r + 1)) {
            let count = neighbour_count(state, &pos);
            let tile_state = if *state.get(&pos).unwrap_or(&false) {
                count != 0 && count <= 2
            } else {
                count == 2
            };
            if tile_state {
                output.insert(pos, true);
            }
        }
    }
    output
}

fn main() -> io::Result<()> {
    assert!(follow_directions("nwwswee") == (0, 0));

    let mut state: State = HashMap::new();
    for line in io::stdin().lock().lines() {
        let tile = follow_directions(&line?);
        state.entry(tile).and_modify(|v| *v = !*v).or_insert(true);
    }
    let black_count = state.values().filter(|&v| *v).count();
    println!("(1) There are {} black tiles", black_count);

    for _ in 0..100 {
        state = flip_tiles(&state);
    }
    let black_count = state.values().filter(|&v| *v).count();
    println!("(2) There are {} black tiles", black_count);

    Ok(())
}

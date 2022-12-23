use itertools::{iproduct, Itertools};
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, BufRead};

type Point = (i32, i32);
type Grid = HashSet<Point>;

fn has_neighbours(elf: &Point, state: &Grid) -> bool {
    iproduct!(-1..=1, -1..=1)
        .any(|(dx, dy)| (dx != 0 || dy != 0) && state.contains(&(elf.0 + dx, elf.1 + dy)))
}

fn is_direction_valid(elf: &Point, dir: &Point, state: &Grid) -> bool {
    if dir.0 == 0 {
        (-1..=1).all(|dx| !state.contains(&(elf.0 + dx, elf.1 + dir.1)))
    } else {
        (-1..=1).all(|dy| !state.contains(&(elf.0 + dir.0, elf.1 + dy)))
    }
}

fn count_empty_tiles(grid: &Grid) -> usize {
    let bounds = [
        grid.iter().map(|&(x, _)| x).minmax(),
        grid.iter().map(|&(_, y)| y).minmax(),
    ]
    .map(|minmax| minmax.into_option().unwrap());

    iproduct!(bounds[0].0..=bounds[0].1, bounds[1].0..=bounds[1].1)
        .filter(|point| !grid.contains(&point))
        .count()
}

fn move_elves(mut state: Grid) {
    let mut dirs = VecDeque::from([(0, -1), (0, 1), (-1, 0), (1, 0)]);
    for i in 1.. {
        let mut targets: HashMap<Point, Vec<Point>> = HashMap::new();
        for elf in &state {
            if !has_neighbours(elf, &state) {
                continue;
            }

            if let Some(target) = dirs.iter().find_map(|dir| {
                is_direction_valid(elf, dir, &state).then(|| (elf.0 + dir.0, elf.1 + dir.1))
            }) {
                targets.entry(target).or_default().push(*elf);
            }
        }

        if targets.is_empty() {
            println!("(2) No elf moved in round {}", i);
            break;
        }

        for (target, elves) in targets {
            if let [elf] = &elves[..] {
                state.remove(&elf);
                state.insert(target);
            }
        }
        dirs.rotate_left(1);

        if i == 10 {
            println!(
                "(1) There are {} empty ground tiles at the end of round 10",
                count_empty_tiles(&state)
            );
        }
    }
}

fn main() {
    let input: Grid = io::stdin()
        .lock()
        .lines()
        .flatten()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(|(x, _)| (x as i32, y as i32))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    // print_grid(&input);
    move_elves(input);
}

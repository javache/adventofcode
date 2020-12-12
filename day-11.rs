use std::io::{self, Read};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Position {
    Floor,
    Empty,
    Occupied,
}

type State = Vec<Vec<Position>>;

fn _print_state(state: &State) {
    let desc = state
        .iter()
        .map(|row| {
            row.iter()
                .map(|pos| match pos {
                    Position::Floor => '.',
                    Position::Empty => 'L',
                    Position::Occupied => '#',
                })
                .collect()
        })
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", desc);
}

const NEIGHBOURS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn get_seat(state: &State, pos: (i32, i32)) -> Option<Position> {
    if pos.0 >= 0 && pos.0 < state.len() as i32 && pos.1 >= 0 && pos.1 < state[0].len() as i32 {
        Some(state[pos.0 as usize][pos.1 as usize])
    } else {
        None
    }
}

fn count_adjacent_occupied(state: &State, base: (i32, i32)) -> usize {
    NEIGHBOURS
        .iter()
        .map(|dir| {
            let pos = (base.0 + dir.0, base.1 + dir.1);
            (get_seat(state, pos) == Some(Position::Occupied)) as usize
        })
        .sum()
}

fn find_first_seat(state: &State, mut pos: (i32, i32), dir: &(i32, i32)) -> usize {
    loop {
        pos = (pos.0 + dir.0, pos.1 + dir.1);
        match get_seat(state, pos) {
            Some(Position::Occupied) => {
                return 1;
            }
            Some(Position::Empty) => {
                return 0;
            }
            Some(Position::Floor) => {}
            None => {
                return 0;
            }
        }
    }
}

fn count_visible_occupied(state: &State, pos: (i32, i32)) -> usize {
    NEIGHBOURS
        .iter()
        .map(|dir| find_first_seat(state, pos, dir))
        .sum()
}

fn step(
    current: &State,
    count_method: fn(&State, (i32, i32)) -> usize,
    max_occupied_to_free: usize,
) -> State {
    let mut next = current.clone();
    for i in 0..current.len() {
        for j in 0..current[i].len() {
            let occupied = count_method(current, (i as i32, j as i32));
            next[i][j] = match current[i][j] {
                Position::Empty if occupied == 0 => Position::Occupied,
                Position::Occupied if occupied >= max_occupied_to_free => Position::Empty,
                _ => current[i][j],
            }
        }
    }
    next
}

fn count_occupied_seats(state: &State) -> usize {
    state
        .iter()
        .map(|row| {
            row.iter()
                .filter(|&seat| *seat == Position::Occupied)
                .count()
        })
        .sum()
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();

    let initial_state: State = input
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    'L' => Position::Empty,
                    '#' => Position::Occupied,
                    '.' => Position::Floor,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let mut state = initial_state.clone();
    let mut prev_state: State = Vec::new();
    while state != prev_state {
        prev_state = state;
        state = step(&prev_state, count_adjacent_occupied, 4);
    }
    println!(
        "(1) There are {} occupied seats in the steady state",
        count_occupied_seats(&state)
    );

    let mut state = initial_state;
    let mut prev_state: State = Vec::new();
    while state != prev_state {
        prev_state = state;
        state = step(&prev_state, count_visible_occupied, 5);
    }
    println!(
        "(2) There are {} occupied seats in the steady state",
        count_occupied_seats(&state)
    );

    Ok(())
}

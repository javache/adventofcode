use std::io::{self, Read};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Position {
    Floor,
    Empty,
    Occupied,
}

type State = Vec<Vec<Position>>;

fn _print_state(state: &State) {
    println!(
        "{}\n",
        state
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
            .join("\n")
    );
}

fn is_occupied(state: &State, i: usize, j: usize) -> usize {
    (state[i][j] == Position::Occupied) as usize
}

fn count_adjacent_occupied(state: &State, pos: (i32, i32)) -> usize {
    let i = pos.0 as usize;
    let j = pos.1 as usize;
    let mut count = 0;
    if i > 0 {
        count += if j > 0 {
            is_occupied(state, i - 1, j - 1)
        } else {
            0
        };
        count += is_occupied(state, i - 1, j);
        count += if j < state[i - 1].len() - 1 {
            is_occupied(state, i - 1, j + 1)
        } else {
            0
        };
    }
    count += if j > 0 {
        is_occupied(state, i, j - 1)
    } else {
        0
    };
    count += if j < state[i].len() - 1 {
        is_occupied(state, i, j + 1)
    } else {
        0
    };
    if i < state.len() - 1 {
        count += if j > 0 {
            is_occupied(state, i + 1, j - 1)
        } else {
            0
        };
        count += is_occupied(state, i + 1, j);
        count += if j < state[i + 1].len() - 1 {
            is_occupied(state, i + 1, j + 1)
        } else {
            0
        };
    }
    count
}

fn find_first_seat(state: &State, mut point: (i32, i32), dir: (i32, i32)) -> usize {
    loop {
        point = (point.0 + dir.0, point.1 + dir.1);
        if point.0 < 0
            || point.0 > (state.len() - 1) as i32
            || point.1 < 0
            || point.1 > (state[0].len() - 1) as i32
        {
            return 0;
        }
        let seat = state[point.0 as usize][point.1 as usize];
        if seat == Position::Occupied {
            return 1;
        } else if seat == Position::Empty {
            return 0;
        }
    }
}

fn count_visible_occupied(state: &State, pos: (i32, i32)) -> usize {
    (-1..=1)
        .map(|x| -> usize {
            (-1..=1)
                .map(|y| {
                    if x != 0 || y != 0 {
                        find_first_seat(state, pos, (x, y))
                    } else {
                        0
                    }
                })
                .sum()
        })
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

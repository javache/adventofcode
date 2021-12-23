use regex::Regex;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Hash, Copy, Clone, PartialEq, Eq)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Amphipod {
    fn from(input: &str) -> Amphipod {
        match input {
            "A" => Amphipod::A,
            "B" => Amphipod::B,
            "C" => Amphipod::C,
            "D" => Amphipod::D,
            _ => panic!("Unknown type {}", input),
        }
    }

    fn target_room(&self) -> usize {
        match self {
            Amphipod::A => 0,
            Amphipod::B => 1,
            Amphipod::C => 2,
            Amphipod::D => 3,
        }
    }

    fn move_cost(&self) -> usize {
        match self {
            Amphipod::A => 1,
            Amphipod::B => 10,
            Amphipod::C => 100,
            Amphipod::D => 1000,
        }
    }
}

const ROOM_SIZE: usize = 4;

#[derive(Default, Copy, Clone, Hash)]
struct State {
    hall: [Option<Amphipod>; 11],
    rooms: [[Option<Amphipod>; ROOM_SIZE]; 4],
}

const VALID_HALL_POSITIONS: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];

fn get_move_cost(elem: Amphipod, hall_idx: usize, room_idx: usize, idx_in_room: usize) -> usize {
    let other_hall_idx = room_to_hall_idx(room_idx);
    let hall_distance = if hall_idx >= other_hall_idx {
        hall_idx - other_hall_idx
    } else {
        other_hall_idx - hall_idx
    };
    (hall_distance + idx_in_room + 1) * elem.move_cost()
}

fn room_to_hall_idx(room_idx: usize) -> usize {
    2 + room_idx * 2
}

impl State {
    fn from(hall: [Option<Amphipod>; 11], rooms: [[Option<Amphipod>; ROOM_SIZE]; 4]) -> State {
        State { hall, rooms }
    }

    fn can_room_finalize(&self, room_idx: usize) -> bool {
        self.rooms[room_idx].iter().all(|elem| match elem {
            Some(amphipod) => amphipod.target_room() == room_idx,
            None => true,
        })
    }

    fn get_hall_destinations(&self, curr: usize) -> Vec<usize> {
        VALID_HALL_POSITIONS
            .iter()
            .cloned()
            .filter(|pos| curr != *pos && self.is_hall_path_unblocked(curr, *pos))
            .collect()
    }

    fn get_top_of_room(&self, room_idx: usize) -> Option<(usize, Amphipod)> {
        self.rooms[room_idx]
            .iter()
            .enumerate()
            .find_map(|(idx, elem)| elem.map(|val| (idx, val)))
    }

    fn is_complete(&self) -> bool {
        return self.rooms.iter().enumerate().all(|(idx, room)| {
            room.iter().all(|elem| match elem {
                Some(amphipod) => amphipod.target_room() == idx,
                None => false,
            })
        });
    }

    fn is_hall_path_unblocked(&self, start: usize, dest: usize) -> bool {
        (if dest >= start {
            &self.hall[(start + 1)..=dest]
        } else {
            &self.hall[dest..=(start - 1)]
        })
        .iter()
        .all(|elem| elem.is_none())
    }

    fn explore_states(&self) -> Vec<(State, usize)> {
        let hall_occupants: Vec<usize> = self
            .hall
            .iter()
            .enumerate()
            .flat_map(|(idx, elem)| elem.map(|_| idx))
            .collect();

        // If we can move something into its final position, let's ignore other options
        for idx in &hall_occupants {
            let elem = self.hall[*idx].unwrap();
            let dest_room = elem.target_room();
            if !self.can_room_finalize(dest_room)
                || !self.is_hall_path_unblocked(*idx, room_to_hall_idx(dest_room))
            {
                continue;
            }

            let mut new_hall = self.hall.clone();
            new_hall[*idx] = None;
            let mut new_rooms = self.rooms.clone();
            let idx_in_room = new_rooms[dest_room]
                .iter()
                .rposition(|elem| elem.is_none())
                .unwrap();
            new_rooms[dest_room][idx_in_room] = Some(elem);

            return vec![(
                State::from(new_hall, new_rooms),
                get_move_cost(elem, *idx, dest_room, idx_in_room),
            )];
        }

        // Move the top element from the rooms
        (0..self.rooms.len())
            .flat_map(|room_idx| {
                // If the rooom is finalizable, it's either empty or all its
                // elements are in the right place and there's nothing to move
                if self.can_room_finalize(room_idx) {
                    return vec![];
                }

                let (idx_in_room, elem) = self.get_top_of_room(room_idx).unwrap();
                self.get_hall_destinations(room_to_hall_idx(room_idx))
                    .iter()
                    .map(|dest| {
                        let mut new_hall = self.hall.clone();
                        new_hall[*dest] = Some(elem);
                        let mut new_rooms = self.rooms.clone();
                        new_rooms[room_idx][idx_in_room] = None;
                        (
                            State::from(new_hall, new_rooms),
                            get_move_cost(elem, *dest, room_idx, idx_in_room),
                        )
                    })
                    .collect::<Vec<(State, usize)>>()
            })
            .collect()
    }
}

fn _solve_dfs(state: &State) -> Option<usize> {
    if state.is_complete() {
        Some(0)
    } else {
        state
            .explore_states()
            .iter()
            .flat_map(|(next_state, transition_cost)| {
                _solve_dfs(next_state).map(|cost| cost + transition_cost)
            })
            .min()
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.hall.iter().zip(other.hall).all(|(a, b)| *a == b)
            && self.rooms.iter().zip(other.rooms).all(|(a, b)| *a == b)
    }
}

impl Eq for State {}

#[derive(Eq, PartialEq)]
struct Vertex {
    cost: usize,
    state: State,
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve_dijkstra(state: &State) -> usize {
    let mut distances: HashMap<State, usize> = HashMap::new();
    let mut heap = BinaryHeap::new();
    heap.push(Vertex {
        state: *state,
        cost: 0,
    });

    while let Some(Vertex { state, cost }) = heap.pop() {
        if state.is_complete() {
            return cost;
        }
        if cost > *distances.get(&state).unwrap_or(&usize::MAX) {
            continue;
        }
        for (next_state, transition_cost) in state.explore_states() {
            let next = Vertex {
                cost: cost + transition_cost,
                state: next_state,
            };
            if next.cost < *distances.get(&next_state).unwrap_or(&usize::MAX) {
                distances.insert(next.state, next.cost);
                heap.push(next);
            }
        }
    }
    panic!("No path found");
}

fn main() -> io::Result<()> {
    let mut input = State::default();
    let input_re = Regex::new(r"#(\w)#(\w)#(\w)#(\w)#").unwrap();

    let mut i = 0;
    io::stdin().lock().lines().flatten().for_each(|line| {
        if let Some(re_match) = input_re.captures(&line) {
            input.rooms[0][i] = Some(Amphipod::from(&re_match[1]));
            input.rooms[1][i] = Some(Amphipod::from(&re_match[2]));
            input.rooms[2][i] = Some(Amphipod::from(&re_match[3]));
            input.rooms[3][i] = Some(Amphipod::from(&re_match[4]));
            i += 1;
        }
    });

    println!(
        "The minimal cost for a solution is {}",
        solve_dijkstra(&input)
    );
    Ok(())
}

use std::fmt;
use std::io::{self, Read};

#[derive(Clone, Debug)]
struct State {
    x_base: i32,
    width: i32,
    y_base: i32,
    height: i32,
    z_base: i32,
    depth: i32,
    w_base: i32,
    w_size: i32,
    data: Vec<bool>,
}

impl State {
    fn new(input: Vec<Vec<bool>>) -> State {
        State {
            x_base: 0,
            width: input[0].len() as i32,
            y_base: 0,
            height: input.len() as i32,
            z_base: 0,
            depth: 1,
            w_base: 0,
            w_size: 1,
            data: input.into_iter().flatten().collect(),
        }
    }

    fn get_offset(&self, x: i32, y: i32, z: i32, w: i32) -> usize {
        ((w - self.w_base) * self.height * self.width * self.depth
            + (z - self.z_base) * self.height * self.width
            + (y - self.y_base) * self.width
            + (x - self.x_base)) as usize
    }

    fn get(&self, x: i32, y: i32, z: i32, w: i32) -> bool {
        if x >= self.x_base
            && x < self.width + self.x_base
            && y >= self.y_base
            && y < self.height + self.y_base
            && z >= self.z_base
            && z < self.depth + self.z_base
            && w >= self.w_base
            && w < self.depth + self.w_base
        {
            self.data[self.get_offset(x, y, z, w)]
        } else {
            false
        }
    }

    fn set(&mut self, x: i32, y: i32, z: i32, w: i32, value: bool) -> () {
        let offset = self.get_offset(x, y, z, w);
        self.data[offset] = value;
    }

    fn count_active_neighbours3(&self, x: i32, y: i32, z: i32) -> usize {
        (-1..=1)
            .map(|z_off| {
                (-1..=1)
                    .map(|y_off| {
                        (-1..=1)
                            .map(|x_off| {
                                if x_off == 0 && y_off == 0 && z_off == 0 {
                                    0
                                } else {
                                    self.get(x + x_off, y + y_off, z + z_off, 0) as usize
                                }
                            })
                            .sum::<usize>()
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    fn count_active_neighbours4(&self, x: i32, y: i32, z: i32, w: i32) -> usize {
        (-1..=1)
            .map(|w_off| {
                (-1..=1)
                    .map(|z_off| {
                        (-1..=1)
                            .map(|y_off| {
                                (-1..=1)
                                    .map(|x_off| {
                                        if x_off == 0 && y_off == 0 && z_off == 0 && w_off == 0 {
                                            0
                                        } else {
                                            self.get(x + x_off, y + y_off, z + z_off, w + w_off)
                                                as usize
                                        }
                                    })
                                    .sum::<usize>()
                            })
                            .sum::<usize>()
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    fn count_active(&self) -> usize {
        self.data.iter().map(|b| *b as usize).sum()
    }

    fn new_with_extended_size(&self) -> State {
        let extended_len =
            (self.w_size + 2) * (self.depth + 2) * (self.height + 2) * (self.width + 2);
        let data = vec![false; extended_len as usize];
        State {
            x_base: self.x_base - 1,
            width: self.width + 2,
            y_base: self.y_base - 1,
            height: self.height + 2,
            z_base: self.z_base - 1,
            depth: self.depth + 2,
            w_base: self.w_base - 1,
            w_size: self.w_size + 2,
            data,
        }
    }

    fn step(&self, dimensions: usize) -> State {
        let mut next = self.new_with_extended_size();
        let mut w_range = 0..1;
        if dimensions == 4 {
            w_range = next.w_base..(next.w_base + next.w_size)
        }
        for w in w_range {
            for z in next.z_base..(next.z_base + next.depth) {
                for y in next.y_base..(next.y_base + next.height) {
                    for x in next.x_base..(next.x_base + next.width) {
                        let neighbours;
                        if dimensions == 4 {
                            neighbours = self.count_active_neighbours4(x, y, z, w);
                        } else {
                            neighbours = self.count_active_neighbours3(x, y, z);
                        }
                        if self.get(x, y, z, w) {
                            next.set(x, y, z, w, neighbours == 2 || neighbours == 3);
                        } else {
                            next.set(x, y, z, w, neighbours == 3);
                        }
                    }
                }
            }
        }
        next
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for w in 0..self.w_size {
            for z in 0..self.depth {
                let offset =
                    self.get_offset(self.x_base, self.y_base, self.z_base + z, self.w_base + w);
                let sum: usize = self.data[offset..(offset + (self.width * self.height) as usize)]
                    .iter()
                    .map(|b| *b as usize)
                    .sum();
                if sum == 0 {
                    continue;
                }

                writeln!(f, "z={} w={}", self.z_base + z, self.w_base + w)?;
                for y in 0..self.height {
                    let row: String = (0..self.width)
                        .map(|x| {
                            if self.get(
                                self.x_base + x,
                                self.y_base + y,
                                self.z_base + z,
                                self.w_base + w,
                            ) {
                                '#'
                            } else {
                                '.'
                            }
                        })
                        .collect();
                    writeln!(f, "{}", row)?;
                }
            }
        }
        Ok(())
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
    for _ in 0..6 {
        state = state.step(3);
    }
    println!(
        "(1) After 6 steps in 3 dimensions, there's {} cells",
        state.count_active()
    );

    let mut state = initial_state;
    for _ in 0..6 {
        state = state.step(4);
    }
    println!(
        "(2) After 6 steps in 4 dimensions, there's {} cells",
        state.count_active()
    );

    Ok(())
}

use std::fmt;
use std::io::{self, Read};

#[derive(Clone)]
struct Tile {
    id: i64,
    size: usize,
    data: Vec<u128>,
}

const PATTERN: &'static str = "
                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ";
const PATTERN_LEN: usize = 20;

fn parse_input_line(line: &str) -> u128 {
    line.chars()
        .rev()
        .fold(0, |acc, c| acc << 1 | (if c == '#' { 1 } else { 0 }))
}

impl Tile {
    fn parse(input: &str) -> Tile {
        let input = input.split("\n").collect::<Vec<&str>>();
        Tile {
            id: input[0]["Tile ".len()..input[0].len() - 1].parse().unwrap(),
            size: input[1].len(),
            data: input.into_iter().skip(1).map(parse_input_line).collect(),
        }
    }

    fn rotated(&self) -> Tile {
        Tile {
            id: self.id,
            size: self.size,
            data: (0..self.size)
                .map(|i| {
                    (0..self.size).fold(0, |acc, j| {
                        let bit = self.data[(self.size - j - 1)] & (1 << i);
                        acc | (bit >> i) << j
                    })
                })
                .collect(),
        }
    }

    fn flipped(&self) -> Tile {
        Tile {
            id: self.id,
            size: self.size,
            data: self.data.iter().copied().rev().collect(),
        }
    }

    fn matches_right_edge(&self, other: &Tile) -> bool {
        (0..self.size).all(|i| {
            ((self.data[i] & 1) << (self.size - 1)) == (other.data[i] & (1 << (self.size - 1)))
        })
    }

    fn matches_bottom_edge(&self, other: &Tile) -> bool {
        self.data[0] == other.data[self.size - 1]
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.size {
            if self.size == 10 {
                f.write_fmt(format_args!("{:010b}\n", self.data[i].reverse_bits() >> 22))?;
            } else {
                f.write_fmt(format_args!("{:024b}\n", self.data[i].reverse_bits() >> 8))?;
            }
        }
        Ok(())
    }
}

fn solve(tiles: &Vec<Tile>, solution: &mut Vec<Tile>, grid_size: usize) -> bool {
    if solution.len() > 0 {
        let last_idx = solution.len() - 1;
        if last_idx % grid_size > 0
            && !solution[last_idx].matches_right_edge(&solution[last_idx - 1])
        {
            return false;
        }
        if last_idx > grid_size
            && !solution[last_idx].matches_bottom_edge(&solution[last_idx - grid_size])
        {
            return false;
        }
    }

    if solution.len() == tiles.len() {
        return true;
    }

    for i in 0..tiles.len() {
        // TODO: generate all variations with iterator and reuse in find_and_remove_pattern
        let curr_tile = &tiles[i];
        if solution.iter().any(|t| t.id == curr_tile.id) {
            continue;
        }

        solution.push(curr_tile.clone());
        if solve(tiles, solution, grid_size) {
            return true;
        }

        let flipped = curr_tile.flipped();
        *solution.last_mut().unwrap() = flipped;
        if solve(tiles, solution, grid_size) {
            return true;
        }

        let mut rotated = curr_tile.clone();
        for _ in 0..3 {
            rotated = rotated.rotated();
            *solution.last_mut().unwrap() = rotated.clone();
            if solve(tiles, solution, grid_size) {
                return true;
            }

            *solution.last_mut().unwrap() = rotated.flipped();
            if solve(tiles, solution, grid_size) {
                return true;
            }
        }

        solution.pop();
    }

    false
}

fn merge_solution(solution: &Vec<Tile>, grid_size: usize) -> Tile {
    let tile_size = solution[0].size;
    let reduced_tile_size = tile_size - 2;
    let merged_size = reduced_tile_size * grid_size;
    Tile {
        id: 0,
        size: merged_size,
        data: (0..merged_size)
            .map(|i| {
                (0..grid_size).fold(0, |acc, j| -> u128 {
                    let tile = (i / reduced_tile_size) * grid_size + j;
                    let row = solution[tile].data[i % reduced_tile_size + 1];
                    let subsection = (row & ((1 << tile_size - 1) - 1)) >> 1;
                    acc | subsection.wrapping_shl((j * reduced_tile_size) as u32)
                })
            })
            .collect(),
    }
}

fn find_and_remove_pattern_impl(
    input: &Tile,
    pattern: &Vec<u128>,
    pattern_len: usize,
) -> Option<Tile> {
    let mut found: Vec<(usize, usize)> = Vec::new();
    for i in 0..(input.size - pattern.len()) {
        for j in 0..(input.size - pattern_len) {
            if pattern.iter().enumerate().all(|(k, p)| {
                input.data[i + k].wrapping_shr((input.size - pattern_len - j) as u32) & p == *p
            }) {
                found.push((i, j));
            }
        }
    }

    if found.len() > 0 {
        let mut result = input.clone();
        for (i, j) in found {
            for (k, p) in pattern.iter().enumerate() {
                result.data[i + k] &= !(p << (input.size - pattern_len - j));
            }
        }
        Some(result)
    } else {
        None
    }
}

fn find_and_remove_pattern(input: &Tile, pattern: &Vec<u128>, pattern_len: usize) -> Tile {
    if let Some(result) = find_and_remove_pattern_impl(input, pattern, pattern_len) {
        return result;
    }

    let flipped = input.flipped();
    if let Some(result) = find_and_remove_pattern_impl(&flipped, pattern, pattern_len) {
        return result;
    }

    let mut rotated = input.clone();
    for _ in 0..3 {
        rotated = rotated.rotated();
        if let Some(result) = find_and_remove_pattern_impl(&rotated, pattern, pattern_len) {
            return result;
        }

        if let Some(result) = find_and_remove_pattern_impl(&rotated.flipped(), pattern, pattern_len)
        {
            return result;
        }
    }

    unreachable!()
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let mut tiles: Vec<Tile> = Vec::new();
    for tile in input.split("\n\n") {
        tiles.push(Tile::parse(tile));
    }

    let grid_size = (tiles.len() as f32).sqrt() as usize;
    let mut solution = vec![];
    if solve(&tiles, &mut solution, grid_size) {
        let solution_ids: Vec<i64> = solution.iter().map(|s| s.id).collect();
        println!("(1) Found solution: {:?}", solution_ids);
        println!(
            "Product of corners is {}",
            solution_ids[0]
                * solution_ids[grid_size - 1]
                * solution_ids[grid_size * (grid_size - 1)]
                * solution_ids[grid_size * grid_size - 1]
        );

        let merged = merge_solution(&solution, grid_size);
        let pattern: Vec<u128> = PATTERN.split("\n").skip(1).map(parse_input_line).collect();
        let remains = find_and_remove_pattern(&merged, &pattern, PATTERN_LEN);
        let waters: u32 = remains.data.iter().map(|row| row.count_ones()).sum();
        println!("(2) Water that is not part of the monster {:?}", waters);
    }

    Ok(())
}

use itertools::Itertools;
use std::io::{self, BufRead};

type Grid = Vec<Vec<char>>;

fn find_all_galaxies(input: &Grid) -> Vec<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c == '#')
                .map(|(x, _)| (y, x))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn find_empty_rows(input: &Grid) -> Vec<usize> {
    (0..input.len())
        .filter(|&y| input[y].iter().all(|&c| c == '.'))
        .collect()
}

fn find_empty_columns(input: &Grid) -> Vec<usize> {
    (0..input[0].len())
        .filter(|&x| input.iter().all(|row| row[x] == '.'))
        .collect()
}

fn calculate_distance(
    a: &(usize, usize),
    b: &(usize, usize),
    empty_space: &(Vec<usize>, Vec<usize>),
    multiplier: usize,
) -> usize {
    // Manhattan distance
    let dist = ((a.0 as i32 - b.0 as i32).abs() + (a.1 as i32 - b.1 as i32).abs()) as usize;

    let mut empty_spaces_crossed = ((a.0.min(b.0))..(a.0.max(b.0)))
        .filter(|y| empty_space.0.contains(y))
        .count();
    empty_spaces_crossed += ((a.1.min(b.1))..(a.1.max(b.1)))
        .filter(|x| empty_space.1.contains(x))
        .count();

    dist + empty_spaces_crossed * (multiplier - 1)
}

fn main() {
    let input: Grid = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let galaxies = find_all_galaxies(&input);
    let empty_space = (find_empty_rows(&input), find_empty_columns(&input));

    let distances_1 = galaxies
        .iter()
        .tuple_combinations()
        .map(|(galaxy_a, galaxy_b)| calculate_distance(galaxy_a, galaxy_b, &empty_space, 2))
        .sum::<usize>();
    println!("(1) Distance between all galaxies is {}", distances_1);

    let distances_2 = galaxies
        .iter()
        .tuple_combinations()
        .map(|(galaxy_a, galaxy_b)| calculate_distance(galaxy_a, galaxy_b, &empty_space, 1000000))
        .sum::<usize>();
    println!("(2) Distance between all galaxies is {}", distances_2);
}

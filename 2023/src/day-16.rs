use std::collections::HashSet;
use std::io::{self, BufRead};

type Grid = Vec<Vec<char>>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn simulate(
    grid: &Grid,
    position: (usize, usize),
    direction: Direction,
) -> Vec<((usize, usize), Direction)> {
    let next = match (grid[position.0][position.1], &direction) {
        ('/', Direction::South) | ('\\', Direction::North) => vec![(position, Direction::West)],
        ('/', Direction::North) | ('\\', Direction::South) => vec![(position, Direction::East)],
        ('/', Direction::East) | ('\\', Direction::West) => vec![(position, Direction::North)],
        ('/', Direction::West) | ('\\', Direction::East) => vec![(position, Direction::South)],
        ('|', Direction::East) | ('|', Direction::West) => {
            vec![(position, Direction::North), (position, Direction::South)]
        }
        ('-', Direction::North) | ('-', Direction::South) => {
            vec![(position, Direction::West), (position, Direction::East)]
        }
        _ => vec![(position, direction)],
    };

    next.iter()
        .map(|(position, direction)| {
            (
                match direction {
                    Direction::North => (position.0 as i32 - 1, position.1 as i32),
                    Direction::East => (position.0 as i32, position.1 as i32 + 1),
                    Direction::South => (position.0 as i32 + 1, position.1 as i32),
                    Direction::West => (position.0 as i32, position.1 as i32 - 1),
                },
                *direction,
            )
        })
        .filter(|&((y, x), _)| {
            y >= 0 && y < grid.len() as i32 && x >= 0 && x < grid[0].len() as i32
        })
        .map(|((y, x), dir)| ((y as usize, x as usize), dir))
        .collect()
}

fn calculate_energized_tiles(grid: &Grid, start: ((usize, usize), Direction)) -> usize {
    let mut occupied = grid
        .iter()
        .map(|row| vec![false; row.len()])
        .collect::<Vec<_>>();
    occupied[start.0 .0][start.0 .1] = true;

    let mut beams = vec![start];
    let mut seen: HashSet<((usize, usize), Direction)> = HashSet::new();
    while !beams.is_empty() {
        beams = beams
            .into_iter()
            .flat_map(|(position, direction)| simulate(&grid, position, direction))
            .filter(|beam| !seen.contains(&beam))
            .collect();
        for beam in &beams {
            occupied[beam.0 .0][beam.0 .1] = true;
            seen.insert(*beam);
        }
    }

    occupied
        .iter()
        .map(|row| row.iter().filter(|&x| *x).count())
        .sum()
}

fn main() {
    let input: Grid = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    println!(
        "(1) {}",
        calculate_energized_tiles(&input, ((0, 0), Direction::East))
    );

    let max = (0..input.len())
        .map(|y| ((y, 0), Direction::East))
        .chain((0..input.len()).map(|y| ((y, input[0].len() - 1), Direction::West)))
        .chain((0..input[0].len()).map(|x| ((0, x), Direction::South)))
        .chain((0..input[0].len()).map(|x| ((input.len() - 1, x), Direction::North)))
        .map(|position| calculate_energized_tiles(&input, position))
        .max();
    println!("(2) {}", max.unwrap());
}

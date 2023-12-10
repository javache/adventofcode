use std::io::{self, BufRead};

type Grid = Vec<Vec<char>>;

fn find_start(grid: &Grid) -> (usize, usize) {
    for (y, row) in grid.iter().enumerate() {
        if let Some(x) = row.iter().position(|&c| c == 'S') {
            return (y, x);
        }
    }
    panic!("Could not find start");
}

fn get_start_symbol(grid: &Grid, start: (usize, usize)) -> char {
    let north = ['|', '7', 'F'].contains(&grid[start.0 - 1][start.1]);
    let south = ['|', 'L', 'J'].contains(&grid[start.0 + 1][start.1]);
    let east = ['-', 'L', 'F'].contains(&grid[start.0][start.1 - 1]);
    let west = ['-', 'J', '7'].contains(&grid[start.0][start.1 + 1]);
    match (north, south, east, west) {
        (true, true, false, false) => '|',
        (false, false, true, true) => '-',
        (true, false, true, false) => 'L',
        (true, false, false, true) => 'J',
        (false, true, false, true) => 'F',
        (false, true, true, false) => '7',
        _ => unreachable!(),
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn get_main_loop(grid: &Grid, start: (usize, usize)) -> Vec<Vec<usize>> {
    let mut dir = Direction::North;
    let mut pos = start;

    let mut points = vec![vec![]; grid.len()];
    loop {
        points[pos.0].push(pos.1);

        let (dir_a, dir_b) = match grid[pos.0][pos.1] {
            '|' => (Direction::North, Direction::South),
            '-' => (Direction::East, Direction::West),
            'L' => (Direction::North, Direction::East),
            'J' => (Direction::North, Direction::West),
            '7' => (Direction::South, Direction::West),
            'F' => (Direction::South, Direction::East),
            _ => unreachable!(),
        };

        let next_dir = if dir == dir_a { dir_b } else { dir_a };
        (pos, dir) = match next_dir {
            Direction::North => ((pos.0 - 1, pos.1), Direction::South),
            Direction::South => ((pos.0 + 1, pos.1), Direction::North),
            Direction::East => ((pos.0, pos.1 + 1), Direction::West),
            Direction::West => ((pos.0, pos.1 - 1), Direction::East),
        };

        if pos == start {
            return points;
        }
    }
}

fn scan_grid(grid: &Grid, main_loop: &mut Vec<Vec<usize>>) -> usize {
    main_loop
        .iter_mut()
        .enumerate()
        .map(|(y, points_crossed)| {
            let row = &grid[y];

            points_crossed.sort();

            let mut paint_start = None;
            let mut painted = 0;
            let mut intersections = 0;

            let mut iter = points_crossed.iter();
            while let Some(mut point) = iter.next() {
                if let Some(prev_point) = paint_start {
                    painted += point - prev_point - 1;
                    paint_start = None;
                }

                match row[*point] {
                    '|' => {
                        intersections += 1;
                    }
                    start_point @ ('L' | 'F') => {
                        // Consume the rest of the horizontal sequence
                        while let Some(next_point) = iter.next() {
                            point = next_point;
                            if row[*next_point] != '-' {
                                break;
                            }
                        }

                        // Ignore sequences that end in the same direction they started
                        let terminators = (start_point, row[*point]);
                        if terminators == ('L', '7') || terminators == ('F', 'J') {
                            intersections += 1;
                        }
                    }
                    _ => unreachable!(),
                };

                // Use the even-odd rule to determine which points are painted
                if intersections % 2 == 1 {
                    paint_start = Some(point);
                }
            }
            painted
        })
        .sum()
}

fn main() {
    let mut input: Grid = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    let start = find_start(&input);
    input[start.0][start.1] = get_start_symbol(&input, start);

    let mut main_loop = get_main_loop(&input, start);
    println!(
        "(1) Furthest position on the main loop is {} steps in",
        (main_loop.iter().map(|row| row.len()).sum::<usize>() as f64 / 2f64).ceil()
    );

    let enclosed_tiles = scan_grid(&input, &mut main_loop);
    println!(
        "(2) There are {} tiles that are enclosed by the main loop",
        enclosed_tiles
    );
}

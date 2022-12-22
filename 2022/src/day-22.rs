use itertools::Itertools;
use std::io::{self, Read};
use std::iter;

#[derive(Debug)]
enum Instr {
    Move(i32),
    RotateR,
    RotateL,
}

type Grid = Vec<Vec<char>>;
type Point = (i32, i32);

fn parse_instructions(input: &str) -> Vec<Instr> {
    let mut output = vec![];
    let mut num_start = 0;
    for (curr, ch) in input.chars().chain(iter::once('$')).enumerate() {
        if ch.is_digit(10) {
            continue;
        }
        if curr > num_start {
            output.push(Instr::Move(input[num_start..curr].parse().unwrap()));
        }
        if ch != '$' {
            output.push(if ch == 'R' {
                Instr::RotateR
            } else {
                Instr::RotateL
            });
        }
        num_start = curr + 1;
    }
    output
}

fn wrap_x(grid: &Grid, mut point: Point, direction: i32) -> (Point, i32) {
    let row = &grid[point.1 as usize];
    if point.0 < 0 || point.0 >= row.len() as i32 || row[point.0 as usize] == ' ' {
        point.0 = (if direction == 0 {
            row.iter().position(|&c| c != ' ')
        } else {
            row.iter().rposition(|&c| c != ' ')
        })
        .unwrap() as i32;
    }
    (point, direction)
}

fn wrap_y(grid: &Grid, mut point: Point, direction: i32) -> (Point, i32) {
    if point.1 < 0
        || point.1 >= grid.len() as i32
        || grid[point.1 as usize][point.0 as usize] == ' '
    {
        let mut column = grid.iter().map(|row| &row[point.0 as usize]);
        point.1 = (if direction == 1 {
            column.position(|&c| c != ' ')
        } else {
            column.rposition(|&c| c != ' ')
        })
        .unwrap() as i32;
    }
    (point, direction)
}

fn walk_path<StepFn: Fn(Point, i32) -> (Point, i32)>(
    grid: &Grid,
    instructions: &Vec<Instr>,
    step: StepFn,
) -> i32 {
    let (pos, direction) = instructions.iter().fold(
        wrap_x(grid, (0, 0), 0),
        |(mut pos, mut direction), instr| match instr {
            Instr::Move(num) => {
                for _ in 0..*num {
                    let (next_pos, next_direction) = step(pos, direction);
                    if grid[next_pos.1 as usize][next_pos.0 as usize] == '#' {
                        break;
                    } else {
                        pos = next_pos;
                        direction = next_direction;
                    }
                }
                (pos, direction)
            }
            Instr::RotateR => (pos, (direction + 1).rem_euclid(4)),
            Instr::RotateL => (pos, (direction - 1).rem_euclid(4)),
        },
    );
    1000 * (pos.1 + 1) + 4 * (pos.0 + 1) + direction
}

const FACE_SIZE: i32 = 50;

// Hard-coded for my input
fn get_cube_face(point: &Point) -> usize {
    match (point.0 / FACE_SIZE, point.1 / FACE_SIZE) {
        _ if point.0 < 0 || point.1 < 0 => 0,
        (2, 0) => 1,
        (1, 0) => 2,
        (1, 1) => 3,
        (1, 2) => 4,
        (0, 2) => 5,
        (0, 3) => 6,
        _ => 0,
    }
}

fn step_cube(pos: Point, direction: i32) -> (Point, i32) {
    let next_pos = match direction {
        0 => (pos.0 + 1, pos.1),
        1 => (pos.0, pos.1 + 1),
        2 => (pos.0 - 1, pos.1),
        3 => (pos.0, pos.1 - 1),
        _ => panic!("Invalid direction {}", direction),
    };
    match (get_cube_face(&pos), direction) {
        _ if get_cube_face(&next_pos) != 0 => (next_pos, direction),
        (1, 0) => ((2 * FACE_SIZE - 1, 3 * FACE_SIZE - pos.1 - 1), 2), /* face 4 */
        (1, 1) => ((2 * FACE_SIZE - 1, pos.0 - FACE_SIZE), 2),         /* face 3 */
        (1, 3) => ((pos.0 - 2 * FACE_SIZE, 4 * FACE_SIZE - 1), 3),     /* face 6 */
        (2, 2) => ((0, 3 * FACE_SIZE - pos.1 - 1), 0),                 /* face 5 */
        (2, 3) => ((0, 2 * FACE_SIZE + pos.0), 0),                     /* face 6 */
        (3, 0) => ((FACE_SIZE + pos.1, FACE_SIZE - 1), 3),             /* face 1 */
        (3, 2) => ((pos.1 - FACE_SIZE, 2 * FACE_SIZE), 1),             /* face 5 */
        (4, 0) => ((3 * FACE_SIZE - 1, 3 * FACE_SIZE - pos.1 - 1), 2), /* face 1 */
        (4, 1) => ((FACE_SIZE - 1, 2 * FACE_SIZE + pos.0), 2),         /* face 6 */
        (5, 2) => ((FACE_SIZE, 3 * FACE_SIZE - pos.1 - 1), 0),         /* face 2 */
        (5, 3) => ((FACE_SIZE, FACE_SIZE + pos.0), 0),                 /* face 3 */
        (6, 0) => ((pos.1 - 2 * FACE_SIZE, 3 * FACE_SIZE - 1), 3),     /* face 4 */
        (6, 1) => ((2 * FACE_SIZE + pos.0, 0), 1),                     /* face 1 */
        (6, 2) => ((FACE_SIZE + pos.1 - 3 * FACE_SIZE, 0), 1),         /* face 2 */
        _ => panic!(
            "Unsupported face change {:?}",
            (get_cube_face(&pos), direction)
        ),
    }
}

fn main() -> io::Result<()> {
    let mut input: String = String::new();
    io::stdin().lock().read_to_string(&mut input)?;

    let (mut grid, instructions) = input
        .split("\n\n")
        .collect_tuple()
        .map(|(grid, instructions)| {
            (
                grid.split("\n")
                    .map(|str| str.chars().collect())
                    .collect::<Grid>(),
                parse_instructions(instructions),
            )
        })
        .unwrap();

    let grid_width = grid.iter().map(|row| row.len()).max().unwrap();
    for row in &mut grid {
        row.resize(grid_width, ' ');
    }

    let step_simple = |pos: Point, direction: i32| {
        if direction == 0 || direction == 2 {
            let next_x = pos.0 + if direction == 0 { 1 } else { -1 };
            wrap_x(&grid, (next_x, pos.1), direction)
        } else {
            let next_y = pos.1 + if direction == 1 { 1 } else { -1 };
            wrap_y(&grid, (pos.0, next_y), direction)
        }
    };

    println!(
        "(1) The password is {}",
        walk_path(&grid, &instructions, step_simple)
    );
    println!(
        "(2) The password is {}",
        walk_path(&grid, &instructions, step_cube)
    );

    Ok(())
}

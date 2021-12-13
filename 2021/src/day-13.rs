use regex::Regex;
use std::collections::HashSet;
use std::io::{self, BufRead};

type Point = (u32, u32);
type Grid = HashSet<Point>;
enum Fold {
    X(u32),
    Y(u32),
}

fn apply_fold(state: &Grid, fold: &Fold) -> Grid {
    state
        .iter()
        .map(|point| match fold {
            Fold::X(x) if point.0 > *x => (x - (point.0 - x), point.1),
            Fold::Y(y) if point.1 > *y => (point.0, y - (point.1 - y)),
            _ => *point,
        })
        .collect()
}

fn print_grid(grid: &Grid) {
    let max_x = *grid.iter().map(|(x, _)| x).max().unwrap();
    let max_y = *grid.iter().map(|(_, y)| y).max().unwrap();
    for y in 0..=max_y {
        println!(
            "{}",
            (0..=max_x)
                .map(|x| if grid.contains(&(x, y)) { '█' } else { '.' })
                .collect::<String>()
        );
    }
}

fn main() -> io::Result<()> {
    let mut input: Grid = HashSet::new();
    let mut instructions: Vec<Fold> = vec![];
    let fold_re = Regex::new(r"^fold along (x|y)=(\d+)$").unwrap();
    for line in io::stdin().lock().lines().flatten() {
        if let [x, y] = line.split(',').collect::<Vec<&str>>()[..] {
            if let (Ok(x), Ok(y)) = (x.parse(), y.parse()) {
                input.insert((x, y));
            }
        } else if let Some(captures) = fold_re.captures(&line) {
            let fold_index: u32 = captures[2].parse().unwrap();
            instructions.push(match &captures[1] {
                "x" => Fold::X(fold_index),
                "y" => Fold::Y(fold_index),
                _ => panic!("Unexpected fold {}", line),
            });
        }
    }

    let mut state = apply_fold(&input, &instructions[0]);
    println!("(1) After the first fold, there are {} points", state.len());

    state = instructions[1..]
        .iter()
        .fold(state, |state, fold| apply_fold(&state, fold));

    println!("(2) After applying all folds, a pattern appears");
    print_grid(&state);

    Ok(())
}

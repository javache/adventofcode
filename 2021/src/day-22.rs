use itertools::Itertools;
use regex::Regex;
use std::io::{self, BufRead};

type Point = (i32, i32, i32);
type Cuboid = (Point, Point);
type Instruction = (Cuboid, bool);
type State = Vec<(i32, Vec<(i32, Vec<(i32, bool)>)>)>;

fn split_range(splits: &mut Vec<i32>, point_1: i32, point_2: i32) {
    match splits.binary_search(&point_1) {
        Ok(_) => (),
        Err(pos) => splits.insert(pos, point_1),
    }
    match splits.binary_search(&point_2) {
        Ok(_) => (),
        Err(pos) => splits.insert(pos, point_2),
    }
}

fn get_point_state(state: &State, point: &Point) -> bool {
    let x_idx = match state.binary_search_by_key(&point.0, |split| split.0) {
        Ok(idx) => idx,
        Err(idx) => idx,
    };
    if x_idx == state.len() {
        return false;
    }

    let y_splits = &state[x_idx].1;
    let y_idx = match y_splits.binary_search_by_key(&point.1, |split| split.0) {
        Ok(idx) => idx,
        Err(idx) => idx,
    };
    if y_idx == y_splits.len() {
        return false;
    }

    let z_splits = &y_splits[y_idx].1;
    let z_idx = match z_splits.binary_search_by_key(&point.2, |split| split.0) {
        Ok(idx) => idx,
        Err(idx) => idx,
    };
    if z_idx == z_splits.len() {
        false
    } else {
        z_splits[z_idx].1
    }
}

fn evaluate_instructions(cuboid: &Cuboid, instructions: &Vec<Instruction>) -> usize {
    let mut state: State = vec![];
    for (i, instr) in instructions.iter().enumerate() {
        println!("Processing instruction {}", i + 1);
        let (range_min, range_max) = (instr.0 .0, instr.0 .1);

        let mut x_splits: Vec<i32> = state.iter().map(|split| split.0).collect();
        split_range(&mut x_splits, range_min.0 - 1, range_max.0);

        // let mut y_splits: Vec<i32> = state
        //     .iter()
        //     .map(|split| split.1.iter().map(|split| split.0))
        //     .flatten()
        //     .unique()
        //     .collect();
        let mut y_splits: Vec<i32> = if state.len() > 0 {
            state[0].1.iter().map(|split| split.0).collect()
        } else {
            vec![]
        };
        split_range(&mut y_splits, range_min.1 - 1, range_max.1);

        let mut z_splits: Vec<i32> = state
            .iter()
            .map(|split| {
                split
                    .1
                    .iter()
                    .map(|split| split.1.iter().map(|split| split.0))
                    .flatten()
            })
            .flatten()
            .unique()
            .sorted()
            .collect();
        // let mut z_splits: Vec<i32> = if state.len() > 0 {
        //     state[0].1[0].1.iter().map(|split| split.0).collect()
        // } else { vec![] };
        split_range(&mut z_splits, range_min.2 - 1, range_max.2);

        // TODO: consider using BTreeMap
        // type State = HashMap<i32, HashMap<i32, HashMap<i32, bool>>>
        // TODO: could merge contiguous splits
        // TODO: we only need to re-evaluate splits within the instruction window
        // TODO: use chain+windows to iterate
        let mut prev_x_split = i32::MIN;
        let mut x_states = vec![];
        for x_split in &x_splits {
            // Avoid re-evaluating splits outside the instruction window
            // FIXME: we need to check all axis are outside the window
            // if *x_split - 1 < range_min.0 && *x_split > range_max.0 {
            //     let old_split = state.iter().find(|split| split.0 == *x_split).unwrap();
            //     x_states.push(old_split.clone());
            //     continue;
            // }

            let mut prev_y_split = i32::MIN;
            let mut y_states = vec![];
            for y_split in &y_splits {
                let mut prev_z_split = i32::MIN;
                let mut z_states: Vec<(i32, bool)> = vec![];
                for z_split in &z_splits {
                    let value = if prev_x_split >= range_min.0
                        && *x_split <= range_max.0
                        && prev_y_split >= range_min.1
                        && *y_split <= range_max.1
                        && prev_z_split >= range_min.2
                        && *z_split <= range_max.2
                    {
                        instr.1
                    } else {
                        get_point_state(&state, &(*x_split, *y_split, *z_split))
                    };

                    // Merge continuous splits
                    // if z_states[..].last().map_or(true, |last| last.1 != value) {
                    z_states.push((*z_split, value));
                    // } else {
                    //     let last_idx = z_states.len() - 1;
                    //     z_states[last_idx].0 = *z_split;
                    // }
                    prev_z_split = *z_split + 1;
                }
                y_states.push((*y_split, z_states));
                prev_y_split = *y_split + 1;
            }
            x_states.push((*x_split, y_states));
            prev_x_split = *x_split + 1;
        }
        state = x_states;
    }

    let mut count = 0;
    // TODO: use windows to iterate
    for (x_idx, x_split) in state.iter().enumerate().skip(1) {
        for (y_idx, y_split) in x_split.1.iter().enumerate().skip(1) {
            for (z_idx, z_split) in y_split.1.iter().enumerate().skip(1) {
                if z_split.1 {
                    let prev_x_split = state[x_idx - 1].0;
                    let prev_y_split = x_split.1[y_idx - 1].0;
                    let prev_z_split = y_split.1[z_idx - 1].0;
                    count += (x_split.0 - prev_x_split) as usize
                        * (y_split.0 - prev_y_split) as usize
                        * (z_split.0 - prev_z_split) as usize
                }
            }
        }
    }
    count
}

fn main() -> io::Result<()> {
    let input_re =
        Regex::new(r"^(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)$").unwrap();
    let instructions: Vec<Instruction> = io::stdin()
        .lock()
        .lines()
        .flatten()
        .flat_map(|line| {
            input_re.captures(&line).map(|re_match| {
                let nums: Vec<i32> = (2..=7).map(|i| &re_match[i]).flat_map(str::parse).collect();
                (
                    ((nums[0], nums[2], nums[4]), (nums[1], nums[3], nums[5])),
                    &re_match[1] == "on",
                )
            })
        })
        .collect();

    let count = evaluate_instructions(&((-50, -50, -50), (50, 50, 50)), &instructions);
    println!("(1) There are {} cubes lit up", count);

    Ok(())
}

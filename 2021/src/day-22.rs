use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead};

type Point = (i32, i32, i32);
type Cuboid = (Point, Point);
type Instruction = (Cuboid, bool);

fn cuboid_intersection(a: &Cuboid, b: &Cuboid) -> Option<Cuboid> {
    let x0 = a.0 .0.max(b.0 .0);
    let x1 = a.1 .0.min(b.1 .0);
    let y0 = a.0 .1.max(b.0 .1);
    let y1 = a.1 .1.min(b.1 .1);
    let z0 = a.0 .2.max(b.0 .2);
    let z1 = a.1 .2.min(b.1 .2);

    (x0 <= x1 && y0 <= y1 && z0 <= z1).then(|| ((x0, y0, z0), (x1, y1, z1)))
}

fn solve(instructions: &Vec<Instruction>) -> usize {
    let mut state: HashMap<Cuboid, i32> = HashMap::new();

    for instr in instructions.iter() {
        let mut updates: HashMap<Cuboid, i32> = HashMap::new();
        for (cuboid, value) in state.iter() {
            if let Some(intersection) = cuboid_intersection(&cuboid, &instr.0) {
                *updates.entry(intersection).or_default() -= value;
            }
        }
        if instr.1 {
            *updates.entry(instr.0).or_default() += 1;
        }
        for (cuboid, value) in updates.iter() {
            *state.entry(*cuboid).or_default() += value;
        }
    }

    state
        .iter()
        .map(|(cuboid, value)| {
            *value as usize
                * (cuboid.1 .0 - cuboid.0 .0 + 1) as usize
                * (cuboid.1 .1 - cuboid.0 .1 + 1) as usize
                * (cuboid.1 .2 - cuboid.0 .2 + 1) as usize
        })
        .sum()
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

    let window = ((-50, -50, -50), (50, 50, 50));
    let limited_instructions: Vec<Instruction> = instructions
        .iter()
        .filter(|(cuboid, _)| cuboid_intersection(cuboid, &window).is_some())
        .cloned()
        .collect();
    println!(
        "(1) There are {} cubes lit up in x=-50..50,y=-50..50,z=-50..50",
        solve(&limited_instructions)
    );

    println!("(2) There are {} cubes lit up", solve(&instructions));

    Ok(())
}

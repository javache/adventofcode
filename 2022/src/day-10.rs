use itertools::Itertools;
use std::io::{self, BufRead};

fn main() {
    let mut register: i32 = 1;
    let mut cycle_states: Vec<i32> = vec![];

    for line in io::stdin().lock().lines().flatten() {
        cycle_states.push(register);
        match line.split(' ').collect::<Vec<&str>>()[..] {
            ["addx", v] => {
                cycle_states.push(register);
                register += v.parse::<i32>().unwrap();
            }
            ["noop"] => {}
            _ => panic!("Unexpected input {}", line),
        }
    }

    let sum: i32 = (20..=220)
        .step_by(40)
        .map(|idx| idx as i32 * cycle_states[idx - 1])
        .sum();
    println!("(1) Sum of signal strengths is {}", sum);

    let output = cycle_states
        .iter()
        .enumerate()
        .map(|(idx, state)| {
            let pixel_pos = (idx % 40) as i32;
            if (state - pixel_pos).abs() <= 1 {
                '█'
            } else {
                ' '
            }
        })
        .chunks(40)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .join("\n");
    println!("(2) The CRT displays\n{}", output);
}

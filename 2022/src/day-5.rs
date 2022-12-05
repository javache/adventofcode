use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use std::io::{self, Read};

type State = Vec<Vec<char>>;

static INSTRUCTION_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^move (\d+) from (\d) to (\d)$").unwrap());

fn parse_state(input: &str) -> State {
    let mut state: State = vec![];
    for (row, line) in input.split('\n').rev().enumerate() {
        if row == 0 {
            state = vec![vec![]; (line.len() + 1) / 4];
        } else {
            for (column, character) in line.chars().skip(1).step_by(4).enumerate() {
                if character != ' ' {
                    state[column].push(character)
                }
            }
        }
    }
    state
}

fn apply_instruction(line: &str, state: &mut State, should_rev: bool) {
    let instruction_re: &Regex = &INSTRUCTION_RE;
    if let Some((count, from, to)) = instruction_re
        .captures(line)
        .unwrap()
        .iter()
        .flat_map(|n| n.unwrap().as_str().parse::<usize>())
        .collect_tuple()
    {
        let new_from_len = state[from - 1].len() - count;
        let popped: Vec<char> = Some(state[from - 1].drain(new_from_len..))
            .map(|stack| {
                if should_rev {
                    stack.rev().collect()
                } else {
                    stack.collect()
                }
            })
            .unwrap();
        state[to - 1].extend(popped);
    }
}

fn result_from_state(state: &State) -> String {
    state.iter().flat_map(|col| col.last()).collect::<String>()
}

fn main() -> io::Result<()> {
    let mut input: String = String::new();
    io::stdin().lock().read_to_string(&mut input)?;

    if let Some((init_state, instructions)) = input.split("\n\n").collect_tuple() {
        let mut state = parse_state(init_state);
        let mut state_2 = state.clone();

        for instr in instructions.split("\n") {
            apply_instruction(instr, &mut state, true);
        }
        println!("(1) Result is {}", result_from_state(&state));

        for instr in instructions.split("\n") {
            apply_instruction(instr, &mut state_2, false);
        }
        println!("(2) Result is {}", result_from_state(&state_2));
    }

    Ok(())
}

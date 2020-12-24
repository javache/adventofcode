use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, Read};

fn parse_state(input: &str) -> VecDeque<u32> {
    input
        .split("\n")
        .skip(1)
        .map(|l| l.parse().unwrap())
        .collect()
}

fn calculate_score(cards: &VecDeque<u32>) -> u32 {
    cards
        .iter()
        .zip((1..=cards.len()).rev())
        .map(|(c, i)| c * i as u32)
        .sum()
}

fn play(mut state: Vec<VecDeque<u32>>, can_recurse: bool) -> Vec<VecDeque<u32>> {
    let mut seen: HashSet<VecDeque<u32>> = HashSet::new();
    while !state[0].is_empty() && !state[1].is_empty() {
        if seen.contains(&state[0]) {
            return vec![state[0].clone(), VecDeque::new()];
        }
        seen.insert(state[0].clone());

        let entries = state
            .iter_mut()
            .map(|p| p.pop_front().unwrap())
            .collect::<Vec<_>>();

        let should_recurse = can_recurse
            && entries[0] <= state[0].len() as u32
            && entries[1] <= state[1].len() as u32;
        let winner_idx = if should_recurse {
            let recursive_result = play(
                vec![
                    state[0].iter().cloned().take(entries[0] as usize).collect(),
                    state[1].iter().cloned().take(entries[1] as usize).collect(),
                ],
                can_recurse,
            );
            recursive_result[0].is_empty() as usize
        } else {
            (entries[1] > entries[0]) as usize
        };

        if winner_idx == 0 {
            state[0].push_back(entries[0]);
            state[0].push_back(entries[1]);
        } else {
            state[1].push_back(entries[1]);
            state[1].push_back(entries[0]);
        }
    }
    state
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;

    let initial_state = input.split("\n\n").map(parse_state).collect::<Vec<_>>();

    let result = play(initial_state.clone(), false);
    let winner_idx = if result[0].is_empty() { 1 } else { 0 };
    let score = calculate_score(&result[winner_idx]);
    println!("(1) Player {} won with score {}", winner_idx + 1, score);

    let recursive_result = play(initial_state.clone(), true);
    let winner_idx = if recursive_result[0].is_empty() { 1 } else { 0 };
    let score = calculate_score(&recursive_result[winner_idx]);
    println!("(2) Player {} won with score {}", winner_idx + 1, score);

    Ok(())
}

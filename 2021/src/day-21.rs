use itertools::iproduct;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead};

fn simple_game(mut positions: Vec<usize>) -> (usize, Vec<usize>) {
    let mut scores = vec![0; positions.len()];

    let mut die = (1..=100).cycle();
    let mut die_rolls = 0;
    for p in (0..2).cycle() {
        let throw = die.next().unwrap() + die.next().unwrap() + die.next().unwrap();
        die_rolls += 3;

        positions[p] += throw;
        if positions[p] > 10 {
            positions[p] = (positions[p] - 1) % 10 + 1;
        }
        scores[p] += positions[p];
        if scores[p] >= 1000 {
            break;
        }
    }
    (die_rolls, scores)
}

type GameCache = HashMap<((usize, usize), (usize, usize), bool), (usize, usize)>;

fn quantum_game(
    cache: &mut GameCache,
    positions: (usize, usize),
    scores: (usize, usize),
    turn: bool,
) -> (usize, usize) {
    if let Some(result) = cache.get(&(positions, scores, turn)) {
        return *result;
    }

    let mut outcomes = (0, 0);
    for throw in iproduct!(1..=3, 1..=3, 1..=3) {
        let (mut next_positions, mut next_scores) = (positions, scores);
        let (player_position, player_score) = if turn {
            (&mut next_positions.1, &mut next_scores.1)
        } else {
            (&mut next_positions.0, &mut next_scores.0)
        };

        *player_position += throw.0 + throw.1 + throw.2;
        if *player_position > 10 {
            *player_position = (*player_position - 1) % 10 + 1;
        }
        *player_score += *player_position;
        if *player_score >= 21 {
            *(if turn {
                &mut outcomes.1
            } else {
                &mut outcomes.0
            }) += 1;
        } else {
            let recursive_outcomes = quantum_game(cache, next_positions, next_scores, !turn);
            outcomes.0 += recursive_outcomes.0;
            outcomes.1 += recursive_outcomes.1;
        }
    }

    cache.insert((positions, scores, turn), outcomes);
    outcomes
}

fn main() -> io::Result<()> {
    let input_re = Regex::new(r"^Player \d+ starting position: (\d+)$").unwrap();
    let positions: Vec<usize> = io::stdin()
        .lock()
        .lines()
        .flatten()
        .flat_map(|line| {
            input_re
                .captures(&line)
                .and_then(|re_match| re_match[1].parse().ok())
        })
        .collect();

    let (die_rolls, scores) = simple_game(positions.clone());
    let losing_score = scores.iter().min().unwrap();
    println!(
        "(1) Losing score is {} * {} die rolls = {}",
        losing_score,
        die_rolls,
        losing_score * die_rolls
    );

    let scores = quantum_game(
        &mut HashMap::new(),
        (positions[0], positions[1]),
        (0, 0),
        false,
    );
    println!(
        "(2) Player 1 won {} times, player 2 won {} times, max = {}",
        scores.0,
        scores.1,
        scores.0.max(scores.1)
    );

    Ok(())
}

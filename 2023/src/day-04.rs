use std::collections::HashSet;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Card {
    card_id: usize,
    value: usize,
}

fn main() {
    let mut cards = io::stdin()
        .lock()
        .lines()
        .flatten()
        .flat_map(|line| {
            let input_start = line.find(":").unwrap();
            let card_id = line["Card ".len()..input_start]
                .trim()
                .parse::<usize>()
                .unwrap();

            if let [ours, winners] = &line[input_start + 1..]
                .split("|")
                .map(|str| {
                    str.split(' ')
                        .flat_map(|num| num.parse::<u32>())
                        .collect::<HashSet<_>>()
                })
                .collect::<Vec<_>>()[..]
            {
                Some(Card {
                    card_id,
                    value: ours.intersection(&winners).count(),
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let score: usize = cards.iter().map(|c| (1 << c.value) >> 1).sum();
    println!(
        "(1) The sum of the scores of all winning cards is {}",
        score
    );

    let mut i = 0;
    while i < cards.len() {
        let card = &cards[i];
        let card_id = card.card_id;
        for j in 0..card.value {
            cards.push(cards[card_id + j].clone());
        }
        i += 1;
    }
    println!("(2) The game finishes with {} cards", cards.len());
}

use std::io::{self, BufRead};
use std::mem;

type Hand = [u8; 5];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn parse_line(s: &str) -> Option<(Hand, u32)> {
    s.split_once(' ').map(|(hand, bid)| {
        (
            // A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
            hand.chars()
                .map(|c| match c {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 11,
                    'T' => 10,
                    _ => c as u8 - b'0',
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            bid.parse().unwrap(),
        )
    })
}

fn get_hand_type(hand: &[u8; 5], allow_joker: bool) -> Type {
    let mut counts = [0; 15];
    for card in hand {
        counts[*card as usize] += 1;
    }

    // Extract the jokers before we sort
    let jokers = if allow_joker {
        mem::take(&mut counts[11])
    } else {
        0
    };

    counts.sort_by(|a, b| b.cmp(a));
    counts[0] += jokers;

    // Only need to look at the top 3 counts to decide
    return match counts[0..3] {
        [5, 0, 0] => Type::FiveOfAKind,
        [4, 1, 0] => Type::FourOfAKind,
        [3, 2, 0] => Type::FullHouse,
        [3, 1, 1] => Type::ThreeOfAKind,
        [2, 2, 1] => Type::TwoPair,
        [2, 1, 1] => Type::OnePair,
        [1, 1, 1] => Type::HighCard,
        _ => panic!("Unexpected {:?}", &counts[0..3]),
    };
}

fn filter_joker(cards: &[u8; 5]) -> [u8; 5] {
    let mut cards = *cards;
    for c in cards.iter_mut() {
        if *c == 11 {
            *c = 1;
        }
    }
    cards
}

fn score(input: &Vec<(Hand, u32)>, allow_joker: bool) -> u32 {
    let mut ranked_hands = input
        .iter()
        .map(|(hand, bid)| {
            (
                if allow_joker {
                    filter_joker(hand)
                } else {
                    *hand
                },
                get_hand_type(hand, allow_joker),
                *bid,
            )
        })
        .collect::<Vec<(Hand, Type, u32)>>();

    ranked_hands.sort_by(|(hand_a, type_a, _), (hand_b, type_b, _)| {
        if type_a != type_b {
            type_b.cmp(type_a)
        } else {
            hand_a.iter().cmp(hand_b.iter())
        }
    });

    ranked_hands
        .iter()
        .enumerate()
        .map(|(pos, (_, _, bid))| (pos + 1) as u32 * *bid)
        .sum()
}

fn main() {
    let input = io::stdin()
        .lock()
        .lines()
        .flatten()
        .flat_map(|str| parse_line(&str))
        .collect::<Vec<_>>();
    println!("(1) Score is {}", score(&input, false));
    println!("(2) Score with jokers is {}", score(&input, true));
}

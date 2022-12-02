use std::io::{self, BufRead};

#[derive(Copy, Clone, Debug, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissor,
}

impl Move {
    pub fn from_str(c: &str) -> Move {
        match c {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissor,
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissor,
            _ => panic!("Unexpected char {}", c),
        }
    }

    fn from_idx(idx: i32) -> Move {
        match idx {
            0 => Move::Rock,
            1 => Move::Paper,
            2 => Move::Scissor,
            _ => panic!("Index out of range {}", idx),
        }
    }

    pub fn winning_move(&self) -> Move {
        Self::from_idx(((*self as i32) + 1).rem_euclid(3))
    }

    pub fn losing_move(&self) -> Move {
        Self::from_idx(((*self as i32) - 1).rem_euclid(3))
    }
}

enum Outcome {
    Lose,
    Win,
    Draw,
}

impl Outcome {
    pub fn from_str(c: &str) -> Outcome {
        match c {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Unexpected char {}", c),
        }
    }

    pub fn get_desired_move(&self, opp_move: &Move) -> Move {
        match self {
            Outcome::Win => opp_move.winning_move(),
            Outcome::Lose => opp_move.losing_move(),
            Outcome::Draw => *opp_move,
        }
    }
}

fn score_turn(my_move: &Move, opp_move: &Move) -> u32 {
    (match my_move {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissor => 3,
    }) + (match (my_move, opp_move) {
        (a, b) if a == &b.winning_move() => 6,
        (a, b) if a == b => 3,
        _ => 0,
    })
}

fn main() {
    let mut part_one_score = 0;
    let mut part_two_score = 0;

    for line in io::stdin().lock().lines().flatten() {
        if let [column_one, column_two] = &line.split(' ').collect::<Vec<&str>>()[..] {
            let opp_move = Move::from_str(column_one);
            part_one_score += score_turn(&Move::from_str(column_two), &opp_move);
            part_two_score += score_turn(
                &Outcome::from_str(column_two).get_desired_move(&opp_move),
                &opp_move,
            );
        }
    }

    println!("(1) Score is {}", part_one_score);
    println!("(2) Score is {}", part_two_score);
}

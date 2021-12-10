use lazy_static::lazy_static;
use std::collections::HashMap;
use std::io::{self, BufRead};

lazy_static! {
    static ref ERROR_SCORING: HashMap<char, u32> =
        HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137),]);
    static ref AUTOCOMPLETE_SCORING: HashMap<char, u64> =
        HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4),]);
}

fn parse_to_stack(line: &str) -> Result<Vec<char>, char> {
    let mut stack = vec![];
    for c in line.chars() {
        let expected = match c {
            ')' => Some('('),
            ']' => Some('['),
            '}' => Some('{'),
            '>' => Some('<'),
            _ => {
                stack.push(c);
                None
            }
        };
        if let Some(expected) = expected {
            if stack.pop().unwrap() != expected {
                return Err(c);
            }
        }
    }
    Ok(stack)
}

fn complete_stack(stack: &Vec<char>) -> Vec<char> {
    stack
        .iter()
        .rev()
        .map(|c| match c {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => panic!("Unexpected char {}", c),
        })
        .collect()
}

fn main() -> io::Result<()> {
    let mut syntax_error_score = 0;
    let mut autocompletion_scores = vec![];

    for line in io::stdin().lock().lines().flatten() {
        match parse_to_stack(&line) {
            Err(c) => {
                syntax_error_score += ERROR_SCORING.get(&c).unwrap();
            }
            Ok(stack) => {
                autocompletion_scores.push(
                    complete_stack(&stack)
                        .iter()
                        .fold(0, |acc, c| acc * 5 + AUTOCOMPLETE_SCORING.get(&c).unwrap()),
                );
            }
        }
    }
    autocompletion_scores.sort();

    println!("(1) The total syntax error score is {}", syntax_error_score);
    println!(
        "(2) The total autocompletion score is {:?}",
        autocompletion_scores[autocompletion_scores.len() / 2]
    );

    Ok(())
}

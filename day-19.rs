use std::collections::HashMap;
use std::collections::VecDeque;
// use std::iter::FromIterator;
use std::io::{self, Read};

#[derive(Debug)]
enum Rule {
    Seq(Vec<usize>),
    Or(Vec<usize>, Vec<usize>),
    Char(char),
}

type Rules = HashMap<usize, Rule>;

fn validate_message(rules: &Rules, message: &str, mut to_match: VecDeque<usize>) -> bool {
    if message.len() == 0 || to_match.len() == 0 {
        return message.len() == to_match.len();
    }

    let rule = &rules[&to_match.pop_front().unwrap()];
    match rule {
        Rule::Seq(sub_rules) => {
            for rule in sub_rules.iter().rev() {
                to_match.push_front(*rule);
            }
            validate_message(rules, message, to_match)
        }
        Rule::Or(option_a, option_b) => {
            let to_match_a = option_a.iter().chain(to_match.iter()).copied().collect();
            if validate_message(rules, message, to_match_a) {
                true
            } else {
                let to_match_b = option_b.iter().chain(to_match.iter()).copied().collect();
                validate_message(rules, message, to_match_b)
            }
        }
        Rule::Char(c) => {
            if *c == message.chars().next().unwrap() {
                validate_message(rules, &message[1..], to_match)
            } else {
                false
            }
        }
    }
}

fn parse_rule(input: &str) -> Rule {
    if input.contains('|') {
        let mut res: Vec<Vec<usize>> = input
            .split(" | ")
            .map(|or| {
                or.split(' ')
                    .flat_map(|n| n.parse::<usize>().ok())
                    .collect::<Vec<usize>>()
            })
            .collect();
        Rule::Or(res.pop().unwrap(), res.pop().unwrap())
    } else if input.contains('"') {
        Rule::Char(input.chars().nth(1).unwrap())
    } else {
        Rule::Seq(
            input
                .split(' ')
                .flat_map(|n| n.parse::<usize>().ok())
                .collect(),
        )
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;

    if let [rules, messages] = input.split("\n\n").collect::<Vec<&str>>()[..] {
        let mut rule_lookup: Rules = HashMap::new();
        for rule in rules.split("\n") {
            if let [key, value] = rule.split(": ").collect::<Vec<&str>>()[..] {
                rule_lookup.insert(key.parse().unwrap(), parse_rule(value));
            }
        }
        let valid_count = messages
            .split("\n")
            .filter(|message| validate_message(&rule_lookup, &message, VecDeque::from(vec![0])))
            .count();
        println!("There are {} valid messages", valid_count);
    }

    Ok(())
}

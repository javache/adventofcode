use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Read};
use std::ops::RangeInclusive;

type Rule = Vec<RangeInclusive<i32>>;
type Rules = HashMap<String, Rule>;

fn parse_ticket(input: &str) -> Vec<i32> {
    input
        .split(",")
        .flat_map(|n| n.parse::<i32>().ok())
        .collect()
}

fn parse_rules(input: &str) -> Rules {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?m)^([\w\s]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    }
    let mut result = HashMap::new();
    for captures in RE.captures_iter(input) {
        let numbers: Vec<i32> = captures
            .iter()
            .skip(2)
            .flat_map(|m| m.and_then(|n| n.as_str().parse().ok()))
            .collect();
        result.insert(
            captures[1].to_string(),
            vec![numbers[0]..=numbers[1], numbers[2]..=numbers[3]],
        );
    }
    result
}

fn matches_rule(rule: &Rule, input: &i32) -> bool {
    rule.iter().any(|range| range.contains(input))
}

fn find_ordering(rules: &Rules, tickets: &Vec<Vec<i32>>, ordering: &mut Vec<String>) -> bool {
    let current_idx = ordering.len();
    if current_idx == rules.len() {
        return true;
    }

    for (rule_name, rule) in rules {
        if ordering.contains(rule_name) {
            continue;
        }

        let is_valid = tickets
            .iter()
            .all(|ticket| matches_rule(rule, &ticket[current_idx]));
        if is_valid {
            ordering.push(rule_name.clone());
            if find_ordering(rules, tickets, ordering) {
                return true;
            }
            ordering.pop();
        }
    }
    false
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;

    let mut rules: Rules = HashMap::new();
    let mut error_rate: i32 = 0;
    let mut my_ticket: Vec<i32> = Vec::new();
    let mut valid_tickets: Vec<Vec<i32>> = Vec::new();

    for section in input.split("\n\n") {
        if rules.is_empty() {
            rules = parse_rules(section);
        } else if section.starts_with("your ticket:") {
            my_ticket = section.split("\n").nth(1).map(parse_ticket).unwrap();
        } else if section.starts_with("nearby tickets:") {
            for ticket in section.split("\n").skip(1).map(parse_ticket) {
                let invalid_sum: i32 = ticket
                    .iter()
                    .filter(|n| !rules.values().any(|rule| matches_rule(rule, *n)))
                    .sum();
                if invalid_sum == 0 {
                    valid_tickets.push(ticket);
                } else {
                    error_rate += invalid_sum;
                }
            }
        }
    }
    println!("(1) Ticket scanning error rate = {}", error_rate);

    let mut ordering = Vec::new();
    find_ordering(&rules, &valid_tickets, &mut ordering);
    let product: i64 = ordering
        .iter()
        .enumerate()
        .filter(|(_, rule)| rule.starts_with("departure"))
        .map(|(idx, _)| my_ticket[idx] as i64)
        .product();
    println!("(2) Product of departure fields = {}", product);

    Ok(())
}

use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Read};
use std::ops::RangeInclusive;

type Part = [i32; 4];
type PartRange = [RangeInclusive<i32>; 4];

#[derive(Debug)]
struct Rule {
    field: usize,
    operator: char,
    value: i32,
    dest: String,
}

type Rules = HashMap<String, Vec<Rule>>;

fn parse_rules(input: &str) -> Vec<Rule> {
    input
        .split(',')
        .map(|rule_desc| {
            let dest_seperator = rule_desc.find(':');
            if let Some(dest_seperator) = dest_seperator {
                let mut chars = rule_desc.chars();
                Rule {
                    field: match chars.next().unwrap() {
                        'x' => 0,
                        'm' => 1,
                        'a' => 2,
                        's' => 3,
                        _ => unreachable!(),
                    },
                    operator: chars.next().unwrap(),
                    value: rule_desc[2..dest_seperator].parse().unwrap(),
                    dest: rule_desc[(dest_seperator + 1)..].to_string(),
                }
            } else {
                Rule {
                    field: 0,
                    operator: '!',
                    value: 0,
                    dest: rule_desc.to_string(),
                }
            }
        })
        .collect()
}

fn is_accepted(all_rules: &Rules, part: &Part) -> bool {
    let mut workflow: &str = &"in";
    while !["A", "R"].contains(&workflow) {
        for rule in all_rules.get(workflow).unwrap() {
            let field = part[rule.field];
            if match rule.operator {
                '!' => true,
                '<' => field < rule.value,
                '>' => field > rule.value,
                _ => unreachable!(),
            } {
                workflow = &rule.dest;
                break;
            }
        }
    }
    workflow == "A"
}

fn split_part_range(part_range: &PartRange, idx: usize, pivot: i32) -> (PartRange, PartRange) {
    let mut left = part_range.clone();
    let mut right = part_range.clone();
    left[idx] = (*part_range[idx].start())..=(pivot - 1);
    right[idx] = pivot..=(*part_range[idx].end());
    (left, right)
}

fn find_accepted_ranges(
    all_rules: &Rules,
    part_range: PartRange,
    curr_workflow: &str,
) -> Vec<PartRange> {
    let rules = all_rules.get(curr_workflow).unwrap();
    rules
        .iter()
        .fold(vec![(part_range, None)], |states, rule| {
            states
                .into_iter()
                .flat_map(|(range, workflow)| {
                    if workflow.is_none() {
                        match rule.operator {
                            '<' if *range[rule.field].end() > rule.value => {
                                let (left, right) =
                                    split_part_range(&range, rule.field, rule.value);
                                vec![(left, Some(&rule.dest[..])), (right, None)]
                            }
                            '>' if *range[rule.field].start() < rule.value => {
                                let (left, right) =
                                    split_part_range(&range, rule.field, rule.value + 1);
                                vec![(left, None), (right, Some(&rule.dest[..]))]
                            }
                            _ => vec![(range, Some(&rule.dest[..]))],
                        }
                    } else {
                        vec![(range, workflow)]
                    }
                })
                .collect()
        })
        .into_iter()
        .flat_map(|(range, workflow)| match workflow {
            Some("A") => vec![range],
            Some("R") => vec![],
            Some(workflow) => find_accepted_ranges(all_rules, range, workflow),
            None => unreachable!(),
        })
        .collect()
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;

    let mut rules: Rules = HashMap::new();
    let mut parts: Vec<Part> = vec![];
    if let [rules_input, parts_input] = input.split("\n\n").collect::<Vec<&str>>()[..] {
        for line in rules_input.lines() {
            let rule_start = line.find('{').unwrap();
            rules.insert(
                line[..rule_start].to_string(),
                parse_rules(&line[rule_start + 1..(line.len() - 1)]),
            );
        }

        let numbers_re = Regex::new("[0-9]+").unwrap();
        for line in parts_input.lines() {
            let v = numbers_re
                .find_iter(&line)
                .map(|m| m.as_str().parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            parts.push(v.try_into().unwrap());
        }
    }

    let accepted_parts = parts
        .iter()
        .filter(|p| is_accepted(&rules, &p))
        .collect::<Vec<_>>();

    println!(
        "(1) Accepted parts sum up to {}",
        accepted_parts
            .iter()
            .map(|p| p.iter().sum::<i32>())
            .sum::<i32>()
    );

    let accepted_ranges = find_accepted_ranges(
        &rules,
        [(1..=4000), (1..=4000), (1..=4000), (1..=4000)],
        "in",
    );
    println!(
        "(2) Accepted ranges give a total of {} combinations",
        accepted_ranges
            .iter()
            .map(|part_range| part_range
                .iter()
                .map(|r| (r.end() - r.start() + 1) as i64)
                .product::<i64>())
            .sum::<i64>()
    );

    Ok(())
}

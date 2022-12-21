use itertools::Itertools;
use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
enum Job {
    Number(i64),
    Operation(String, char, String),
}

type Input = HashMap<String, Job>;

fn evaluate(name: &str, monkeys: &Input) -> i64 {
    match &monkeys[name] {
        Job::Number(n) => *n,
        Job::Operation(a, '+', b) => evaluate(&a, monkeys) + evaluate(&b, monkeys),
        Job::Operation(a, '*', b) => evaluate(&a, monkeys) * evaluate(&b, monkeys),
        Job::Operation(a, '-', b) => evaluate(&a, monkeys) - evaluate(&b, monkeys),
        Job::Operation(a, '/', b) => evaluate(&a, monkeys) / evaluate(&b, monkeys),
        Job::Operation(_, op, _) => panic!("Unsupported operation {}", op),
    }
}

fn dependency_path(name: &str, dep: &str, monkeys: &Input) -> Option<Vec<String>> {
    match &monkeys[name] {
        Job::Number(_) => (name == dep).then(|| vec![name.to_string()]),
        Job::Operation(a, _, b) => dependency_path(a, dep, monkeys)
            .or_else(|| dependency_path(b, dep, monkeys))
            .map(|mut path| {
                path.push(name.to_string());
                path
            }),
    }
}

fn solve_for_human(a: &str, b: &str, monkeys: &Input) -> i64 {
    let (path_a, path_b) = (
        dependency_path(a, "humn", &monkeys),
        dependency_path(b, "humn", &monkeys),
    );
    let mut target_value = evaluate(if path_a.is_some() { b } else { a }, &monkeys);
    for (curr, next) in path_a.or(path_b).unwrap().iter().rev().tuple_windows() {
        match &monkeys[curr] {
            Job::Operation(a, '+', b) | Job::Operation(b, '+', a) if a == next => {
                target_value -= evaluate(b, &monkeys)
            }
            Job::Operation(a, '*', b) | Job::Operation(b, '*', a) if a == next => {
                target_value /= evaluate(b, &monkeys)
            }
            Job::Operation(a, '-', b) if a == next => target_value += evaluate(b, &monkeys),
            Job::Operation(b, '-', a) if a == next => {
                target_value = evaluate(b, &monkeys) - target_value
            }
            Job::Operation(a, '/', b) if a == next => target_value *= evaluate(b, &monkeys),
            Job::Operation(b, '/', a) if a == next => {
                target_value = evaluate(b, &monkeys) / target_value
            }
            job => panic!("Unsupported job: {:?}", job),
        }
    }
    target_value
}

fn main() {
    let input: Input = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| {
            let (name, job) = line.split(": ").collect_tuple().unwrap();
            (
                name.to_string(),
                if let Ok(num) = job.parse::<i64>() {
                    Job::Number(num)
                } else {
                    let parts = job.split(' ').collect_tuple::<(_, _, _)>().unwrap();
                    Job::Operation(
                        parts.0.to_string(),
                        parts.1.chars().next().unwrap(),
                        parts.2.to_string(),
                    )
                },
            )
        })
        .collect();

    println!("(1) Monkey root yells {}", evaluate("root", &input));

    if let Job::Operation(a, _, b) = &input["root"] {
        println!(
            "(2) Value for human should be {}",
            solve_for_human(a, b, &input)
        );
    }
}

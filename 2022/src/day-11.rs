use std::collections::BinaryHeap;
use std::error;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add(u64),
    Mul(u64),
    Sqr,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    op: Operation,
    divisor: u64,
    throws: (usize, usize),
}

fn parse_next<T: FromStr>(line: &str, separator: &str) -> Result<T, T::Err> {
    line.split(separator)
        .skip(1)
        .next()
        .map(|s| s.parse::<T>())
        .unwrap()
}

impl FromStr for Monkey {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.split('\n').collect();
        let items = parse_next::<String>(lines[1], ": ")
            .map(|s| s.split(',').map(|n| n.trim().parse()).flatten().collect())?;
        let op = parse_next::<String>(lines[2], "new = ").map(|s| {
            match &s.split(' ').collect::<Vec<_>>()[..] {
                ["old", "*", "old"] => Operation::Sqr,
                ["old", "*", n] => Operation::Mul(n.parse().unwrap()),
                ["old", "+", n] => Operation::Add(n.parse().unwrap()),
                _ => panic!("Unexpected op {}", s),
            }
        })?;
        Ok(Monkey {
            items,
            op,
            divisor: parse_next(lines[3], "divisible by ")?,
            throws: (
                parse_next(lines[4], "to monkey ")?,
                parse_next(lines[5], "to monkey ")?,
            ),
        })
    }
}

fn count_inspections(mut monkeys: Vec<Monkey>, rounds: usize, relief_divisor: u64) -> Vec<usize> {
    // All divisors are prime, so we can just take the product
    // otherwise, we should calculate the lowest-common-multiple
    let lcm: u64 = monkeys.iter().map(|m| m.divisor).product();

    let mut inspections = vec![0; monkeys.len()];
    for _ in 0..rounds {
        for j in 0..monkeys.len() {
            let items: Vec<_> = monkeys[j].items.drain(0..).collect();
            for item in items {
                inspections[j] += 1;

                let monkey = &monkeys[j];
                let new_value = match monkey.op {
                    Operation::Mul(n) => item * n,
                    Operation::Add(n) => item + n,
                    Operation::Sqr => item * item,
                } / relief_divisor;

                let next_monkey = if new_value % monkey.divisor == 0 {
                    monkey.throws.0
                } else {
                    monkey.throws.1
                };
                monkeys[next_monkey].items.push(new_value % lcm);
            }
        }
    }
    inspections
}

fn top_2_product(input: Vec<usize>) -> usize {
    let mut heap = BinaryHeap::from(input);
    (0..2).map(|_| heap.pop().unwrap()).product()
}

fn main() -> std::io::Result<()> {
    let mut input: String = String::new();
    io::stdin().lock().read_to_string(&mut input)?;

    let monkeys: Vec<Monkey> = input.split("\n\n").map(str::parse).flatten().collect();
    println!(
        "(1) Level of monkey business after 20 rounds is {}",
        top_2_product(count_inspections(monkeys.clone(), 20, 3)),
    );
    println!(
        "(2) Level of monkey business after 10.000 rounds is {}",
        top_2_product(count_inspections(monkeys.clone(), 10_000, 1)),
    );

    Ok(())
}

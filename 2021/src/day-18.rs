use itertools::Itertools;
use std::fmt;
use std::io::{self, BufRead};
use std::str;

#[derive(Clone)]
enum SnailNum {
    Value(u8),
    Pair(Box<SnailNum>, Box<SnailNum>),
}

#[derive(Debug, PartialEq)]
enum ExplodeResult {
    Explode(u8, u8),
    ExplodeLeft(u8),
    ExplodeRight(u8),
    Changed,
    None,
}

impl SnailNum {
    fn parse<'a>(input: &'a mut str::Chars<'a>) -> (SnailNum, &'a mut str::Chars<'a>) {
        let c = input.next().unwrap();
        if c == '[' {
            let (val1, input) = SnailNum::parse(input);
            assert_eq!(input.next(), Some(','));
            let (val2, input) = SnailNum::parse(input);
            assert_eq!(input.next(), Some(']'));
            (SnailNum::Pair(Box::new(val1), Box::new(val2)), input)
        } else {
            (SnailNum::Value((c as u8) - b'0'), input)
        }
    }

    fn value(&self) -> Option<u8> {
        if let SnailNum::Value(val) = *self {
            Some(val)
        } else {
            None
        }
    }

    fn add_to_leftmost_node(&mut self, to_add: u8) {
        match self {
            SnailNum::Pair(left, _) => left.add_to_leftmost_node(to_add),
            SnailNum::Value(value) => *value += to_add,
        }
    }

    fn add_to_rightmost_node(&mut self, to_add: u8) {
        match self {
            SnailNum::Pair(_, right) => right.add_to_rightmost_node(to_add),
            SnailNum::Value(value) => *value += to_add,
        }
    }

    fn explode(&mut self, depth: u8) -> ExplodeResult {
        match self {
            SnailNum::Value(_) => ExplodeResult::None,
            SnailNum::Pair(left, right) => {
                if depth > 4 {
                    if let (Some(left_val), Some(right_val)) = (left.value(), right.value()) {
                        *self = SnailNum::Value(0);
                        return ExplodeResult::Explode(left_val, right_val);
                    } else {
                        panic!("Unexpected num {} at depth {}", self, depth);
                    }
                }
                match left.explode(depth + 1) {
                    ExplodeResult::Explode(to_add_left, to_add_right) => {
                        right.add_to_leftmost_node(to_add_right);
                        return ExplodeResult::ExplodeLeft(to_add_left);
                    }
                    ExplodeResult::ExplodeRight(to_add) => {
                        right.add_to_leftmost_node(to_add);
                        return ExplodeResult::Changed;
                    }
                    ExplodeResult::None => {
                        return match right.explode(depth + 1) {
                            ExplodeResult::Explode(to_add_left, to_add_right) => {
                                left.add_to_rightmost_node(to_add_left);
                                return ExplodeResult::ExplodeRight(to_add_right);
                            }
                            ExplodeResult::ExplodeLeft(to_add) => {
                                left.add_to_rightmost_node(to_add);
                                return ExplodeResult::Changed;
                            }
                            result => result,
                        }
                    }
                    result => result,
                }
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            SnailNum::Value(value) => {
                if *value >= 10 {
                    let (new_left, new_right) = (*value / 2, (*value + 1) / 2);
                    *self = SnailNum::Pair(
                        Box::new(SnailNum::Value(new_left)),
                        Box::new(SnailNum::Value(new_right)),
                    );
                    true
                } else {
                    false
                }
            }
            SnailNum::Pair(left, right) => left.split() || right.split(),
        }
    }

    fn combine(a: &SnailNum, b: &SnailNum) -> SnailNum {
        let mut combined = SnailNum::Pair(Box::new(a.clone()), Box::new(b.clone()));
        while combined.explode(1) != ExplodeResult::None || combined.split() {}
        combined
    }

    fn magnitude(&self) -> usize {
        match self {
            SnailNum::Value(value) => *value as usize,
            SnailNum::Pair(left, right) => left.magnitude() * 3 + right.magnitude() * 2,
        }
    }
}

impl fmt::Display for SnailNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SnailNum::Value(value) => write!(f, "{}", value),
            SnailNum::Pair(left, right) => write!(f, "[{},{}]", *left, *right),
        }
    }
}

fn main() -> io::Result<()> {
    let numbers = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| SnailNum::parse(&mut line.chars()).0)
        .collect::<Vec<SnailNum>>();

    let sum = numbers
        .iter()
        .fold(SnailNum::Value(0), |a, b| SnailNum::combine(&a, b));
    println!("(1) Magnitude of total sum is {}", sum.magnitude());

    let max_magnitude = numbers
        .iter()
        .permutations(2)
        .map(|p| SnailNum::combine(p[0], p[1]).magnitude())
        .max();
    println!("(2) Max magnitude of any sum is {}", max_magnitude.unwrap());

    Ok(())
}

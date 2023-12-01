use std::io::{self, BufRead};

const NUMBERS: &'static [&str] = &[
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine"
];

fn main() {
    let mut sum_one = 0;
    let mut sum_two = 0;

    for line in io::stdin().lock().lines().flatten() {
        let mut number_one = (None, 0);
        let mut number_two = (None, 0);

        let mut it = line.chars();
        while !it.as_str().is_empty() {
            // Grab the substr first, as we'll advance it afterwards
            let substr = it.as_str();
            
            let numeric_digit = it.next().and_then(|c| c.to_digit(10));
            let digit = numeric_digit.or_else(|| {
                let idx = NUMBERS.iter().position(|num| substr.starts_with(num));
                idx.map(|i| i as u32 + 1)
            });

            if let Some(numeric_digit) = numeric_digit {
                number_one = (number_one.0.or(Some(numeric_digit)), numeric_digit);
            }
            if let Some(digit) = digit {
                number_two = (number_two.0.or(Some(digit)), digit);
            }          
        }

        sum_one += number_one.0.unwrap() * 10 + number_one.1;
        sum_two += number_two.0.unwrap() * 10 + number_two.1;
    }

    println!("(1) Sum of all calibration values is {}", sum_one);
    println!("(2) Sum of all calibration values is {}", sum_two);
}

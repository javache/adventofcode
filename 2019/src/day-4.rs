use std::io::{self, BufRead};
use std::ops::RangeInclusive;

fn count_options(
    base: u32,
    range: RangeInclusive<u32>,
    last_digit: u32,
    has_double: bool,
    allow_overruns: bool,
    current_run: u32,
) -> u32 {
    if base == 0 {
        return (has_double || current_run == 2) as u32;
    }

    let (min_digit, min_remainder) = (range.start() / base, range.start() % base);
    let (max_digit, max_remainder) = (range.end() / base, range.end() % base);

    let next_base = base / 10;
    (min_digit..=max_digit)
        .map(|digit| {
            let next_min = if digit == min_digit {
                min_remainder.max(digit * next_base)
            } else {
                digit * next_base
            };
            let next_max = if digit == max_digit {
                max_remainder
            } else {
                base - 1
            };
            let next_has_double = if allow_overruns {
                has_double || current_run >= 2
            } else {
                has_double || (digit != last_digit && current_run == 2)
            };
            count_options(
                next_base,
                next_min..=next_max,
                digit,
                next_has_double,
                allow_overruns,
                if digit == last_digit {
                    current_run + 1
                } else {
                    1
                },
            )
        })
        .sum()
}

fn main() -> io::Result<()> {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line)?;

    if let [lower, upper] = line
        .split('-')
        .flat_map(|n| n.parse())
        .collect::<Vec<u32>>()[..]
    {
        let base = (10 as u32).pow((line.len() / 2 - 1) as u32);
        println!(
            "(1) There are {} password options",
            count_options(base, lower..=upper, u32::MAX, false, true, 0)
        );
        println!(
            "(2) There are {} password options when not allowing for longer runs",
            count_options(base, lower..=upper, u32::MAX, false, false, 0)
        );
    }

    Ok(())
}

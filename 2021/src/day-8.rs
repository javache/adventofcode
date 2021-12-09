use itertools::Itertools;
use std::io::{self, BufRead};

const SEGMENTS: &'static [&[char]; 10] = &[
    &['a', 'b', 'c', 'e', 'f', 'g'],
    &['c', 'f'],
    &['a', 'c', 'd', 'e', 'g'],
    &['a', 'c', 'd', 'f', 'g'],
    &['b', 'c', 'd', 'f'],
    &['a', 'b', 'd', 'f', 'g'],
    &['a', 'b', 'd', 'e', 'f', 'g'],
    &['a', 'c', 'f'],
    &['a', 'b', 'c', 'd', 'e', 'f', 'g'],
    &['a', 'b', 'c', 'd', 'f', 'g'],
];

fn count_unique_segment_outputs(outputs: &Vec<&str>) -> usize {
    let unique_segments = [
        SEGMENTS[1].len(),
        SEGMENTS[4].len(),
        SEGMENTS[7].len(),
        SEGMENTS[8].len(),
    ];
    outputs
        .iter()
        .filter(|output| unique_segments.contains(&output.len()))
        .count()
}

fn find_mapping(signals: &Vec<&str>) -> Option<Vec<char>> {
    let mut signals: Vec<Vec<char>> = signals
        .iter()
        .map(|signal| signal.chars().collect())
        .collect();
    signals.iter_mut().for_each(|s| s.sort());

    ('a'..='g').permutations(7).find(|mapping| {
        let mut mapped_segments: Vec<Vec<char>> = SEGMENTS
            .iter()
            .map(|s| {
                s.iter()
                    .map(|c| mapping[(*c as u8 - b'a') as usize])
                    .collect()
            })
            .collect();
        mapped_segments.iter_mut().for_each(|s| s.sort());
        signals
            .iter()
            .all(|signal| mapped_segments.iter().any(|s| s == signal))
    })
}

fn decode_signal(signal: &str, mapping: &Vec<char>) -> usize {
    let mut chars: Vec<char> = signal
        .chars()
        .map(|c| {
            let index = mapping.iter().position(|m| *m == c).unwrap();
            (b'a' + index as u8) as char
        })
        .collect();
    chars.sort();
    SEGMENTS
        .iter()
        .position(|segment| *segment == chars)
        .unwrap()
}

fn main() -> io::Result<()> {
    let mut unique_segment_count = 0;
    let mut output_sum = 0;

    for line in io::stdin().lock().lines().flatten() {
        if let [signals, outputs] = &line
            .split(" | ")
            .map(|readings| readings.split(' ').collect())
            .collect::<Vec<Vec<&str>>>()[..]
        {
            unique_segment_count += count_unique_segment_outputs(outputs);

            if let Some(mapping) = find_mapping(signals) {
                output_sum += outputs
                    .iter()
                    .fold(0, |acc, output| acc * 10 + decode_signal(output, &mapping));
            }
        }
    }

    println!(
        "(1) There are {} numbers that use unique segments (1, 4, 7 or 8)",
        unique_segment_count
    );
    println!("(2) The sum of all outputs is {}", output_sum);

    Ok(())
}

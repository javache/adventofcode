use std::collections::HashMap;
use std::io::{self, BufRead};

type Cache = HashMap<(usize, usize), usize>;

// Must be called on a valid boundary position (? or #)
fn solve(input: &str, spec: &[usize], cache: &mut Cache) -> usize {
    if input.is_empty() {
        return 0;
    }

    if let Some(result) = cache.get(&(input.len(), spec.len())) {
        *result
    } else {
        let result = try_skip(input, spec, cache) + try_match(input, spec, cache);
        cache.insert((input.len(), spec.len()), result);
        result
    }
}

// Path a: treat ? as a .
fn try_skip(input: &str, spec: &[usize], cache: &mut Cache) -> usize {
    if input.starts_with('?') {
        solve(&input[1..].trim_start_matches('.'), spec, cache)
    } else {
        0
    }
}

fn try_match(input: &str, spec: &[usize], cache: &mut Cache) -> usize {
    let seq_len = spec[0];

    // Sequence can't be matched with the next characters
    if input.len() < seq_len || input[0..seq_len].contains('.') {
        return 0;
    }

    // Completed the solution, validate there's no more # remaining
    if spec.len() == 1 {
        let has_remaining_symbols = input[seq_len..].contains('#');
        return !has_remaining_symbols as usize;
    }

    // Validate there's a . or ? following, otherwise we can't complete this match
    if input[seq_len..].starts_with("#") || input.len() < seq_len + 2 {
        return 0;
    }

    solve(
        &input[(seq_len + 1)..].trim_start_matches('.'),
        &spec[1..],
        cache,
    )
}

fn main() {
    let (mut sum_a, mut sum_b) = (0, 0);
    for line in io::stdin().lock().lines().flatten() {
        if let Some((input, spec)) = line.split_once(' ') {
            let spec = spec
                .split(',')
                .flat_map(|s| s.parse::<usize>())
                .collect::<Vec<_>>();

            let mut cache = HashMap::new();
            sum_a += solve(input.trim_matches('.'), &spec[..], &mut cache);
            sum_b += solve(
                [input; 5].join("?").trim_matches('.'),
                &[&spec[..]; 5].concat()[..],
                &mut cache,
            );
        }
    }

    println!("(1) The total number of solutions is {}", sum_a);
    println!("(2) The total number of solutions is {}", sum_b);
}

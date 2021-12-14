use itertools::Itertools;
use std::collections::HashMap;
use std::io::{self, BufRead};

type Rules = HashMap<(char, char), char>;
type PairCount = HashMap<(char, char), usize>;

fn apply_rules(rules: &Rules, pairs: &PairCount) -> PairCount {
    let mut result: PairCount = HashMap::new();
    for (pair, count) in pairs {
        if let Some(insertion) = rules.get(&pair) {
            *result.entry((pair.0, *insertion)).or_default() += count;
            *result.entry((*insertion, pair.1)).or_default() += count;
        }
    }
    result
}

fn solve(input: &str, rules: &Rules, iterations: usize) -> usize {
    let mut pairs: PairCount = HashMap::new();
    input
        .chars()
        .tuple_windows::<(_, _)>()
        .for_each(|pair| *pairs.entry(pair).or_default() += 1);

    let result = (0..iterations).fold(pairs, |state, _| apply_rules(&rules, &state));

    let first_char = input.chars().next().unwrap();
    // Add an entry first char to avoid under-counting it when adding up pairs
    let mut freq: HashMap<char, usize> = HashMap::from([(first_char, 1)]);
    result
        .iter()
        .for_each(|((_, c), count)| *freq.entry(*c).or_default() += count);

    let (max_char, _) = freq.iter().max_by_key(|(_, v)| *v).unwrap();
    let (min_char, _) = freq.iter().min_by_key(|(_, v)| *v).unwrap();
    freq[max_char] - freq[min_char]
}

fn main() -> io::Result<()> {
    let mut input: String = String::new();
    let mut rules: Rules = HashMap::new();

    for line in io::stdin().lock().lines().flatten() {
        if let [pair, output] = line.split(" -> ").collect::<Vec<&str>>()[..] {
            rules.insert(
                pair.chars().next_tuple().unwrap(),
                output.chars().next().unwrap(),
            );
        } else if line.len() > 0 {
            input = line;
        }
    }

    println!(
        "(1) After running 10 iterations, the value is {}",
        solve(&input, &rules, 10)
    );
    println!(
        "(2) After running 40 iterations, the value is {}",
        solve(&input, &rules, 40)
    );

    Ok(())
}

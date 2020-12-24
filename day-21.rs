use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn parse_line(input: &str) -> (HashSet<String>, HashSet<String>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([\w\s]+) \(contains ([\w\s,]+)\)$").unwrap();
    }
    RE.captures_iter(input)
        .next()
        .map(|captures| {
            (
                captures[1]
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect(),
                captures[2].split(", ").map(|s| s.to_string()).collect(),
            )
        })
        .unwrap()
}

fn map_allergens_to_ingredients<'a>(
    input: &'a Vec<(HashSet<String>, HashSet<String>)>,
) -> HashMap<&str, HashSet<&str>> {
    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (ingredients, allergens) in input {
        let ingredients: HashSet<&str> = ingredients.iter().map(|i| i.as_ref()).collect();
        for allergen in allergens {
            map.entry(allergen)
                .and_modify(|s| *s = s.intersection(&ingredients).cloned().collect())
                .or_insert(ingredients.clone());
        }
    }
    map
}

fn reduce_map<'a>(mapping: &'a mut HashMap<&str, HashSet<&str>>) -> BTreeMap<&'a str, &'a str> {
    let mut solution: BTreeMap<&str, &str> = BTreeMap::new();
    while !mapping.is_empty() {
        let mut to_remove = Vec::new();
        mapping.retain(|allergen, ingredients| {
            if let [ingredient] = ingredients.iter().collect::<Vec<_>>()[..] {
                to_remove.push(*ingredient);
                solution.insert(allergen, ingredient);
                false
            } else {
                true
            }
        });
        for ingredients in mapping.values_mut() {
            for el in &to_remove {
                ingredients.remove(el);
            }
        }
    }
    solution
}

fn main() -> io::Result<()> {
    let mut input = Vec::new();
    for line in io::stdin().lock().lines() {
        input.push(parse_line(&line?));
    }

    let mut map = map_allergens_to_ingredients(&input);
    let non_allergenic: usize = input
        .iter()
        .map(|i| {
            i.0.iter()
                .filter(|i| {
                    !map.iter()
                        .any(|(_, ingredients)| ingredients.contains(&i[..]))
                })
                .count()
        })
        .sum();
    println!(
        "(1) {} ingredients do not contain allergens",
        non_allergenic
    );

    let solution = reduce_map(&mut map);
    println!(
        "(2) {}",
        solution.values().cloned().collect::<Vec<_>>().join(",")
    );

    Ok(())
}

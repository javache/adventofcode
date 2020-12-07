use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, BufRead};

type MultiMap<K, V> = HashMap<K, Vec<V>>;

fn parse_bag(input: &str) -> Option<(u32, String)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d)+\s*([\w\s]+) bag").unwrap();
    }
    RE.captures_iter(input)
        .next()
        .and_then(|captures| match captures[1].parse() {
            Ok(count) => Some((count, captures[2].to_string())),
            _ => None,
        })
}

fn collect_containers<'a>(
    target: &str,
    bag_map: &'a MultiMap<String, String>,
    mut result: &mut HashSet<&'a str>,
) {
    if let Some(container_types) = bag_map.get(target) {
        for container in container_types {
            if !result.contains::<str>(container) {
                result.insert(container);
                collect_containers(container, &bag_map, &mut result);
            }
        }
    }
}

fn count_contents(target: &str, bag_map: &MultiMap<String, (u32, String)>) -> u32 {
    bag_map
        .get(target)
        .map(|contained_types| {
            contained_types.iter().fold(0, |acc, (count, bag_type)| {
                acc + count * (1 + count_contents(&bag_type, bag_map))
            })
        })
        .unwrap_or(0)
}

fn main() -> io::Result<()> {
    let mut container_map = MultiMap::new();
    let mut contents_map = MultiMap::new();

    for line in io::stdin().lock().lines() {
        if let [bag_type, contents] = line?
            .split(" bags contain ")
            .collect::<Vec<&str>>()
            .as_slice()
        {
            if contents == &"no other bags." {
                continue;
            }

            for contained_bag in contents.split(", ") {
                if let Some((bag_count, contained_color)) = parse_bag(contained_bag) {
                    container_map
                        .entry(contained_color.clone())
                        .or_insert_with(|| vec![])
                        .push(bag_type.to_string());
                    contents_map
                        .entry(bag_type.to_string())
                        .or_insert_with(|| vec![])
                        .push((bag_count, contained_color))
                }
            }
        }
    }

    let target = "shiny gold";

    let mut containers = HashSet::new();
    collect_containers(target, &container_map, &mut containers);
    println!(
        "(1) Found {} potential containers for {}",
        containers.len(),
        target
    );

    let count = count_contents(target, &contents_map);
    println!("(2) One {} bag contains {} other bags", target, count);

    Ok(())
}

use std::io::{self, BufRead};
use std::ops::Range;

#[derive(Debug)]
struct Mapping {
    src: usize,
    dest: usize,
    len: usize,
}

fn map_value(mapping: &Vec<Mapping>, value: usize) -> usize {
    // For some reason this is faster than using partition_point / binary search
    mapping
        .iter()
        .find(|m| m.src <= value && value < m.src + m.len)
        .map(|m| m.dest + (value - m.src))
        .unwrap_or(value)
}

fn map_range(mapping: &Vec<Mapping>, range: &Range<usize>) -> Vec<Range<usize>> {
    let mut result = vec![];
    let mut last_end = range.start;
    for m in mapping {
        if m.src > range.end {
            break;
        }
        if m.src > last_end {
            result.push(last_end..m.src);
            last_end = m.src;
        }

        let adj_start = m.src.max(range.start);
        let adj_end = (m.src + m.len).min(range.end);
        if adj_end > adj_start {
            result.push((m.dest + (adj_start - m.src))..(m.dest + (adj_end - m.src)));
            last_end = adj_end;
        }
    }
    if last_end < range.end {
        result.push(last_end..range.end);
    }
    result
}

fn main() -> io::Result<()> {
    let mut seeds = String::new();
    io::stdin().lock().read_line(&mut seeds)?;
    let seeds = seeds
        .trim()
        .split(' ')
        .flat_map(|num| num.parse())
        .collect::<Vec<usize>>();

    let mut mappings: Vec<Vec<Mapping>> = vec![];
    let mut current_mapping: Vec<Mapping> = vec![];
    for line in io::stdin().lock().lines().flatten() {
        if line.len() == 0 && current_mapping.len() > 0 {
            mappings.push(current_mapping);
            current_mapping = vec![];
            continue;
        }
        if let [dest, src, len] = line
            .split(' ')
            .flat_map(|num| num.parse())
            .collect::<Vec<usize>>()[..]
        {
            current_mapping.push(Mapping { src, dest, len });
        }
    }
    mappings.push(current_mapping);

    // Sort the mappings by src
    mappings.iter_mut().for_each(|mapping| {
        mapping.sort_by(|a, b| a.src.cmp(&b.src));
    });

    let closest_location = seeds
        .iter()
        .map(|seed| {
            mappings
                .iter()
                .fold(*seed, |v, mapping| map_value(mapping, v))
        })
        .min();
    println!("(1) Closest location is {}", closest_location.unwrap());

    let closest_location = seeds
        .chunks(2)
        .map(|pair| pair[0]..(pair[0] + pair[1]))
        .map(|range| {
            let mut curr = vec![range];
            for mapping in &mappings {
                curr = curr
                    .iter()
                    .flat_map(|range| map_range(mapping, range))
                    .collect();
            }
            curr.iter().map(|r| r.start).min().unwrap()
        })
        .min();
    println!("(2) Closest location is {}", closest_location.unwrap());

    Ok(())
}

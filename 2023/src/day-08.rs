use std::collections::HashMap;
use std::io::{self, BufRead};

fn get_path_len(
    lookup: &HashMap<String, (String, String)>,
    directions: &str,
    start: &str,
) -> usize {
    let mut node = start;
    let (steps, _) = (1..)
        .zip(directions.trim().chars().cycle())
        .skip_while(|(_, dir)| {
            node = match dir {
                'L' => &lookup[node].0,
                'R' => &lookup[node].1,
                _ => panic!(),
            };
            !node.ends_with('Z')
        })
        .next()
        .unwrap();
    steps
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(vals: &Vec<usize>) -> usize {
    vals.iter().fold(1, |acc, x| acc * x / gcd(acc, *x))
}

fn main() -> io::Result<()> {
    let mut directions = String::new();
    io::stdin().lock().read_line(&mut directions)?;

    let mut nodes: HashMap<String, (String, String)> = HashMap::new();
    for line in io::stdin().lock().lines().flatten() {
        if line.len() > 0 {
            nodes.insert(
                line[0..3].to_string(),
                (line[7..10].to_string(), line[12..15].to_string()),
            );
        }
    }

    let path_len = get_path_len(&nodes, &directions, "AAA");
    println!("(1) Reached ZZZ after {} steps", path_len);

    let path_lengths = nodes
        .keys()
        .filter(|node| node.ends_with('A'))
        .map(|start| get_path_len(&nodes, &directions, start))
        .collect::<Vec<_>>();
    println!(
        "(2) Can reach all Z nodes after {} steps",
        lcm(&path_lengths)
    );

    Ok(())
}

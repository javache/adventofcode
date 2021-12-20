use std::collections::HashMap;
use std::io::{self, BufRead};

type OrbiterMap = HashMap<String, String>;
type OrbitMap = HashMap<String, Vec<String>>;

fn count_orbits(map: &OrbitMap, orbitee: &str, depth: usize) -> usize {
    depth
        + map
            .get(orbitee)
            .map(|orbitees| {
                orbitees
                    .iter()
                    .map(|orbiter| count_orbits(map, orbiter, depth + 1))
                    .sum()
            })
            .unwrap_or(0)
}

fn lca_path_size(map: &OrbiterMap, a: &str, b: &str) -> usize {
    let mut a_parents = vec![a];
    let mut parent = a;
    while let Some(node) = map.get(parent) {
        a_parents.push(node);
        parent = node;
    }

    let mut b_parents = vec![b];
    let mut parent = b;
    while let Some(node) = map.get(parent) {
        b_parents.push(node);
        parent = node;
    }

    let lca_position = a_parents
        .iter()
        .rev()
        .zip(b_parents.iter().rev())
        .position(|(a, b)| a != b)
        .unwrap();
    let depth_in_a = a_parents.len() - lca_position;
    let depth_in_b = b_parents
        .iter()
        .position(|node| *node == a_parents[depth_in_a])
        .unwrap();
    depth_in_a + depth_in_b - 1
}

fn main() -> io::Result<()> {
    let mut orbit_map: OrbitMap = HashMap::new();
    let mut orbiter_map: OrbiterMap = HashMap::new();
    io::stdin().lock().lines().flatten().for_each(|line| {
        if let [orbitee, orbiter] = line.split(')').collect::<Vec<&str>>()[..] {
            orbit_map
                .entry(orbitee.to_owned())
                .or_default()
                .push(orbiter.to_owned());
            orbiter_map.insert(orbiter.to_owned(), orbitee.to_owned());
        }
    });

    println!(
        "(1) There are {} direct and indirect orbits",
        count_orbits(&orbit_map, "COM", 0)
    );
    println!(
        "(2) It takes {} orbit transfers to get YOU to SAN's orbiter",
        lca_path_size(&orbiter_map, "YOU", "SAN") - 1,
    );

    Ok(())
}

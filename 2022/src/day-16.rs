use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead};

static SCAN_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? (.+)$").unwrap()
});

// For each node, track its flow rate, and the distances to all other non-0 nodes
type Graph = HashMap<u16, (u8, HashMap<u16, u8>)>;

fn find_optimal_route(
    graph: &Graph,
    curr: u16,
    opened: &mut Vec<u16>,
    time_left: i32,
    add_support: bool,
) -> usize {
    if opened.len() == graph.len() {
        return 0;
    }

    let (_, edges) = &graph[&curr];
    let mut best_score = edges
        .iter()
        .map(|(id, cost)| {
            let time_left = time_left - *cost as i32 - 1;
            if opened.contains(id) || time_left <= 0 {
                0
            } else {
                opened.push(*id);
                let score = find_optimal_route(graph, *id, opened, time_left, add_support);
                opened.pop();
                score + (graph[id].0 as i32 * time_left) as usize
            }
        })
        .max()
        .unwrap();

    // For each possible path consider the score of another path visiting the remaining unvisited nodes
    if add_support {
        let filtered_graph: Graph = graph
            .iter()
            .filter(|(node, _)| !opened.contains(node))
            .map(|(node, (flow_rate, distances))| {
                let filtered_distances = distances
                    .iter()
                    .filter(|(node, _)| !opened.contains(node))
                    .map(|(k, v)| (*k, *v))
                    .collect();
                (*node, (*flow_rate, filtered_distances))
            })
            .collect();
        best_score = find_optimal_route(&filtered_graph, node_id("AA"), &mut vec![], 26, false)
            .max(best_score);
    }

    best_score
}

fn node_id(name: &str) -> u16 {
    name.chars()
        .map(|c| ((c as u8) - b'A') as u16)
        .reduce(|acc, c| acc * 100 + c)
        .unwrap()
}

fn calculate_distances(input: &HashMap<u16, (u8, Vec<u16>)>) -> HashMap<u16, HashMap<u16, u8>> {
    let mut distances = HashMap::<u16, HashMap<u16, u8>>::new();
    for (node, (_, valves)) in input {
        let from_node = distances.entry(*node).or_default();
        for valve in valves {
            from_node.insert(*valve, 1);
        }
    }

    // Use Floyd-Warshall to get minimum distances between each node
    let nodes = distances.keys().cloned().collect::<Vec<_>>();
    for k in &nodes {
        for i in &nodes {
            for j in &nodes {
                if k == i || k == j || i == j {
                    continue;
                }

                let distance_ij = distances[i].get(j).unwrap_or(&u8::MAX);
                if let (Some(distance_ik), Some(distance_kj)) =
                    (distances[i].get(k), distances[k].get(j))
                {
                    if *distance_ij > distance_ik + distance_kj {
                        let cost = distance_ik + distance_kj;
                        distances.get_mut(i).unwrap().insert(*j, cost);
                    }
                }
            }
        }
    }
    distances
}

fn main() -> io::Result<()> {
    let input: HashMap<u16, (u8, Vec<u16>)> = io::stdin()
        .lock()
        .lines()
        .flatten()
        .flat_map(|line| {
            SCAN_RE.captures(&line).map(|caps| {
                let valves: Vec<_> = caps[3].split(", ").map(node_id).collect();
                (node_id(&caps[1]), (caps[2].parse().unwrap(), valves))
            })
        })
        .collect();

    let mut distances = calculate_distances(&input);
    let graph: Graph = input
        .iter()
        .filter_map(|(node, (flow_rate, _))| {
            (*node == node_id("AA") || *flow_rate > 0).then(|| {
                let node_distances = distances
                    .remove(node)
                    .unwrap()
                    .into_iter()
                    .filter(|(key, _)| input[key].0 > 0)
                    .collect();
                (*node, (*flow_rate, node_distances))
            })
        })
        .collect();

    println!(
        "(1) Optimal route scores {}",
        find_optimal_route(&graph, node_id("AA"), &mut vec![], 30, false)
    );
    println!(
        "(2) Optimal route when working together scores {}",
        find_optimal_route(&graph, node_id("AA"), &mut vec![], 26, true)
    );

    Ok(())
}

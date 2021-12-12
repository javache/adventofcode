use std::io::{self, BufRead};

struct Node {
    name: String,
    is_small: bool,
    neighbours: Vec<usize>,
}

type NodeMap = Vec<Node>;

fn get_node(nodes: &mut NodeMap, name: &str) -> usize {
    if let Some(index) = nodes.iter().position(|n| n.name == name) {
        index
    } else {
        nodes.push(Node {
            name: name.to_string(),
            is_small: name.chars().any(|c| c.is_lowercase()),
            neighbours: vec![],
        });
        nodes.len() - 1
    }
}

fn explore_paths(
    nodes: &NodeMap,
    path: &mut Vec<usize>,
    allow_double_visit_of_small_nodes: bool,
) -> usize {
    let current_node = &nodes[*path.last().unwrap()];
    if current_node.name == "end" {
        1
    } else {
        current_node
            .neighbours
            .iter()
            .map(|neighbour| {
                let mut paths_found = 0;
                if nodes[*neighbour].is_small && path.contains(&neighbour) {
                    if allow_double_visit_of_small_nodes {
                        path.push(*neighbour);
                        paths_found = explore_paths(nodes, path, false);
                        path.pop();
                    }
                } else {
                    path.push(*neighbour);
                    paths_found = explore_paths(nodes, path, allow_double_visit_of_small_nodes);
                    path.pop();
                }
                paths_found
            })
            .sum()
    }
}

fn main() -> io::Result<()> {
    let mut nodes: NodeMap = vec![];
    io::stdin().lock().lines().flatten().for_each(|line| {
        if let [from, to] = line.split('-').collect::<Vec<&str>>()[..] {
            let from_node = get_node(&mut nodes, from);
            let to_node = get_node(&mut nodes, to);
            nodes[from_node].neighbours.push(to_node);
            nodes[to_node].neighbours.push(from_node);
        }
    });

    let start_node = nodes.iter().position(|n| n.name == "start").unwrap();

    // Remove edges back into to the start node to simplify (2)
    nodes
        .iter_mut()
        .for_each(|n| n.neighbours.retain(|n2| *n2 != start_node));

    println!(
        "(1) Visiting small nodes at most once, there are {} paths",
        explore_paths(&nodes, &mut vec![start_node], false)
    );
    println!(
        "(2) Visiting one small node at most twice, there are {} paths",
        explore_paths(&nodes, &mut vec![start_node], true)
    );

    Ok(())
}

use std::collections::HashMap;
use std::io::{self, BufRead};

type Grid = Vec<Vec<char>>;
type Graph = HashMap<Point, Vec<(Point, i32)>>;
type Point = (usize, usize);

const NEIGHBOURS: &[(i32, i32)] = &[(0, 1), (0, -1), (1, 0), (-1, 0)];

fn find_valid_steps(point: &Point, grid: &Grid, is_part2: bool) -> Vec<(Point, i32)> {
    let grid_size = (0..grid.len() as i32, 0..grid[0].len() as i32);
    (match grid[point.0][point.1] {
        _ if is_part2 => NEIGHBOURS,
        '.' => &NEIGHBOURS,
        '>' => &NEIGHBOURS[0..1],
        '<' => &NEIGHBOURS[1..2],
        'v' => &NEIGHBOURS[2..3],
        '^' => &NEIGHBOURS[3..4],
        _ => unreachable!(),
    })
    .into_iter()
    .filter_map(move |(dy, dx)| {
        let neighbour = (point.0 as i32 + dy, point.1 as i32 + dx);
        (grid_size.0.contains(&neighbour.0) && grid_size.1.contains(&neighbour.1))
            .then(|| (((neighbour.0 as usize), (neighbour.1 as usize)), 1))
    })
    .filter(|&(p, _)| grid[p.0][p.1] != '#')
    .collect::<Vec<_>>()
}

fn dfs(point: &Point, graph: &Graph, visited: &mut Grid) -> i32 {
    if point.0 == visited.len() - 1 {
        return 0;
    }

    visited[point.0][point.1] = '1';
    let mut max = i32::MIN;
    for (neighbour, cost) in &graph[point] {
        if visited[neighbour.0][neighbour.1] == '1' {
            continue;
        }
        max = (dfs(&neighbour, graph, visited) + cost).max(max);
    }
    visited[point.0][point.1] = '0';

    max
}

fn build_graph(grid: &Grid, is_part2: bool) -> Graph {
    let mut graph: Graph = HashMap::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '#' {
                continue;
            }
            graph.insert((y, x), find_valid_steps(&(y, x), grid, is_part2));
        }
    }

    // Compress graph
    let paths = graph
        .iter()
        .filter_map(|(node, neighbours)| (neighbours.len() == 2).then(|| *node))
        .collect::<Vec<_>>();
    for path in paths {
        let neighbours = graph.remove(&path).unwrap();
        graph.get_mut(&neighbours[0].0).map(|n1| {
            n1.iter_mut()
                .filter(|(p, _)| p == &path)
                .for_each(|n| *n = (neighbours[1].0, n.1 + neighbours[1].1))
        });
        graph.get_mut(&neighbours[1].0).map(|n2| {
            n2.iter_mut()
                .filter(|(p, _)| p == &path)
                .for_each(|n| *n = (neighbours[0].0, n.1 + neighbours[0].1))
        });
    }
    graph
}

fn main() {
    let input: Grid = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let start = (0, input[0].iter().position(|c| *c == '.').unwrap());
    let mut visited = vec![vec!['0'; input[0].len()]; input.len()];

    let graph1 = build_graph(&input, false);
    let longest_path = dfs(&start, &graph1, &mut visited);
    println!("(1) Longest path is {} steps", longest_path);

    let graph2 = build_graph(&input, true);
    let longest_path = dfs(&start, &graph2, &mut visited);
    println!("(2) Longest path is {} steps", longest_path);
}

use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use std::io::{self, BufRead};

type Blueprint = [[u8; 3]; 4];

static BLUEPRINT_RE: Lazy<[Regex; 4]> = Lazy::new(|| {
    [
        Regex::new(r"ore robot costs (\d+) ore.").unwrap(),
        Regex::new(r"clay robot costs (\d+) ore.").unwrap(),
        Regex::new(r"obsidian robot costs (\d+) ore and (\d+) clay.").unwrap(),
        Regex::new(r"geode robot costs (\d+) ore and (\d+) obsidian.").unwrap(),
    ]
});

fn parse_blueprint(line: String) -> Blueprint {
    let nums: Vec<(u8, u8)> = BLUEPRINT_RE
        .iter()
        .map(|cost_re| {
            cost_re
                .captures(&line)
                .unwrap()
                .iter()
                .flat_map(|n| n.unwrap().as_str().parse::<u8>())
                .pad_using(2, |_| 0)
                .collect_tuple()
                .unwrap()
        })
        .collect();
    [
        [nums[0].0, 0, 0],
        [nums[1].0, 0, 0],
        [nums[2].0, nums[2].1, 0],
        [nums[3].0, 0, nums[3].1],
    ]
}

fn max_robots_needed(blueprint: &Blueprint) -> [u8; 3] {
    [
        blueprint.iter().map(|c| c[0]).max().unwrap(),
        blueprint.iter().map(|c| c[1]).max().unwrap(),
        blueprint.iter().map(|c| c[2]).max().unwrap(),
    ]
}

fn find_optimal_solution(blueprint: &Blueprint, time: u16) -> usize {
    let max_robots = max_robots_needed(&blueprint);
    find_optimal_solution_inner(
        blueprint,
        &max_robots,
        [1, 0, 0, 0],
        None,
        [0, 0, 0],
        time,
        0,
        0,
    ) as usize
}

fn find_optimal_solution_inner(
    blueprint: &Blueprint,
    max_robots: &[u8; 3],
    mut robots: [u8; 4],
    robot_being_built: Option<usize>,
    mut inventory: [u8; 3],
    time_left: u16,
    mut curr_score: u8,
    mut best_score: u8,
) -> u8 {
    if time_left == 0 {
        return curr_score;
    }

    curr_score += robots[3];

    let mut upper_bound = curr_score as u16 + robots[3] as u16 * (time_left - 1);
    upper_bound += (time_left * (time_left - 1)) / 2;
    if best_score as u16 > upper_bound {
        return 0;
    }

    // Update inventory
    for (idx, material) in inventory.iter_mut().enumerate() {
        *material += robots[idx];
    }

    // Update robot list
    if let Some(robot_being_built) = robot_being_built {
        robots[robot_being_built] += 1;
    }

    // Try creating a new robot
    'outer: for robot_idx in (0..4).rev() {
        if robot_idx < 3 && robots[robot_idx] >= max_robots[robot_idx] {
            continue;
        }

        let price = blueprint[robot_idx];
        let mut inventory = inventory.clone();
        for (idx, material) in inventory.iter_mut().enumerate() {
            // Can't build this robot right now
            if *material < price[idx] {
                continue 'outer;
            }
            *material -= price[idx];
        }

        best_score = find_optimal_solution_inner(
            blueprint,
            max_robots,
            robots,
            Some(robot_idx),
            inventory,
            time_left - 1,
            curr_score,
            best_score,
        )
        .max(best_score);

        // If we can build a geode, that's always the best option, no need to recurse
        if robot_idx == 3 {
            return best_score;
        }
    }

    // ... or do nothing
    find_optimal_solution_inner(
        blueprint,
        max_robots,
        robots,
        None,
        inventory,
        time_left - 1,
        curr_score,
        best_score,
    )
    .max(best_score)
}

fn main() {
    let input: Vec<Blueprint> = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(&parse_blueprint)
        .collect();

    let quality_levels = input
        .iter()
        .enumerate()
        .map(|(idx, blueprint)| {
            let score = find_optimal_solution(&blueprint, 24);
            println!("Blueprint {} scored {}", idx + 1, score);
            (idx + 1) * score
        })
        .sum::<usize>();
    println!("(1) Sum of quality levels is {}", quality_levels);

    let quality_levels = input
        .iter()
        .take(3)
        .enumerate()
        .map(|(idx, blueprint)| {
            let score = find_optimal_solution(&blueprint, 32);
            println!("Blueprint {} scored {}", idx + 1, score);
            score
        })
        .product::<usize>();
    println!(
        "(2) Product of quality levels of first 3 blueprints is {}",
        quality_levels
    );
}

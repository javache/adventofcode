use itertools::Itertools;
use std::collections::HashMap;
use std::io::{self, BufRead};

fn find_best_position(positions: &Vec<(i32, i32)>) -> ((i32, i32), usize) {
    let mut state: HashMap<(i32, i32), usize> =
        HashMap::from_iter(positions.iter().map(|p| (*p, 0)));
    for (pos_a, pos_b) in positions.iter().tuple_combinations() {
        let obstructed = if pos_b.0 - pos_a.0 == 0 {
            // vertical line
            ((pos_b.1.min(pos_a.1) + 1)..(pos_b.1.max(pos_a.1)))
                .any(|y| state.contains_key(&(pos_a.0, y)))
        } else {
            let slope = (pos_b.1 - pos_a.1) as f64 / (pos_b.0 - pos_a.0) as f64;
            ((pos_b.0.min(pos_a.0) + 1)..(pos_b.0.max(pos_a.0)))
                .map(|x| (x, slope * (x - pos_a.0) as f64 + pos_a.1 as f64))
                .filter(|&(_, y)| y == (y as i32) as f64)
                .any(|(x, y)| state.contains_key(&(x, y as i32)))
        };
        if !obstructed {
            *state.get_mut(pos_a).unwrap() += 1;
            *state.get_mut(pos_b).unwrap() += 1;
        }
    }

    let (pos, count) = state.iter().max_by_key(|(_, v)| *v).unwrap();
    (*pos, *count)
}

fn get_distances(positions: &Vec<(i32, i32)>, target: &(i32, i32)) -> Vec<((i32, i32), f64, f64)> {
    // Calculate distance and angle to each point
    let mut distances = positions
        .iter()
        .filter(|&p| p != target)
        .map(|p| {
            let dx = (p.0 - target.0) as f64;
            let dy = (p.1 - target.1) as f64;
            let dist = dx * dx + dy * dy;
            (
                *p,
                dist,
                (dy.atan2(dx).to_degrees() + 90.0).rem_euclid(360.0),
            )
        })
        .collect::<Vec<_>>();
    // Sort by angle first, then distance
    distances.sort_by(|a, b| (a.2, a.1).partial_cmp(&(b.2, b.1)).unwrap());
    distances
}

fn main() {
    let mut input: Vec<(i32, i32)> = vec![];
    for (y, line) in io::stdin().lock().lines().flatten().enumerate() {
        for (x, _) in line.chars().enumerate().filter(|&(_, c)| c != '.') {
            input.push((x as i32, y as i32));
        }
    }

    let (position, visible_sensors) = find_best_position(&input);
    println!(
        "(1) From the best position {:?}, we can see {} asteroids",
        position, visible_sensors
    );

    let mut distances = get_distances(&input, &position);

    let mut last_angle = -1.0;
    for i in 1..=200 {
        if distances.is_empty() {
            break;
        }
        let next = distances
            .iter()
            .position(|&(_, _, angle)| angle > last_angle)
            .unwrap_or(0);
        let asteroid = &distances[next];

        if i == 200 {
            println!(
                "(2) Vaporizing {:?} => {} in turn 200",
                asteroid.0,
                asteroid.0 .0 * 100 + asteroid.0 .1
            );
        }

        last_angle = asteroid.2;
        distances.remove(next);
    }
}

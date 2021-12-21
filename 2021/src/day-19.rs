use itertools::Itertools;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufRead};
use std::str;

type Point = (i32, i32, i32);

const ORIENTATIONS: [(i8, i8, i8); 24] = [
    (1, 2, 3),
    (1, 3, -2),
    (1, -2, -3),
    (1, -3, 2),
    (-1, 2, -3),
    (-1, 3, 2),
    (-1, -2, 3),
    (-1, -3, -2),
    (2, 1, -3),
    (2, 3, 1),
    (2, -1, 3),
    (2, -3, -1),
    (-2, 1, 3),
    (-2, 3, -1),
    (-2, -1, -3),
    (-2, -3, 1),
    (3, 1, 2),
    (3, 2, -1),
    (3, -1, -2),
    (3, -2, 1),
    (-3, 1, -2),
    (-3, 2, 1),
    (-3, -1, 2),
    (-3, -2, -1),
];

const MIN_BEACON_OVERLAP: usize = 12;

fn parse_point(line: &String) -> Option<Point> {
    if let [x, y, z] = line.split(',').flat_map(str::parse).collect::<Vec<i32>>()[..] {
        Some((x, y, z))
    } else {
        None
    }
}

fn orientate_readings(points: &Vec<Point>, orientation: &(i8, i8, i8)) -> Vec<Point> {
    points
        .iter()
        .map(|p| {
            let p = vec![p.0, p.1, p.2];
            (
                p[(orientation.0.abs() - 1) as usize] * (orientation.0.signum() as i32),
                p[(orientation.1.abs() - 1) as usize] * (orientation.1.signum() as i32),
                p[(orientation.2.abs() - 1) as usize] * (orientation.2.signum() as i32),
            )
        })
        .collect()
}

fn find_overlap(base: &HashSet<Point>, reading: &Vec<Point>) -> Option<(Point, HashSet<Point>)> {
    // Brute-force search a match between reading and base
    ORIENTATIONS.iter().find_map(|orientation| {
        let oriented_readings = orientate_readings(reading, orientation);
        for reference_point in base {
            for point in &oriented_readings {
                let delta = (
                    reference_point.0 - point.0,
                    reference_point.1 - point.1,
                    reference_point.2 - point.2,
                );

                let transformed: HashSet<Point> = oriented_readings
                    .iter()
                    .map(|p| (p.0 + delta.0, p.1 + delta.1, p.2 + delta.2))
                    .collect();
                if transformed.intersection(base).count() >= MIN_BEACON_OVERLAP {
                    return Some((delta, transformed));
                }
            }
        }
        None
    })
}

fn pairwise_distances(reading: &Vec<Point>) -> HashSet<Point> {
    reading
        .iter()
        .combinations(2)
        .map(|combo| {
            [
                (combo[0].0 - combo[1].0).abs(),
                (combo[0].1 - combo[1].1).abs(),
                (combo[0].2 - combo[1].2).abs(),
            ]
            .iter()
            .permutations(3)
            .map(|permutation| (*permutation[0], *permutation[1], *permutation[2]))
            .collect::<Vec<Point>>()
        })
        .flatten()
        .collect()
}

fn main() -> io::Result<()> {
    let lines: Vec<String> = io::stdin().lock().lines().flatten().collect();
    let readings: Vec<Vec<Point>> = lines[..]
        .split(|l| l == "")
        .map(|s| s[1..].iter().flat_map(parse_point).collect())
        .collect();
    let distances: Vec<HashSet<Point>> = readings.iter().map(pairwise_distances).collect();

    let mut resolved_sensors: Vec<(usize, Point)> = vec![(0, (0, 0, 0))];
    let mut resolved_readings: Vec<HashSet<Point>> = vec![readings[0].iter().cloned().collect()];

    let mut unresolved_readings = VecDeque::from_iter(readings.iter().enumerate().skip(1));
    while unresolved_readings.len() > 0 {
        let (i, reading) = unresolved_readings.pop_front().unwrap();
        if let Some((delta, transformed)) =
            resolved_readings
                .iter()
                .enumerate()
                .find_map(|(j, beacons)| {
                    let sensor_idx = resolved_sensors[j].0;
                    if distances[sensor_idx].intersection(&distances[i]).count() > 100 {
                        find_overlap(beacons, reading)
                    } else {
                        None
                    }
                })
        {
            resolved_sensors.push((i, delta));
            resolved_readings.push(transformed);
        } else {
            unresolved_readings.push_back((i, reading));
        }
    }

    let all_beacons: HashSet<Point> = resolved_readings.iter().cloned().flatten().collect();
    println!("(1) There are {} beacons", all_beacons.len());

    let max_distance = resolved_sensors
        .iter()
        .map(|(_, position)| position)
        .combinations(2)
        .map(|pair| {
            (pair[0].0 - pair[1].0).abs()
                + (pair[0].1 - pair[1].1).abs()
                + (pair[0].2 - pair[1].2).abs()
        })
        .max()
        .unwrap();
    println!("(2) The max distance between sensors is {}", max_distance);

    Ok(())
}

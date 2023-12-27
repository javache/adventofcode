use itertools::{iproduct, Itertools};
use std::collections::VecDeque;
use std::io::{self, BufRead};
use std::str::FromStr;

type Point = (usize, usize, usize);

fn get_block_dependencies(blocks: &Vec<(Point, Point)>) -> Vec<Vec<usize>> {
    let max_x = blocks.iter().map(|(_, end)| end.0).max().unwrap();
    let max_y = blocks.iter().map(|(_, end)| end.1).max().unwrap();
    let mut levels: Vec<Vec<(usize, i32)>> = vec![vec![(0, -1); max_y + 1]; max_x + 1];
    blocks
        .iter()
        .enumerate()
        .map(|(idx, (mut start, mut end))| {
            // For each x-y, track the max z occupied so far
            let mut min_z = 0;
            for (x, y) in iproduct!(start.0..=end.0, start.1..=end.1) {
                if levels[x][y].0 + end.2 - start.2 + 1 > min_z {
                    min_z = levels[x][y].0 + 1;
                }
            }
            end.2 = min_z + end.2 - start.2;
            start.2 = min_z;

            let deps = iproduct!(start.0..=end.0, start.1..=end.1)
                .map(|(x, y)| levels[x][y])
                .filter(|&(depth, dep)| depth == start.2 - 1 && dep != -1)
                .map(|(_, dep)| dep as usize)
                .sorted_unstable()
                .dedup()
                .collect::<Vec<_>>();
            for (x, y) in iproduct!(start.0..=end.0, start.1..=end.1) {
                levels[x][y] = (end.2, idx as i32);
            }
            deps
        })
        .collect::<Vec<_>>()
}

fn count_safe_to_disintegrate(deps: &Vec<Vec<usize>>) -> usize {
    (0..deps.len())
        .filter(|idx| {
            deps.iter()
                .enumerate()
                .filter(|(_, dep)| dep.contains(idx))
                .all(|(i, _)| deps[i].len() > 1)
        })
        .count()
}

fn count_dominator_size(deps: &Vec<Vec<usize>>) -> Vec<usize> {
    (0..deps.len())
        .map(|idx| {
            let mut queue: VecDeque<usize> = [idx].into_iter().collect();
            let mut deps = deps.clone();

            let mut count = 0;
            while let Some(removed_idx) = queue.pop_front() {
                count += 1;
                for (dep_idx, dep) in deps.iter_mut().enumerate() {
                    if !dep.is_empty() {
                        dep.retain(|&d| d != removed_idx);
                        if dep.is_empty() {
                            queue.push_back(dep_idx);
                        }
                    }
                }
            }
            count - 1
        })
        .collect()
}

fn main() {
    let mut input = io::stdin()
        .lock()
        .lines()
        .flatten()
        .flat_map(|line| {
            line.split_once('~').map(|(start, end)| {
                let start: Point = start
                    .split(',')
                    .flat_map(usize::from_str)
                    .collect_tuple()
                    .unwrap();
                let end: Point = end
                    .split(',')
                    .flat_map(usize::from_str)
                    .collect_tuple()
                    .unwrap();
                assert!(start.0 <= end.0);
                assert!(start.1 <= end.1);
                assert!(start.2 <= end.2);
                (start, end)
            })
        })
        .collect::<Vec<_>>();
    input.sort_by(|(_, (_, _, a_z)), (_, (_, _, b_z))| a_z.cmp(&b_z));

    let deps = get_block_dependencies(&input);
    println!(
        "(1) There are {} blocks safe to disintegrate",
        count_safe_to_disintegrate(&deps)
    );

    let dominator_sizes = count_dominator_size(&deps);
    println!(
        "(2) Maximum to fall with one block is {}",
        dominator_sizes.iter().sum::<usize>()
    );
}

use std::char;
use std::collections::HashMap;
use std::io::{self, Read};
use std::ops::Range;

struct Node {
    val: u32,
    next: usize,
}

struct Ring {
    nodes: Vec<Node>,
    lookup: HashMap<u32, usize>,
}

impl Ring {
    fn new(data: &Vec<u32>) -> Ring {
        let mut nodes: Vec<_> = data
            .iter()
            .enumerate()
            .map(|(i, &val)| Node { val, next: i + 1 })
            .collect();
        nodes.last_mut().map(|el| el.next = 0);
        Ring {
            nodes,
            lookup: data.iter().enumerate().map(|(i, &val)| (val, i)).collect(),
        }
    }

    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn find_first_in_range(&self, range: Range<u32>, exclude_list: &Vec<u32>) -> Option<u32> {
        range.rev().find_map(|search| {
            if self.lookup.contains_key(&search) && !exclude_list.contains(&search) {
                return Some(search);
            } else {
                None
            }
        })
    }

    fn remove(&mut self, from: u32, to: u32) {
        self.nodes[self.lookup[&from]].next = self.nodes[self.lookup[&to]].next;
    }

    fn insert(&mut self, node_val: u32, to_insert: &Vec<u32>) {
        let node_idx = self.lookup[&node_val];
        let node_next = self.nodes[node_idx].next;
        self.nodes[node_idx].next = self.lookup[&to_insert[0]];
        self.nodes[self.lookup[&to_insert[to_insert.len() - 1]]].next = node_next;
    }

    fn iter_from(&self, from_val: u32) -> RingIterator {
        let start = &self.nodes[self.lookup[&from_val]];
        RingIterator {
            nodes: &self.nodes,
            curr: &self.nodes[start.next],
        }
    }

    fn formatted_result(&self) -> String {
        self.iter_from(1)
            .take(self.nodes.len() - 1)
            .flat_map(|node| char::from_digit(node, 10))
            .collect()
    }
}

struct RingIterator<'a> {
    nodes: &'a Vec<Node>,
    curr: &'a Node,
}

impl<'a> Iterator for RingIterator<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let curr = self.curr;
        self.curr = &self.nodes[curr.next];
        Some(curr.val)
    }
}

fn run(mut ring: Ring, turns: usize) -> Ring {
    let mut current = ring.nodes[0].val;
    for _ in 0..turns {
        let removed = ring.iter_from(current).take(3).collect::<Vec<_>>();
        ring.remove(current, removed[removed.len() - 1]);

        let dest_val = ring.find_first_in_range(1..current, &removed);
        let dest_val = dest_val.unwrap_or_else(|| {
            ring.find_first_in_range(current..(ring.len() as u32 + 1), &removed)
                .unwrap()
        });
        ring.insert(dest_val, &removed);

        current = ring.iter_from(current).next().unwrap();
    }
    ring
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;

    let mut input = input
        .chars()
        .flat_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();

    let result = run(Ring::new(&input), 100);
    println!("(1) Result: {}", result.formatted_result());

    input.extend(((input.len() + 1) as u32)..=1000000);
    let result = run(Ring::new(&input), 10000000);
    let values = result.iter_from(1).take(2).collect::<Vec<_>>();
    println!("(2) Result: {}", values[0] as u64 * values[1] as u64);

    Ok(())
}

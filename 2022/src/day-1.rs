use std::collections::BinaryHeap;
use std::io::{self, BufRead};

fn main() {
    let mut heap = BinaryHeap::new();
    let mut curr = 0;
    for line in io::stdin().lock().lines().flatten() {
        if line == "" {
            heap.push(curr);
            curr = 0;
        } else {
            curr += line.parse::<i32>().unwrap();
        }
    }
    if curr > 0 {
        heap.push(curr);
    }

    if let Some(max) = heap.peek() {
        println!("(1) Elf with the most calories is carrying {}", max);
    }

    let sum: i32 = (0..3).map(|_| heap.pop()).flatten().sum();
    println!("(2) 3 top elfs are carrying {}", sum);
}

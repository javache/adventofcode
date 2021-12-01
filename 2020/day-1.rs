use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let sum = 2020;
    let mut seen = HashSet::new();

    for line in io::stdin().lock().lines() {
        let num = line?.parse::<i32>().unwrap();
        seen.insert(num);
    }

    for x in &seen {
        let y = sum - x;
        if seen.contains(&y) && x < &y {
            println!("(1) {} * {} = {}", x, y, x * y);
        }
    }

    for x in &seen {
        for y in &seen {
            let z = sum - x - y;
            if seen.contains(&z) && x < &y && y < &z {
                println!("(2) {} * {} * {} = {}", x, y, z, x * y * z);
            }
        }
    }

    Ok(())
}

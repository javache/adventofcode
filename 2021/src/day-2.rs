use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut position = 0;
    let mut depth = 0;
    let mut depth_v2 = 0;
    let mut aim = 0;

    for line in io::stdin().lock().lines().flatten() {
        let mut line_it = line.split(' ');
        let direction: &str = line_it.next().unwrap();
        let amount: i32 = line_it.next().and_then(|n| n.parse::<i32>().ok()).unwrap();
        match direction {
            "forward" => {
                position += amount;
                depth_v2 += aim * amount;
            }
            "down" => {
                depth += amount;
                aim += amount;
            }
            "up" => {
                depth -= amount;
                aim -= amount;
            }
            _ => {}
        }
    }

    println!(
        "(1) Final value is {} * {} = {}",
        position,
        depth,
        position * depth
    );
    println!(
        "(2) Final value is {} * {} = {}",
        position,
        depth_v2,
        position * depth_v2
    );

    Ok(())
}

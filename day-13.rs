use std::io::{self, Read};

fn find_first_bus(min_time: i64, bus_pattern: &Vec<Option<i64>>) -> (i64, i64) {
    bus_pattern
        .iter()
        .filter_map(|b| *b)
        .map(|b| (b, b - (min_time % b)))
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn find_bus_sequence(bus_pattern: &Vec<Option<i64>>) -> i64 {
    let schedule = bus_pattern
        .iter()
        .enumerate()
        .filter_map(|(idx, bus)| bus.and_then(|bus| Some((idx, bus))));
    let product = schedule.clone().map(|(_, b)| b).product::<i64>();
    schedule
        .map(|(idx, bus)| {
            let a = (bus - idx as i64) % bus;
            let m = product / bus;
            let (_, x, _) = egcd(m, bus);
            a * m * x
        })
        .sum::<i64>()
        % product
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;

    if let [min_time, buses] = input.split("\n").collect::<Vec<&str>>()[..] {
        let min_time: i64 = min_time.parse().unwrap();
        let bus_pattern: Vec<Option<i64>> =
            buses.split(",").map(|b| b.parse::<i64>().ok()).collect();

        let (next_bus, next_bus_ts) = find_first_bus(min_time, &bus_pattern);
        println!(
            "(1) First bus {} is in {} minutes = {}",
            next_bus,
            next_bus_ts,
            next_bus * next_bus_ts
        );

        let ts = find_bus_sequence(&bus_pattern);
        println!("(2) Bus pattern matches at {}", ts);
    }

    Ok(())
}

use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut max: u32 = 0;
    let mut min: u32 = u32::MAX;
    let mut sum: u32 = 0;

    for line in io::stdin().lock().lines() {
        let seat_id = line?
            .chars()
            .fold(0, |acc, c| (acc << 1) + (c == 'B' || c == 'R') as u32);
        max = max.max(seat_id);
        min = min.min(seat_id);
        sum += seat_id;
    }

    println!("(1) Maximum is {}", max);

    let expected = (max * (max + 1) - min * (min - 1)) / 2;
    println!("(2) Seat {} is open", expected - sum);

    Ok(())
}

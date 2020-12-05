use std::collections::BTreeSet;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut seats_taken = BTreeSet::new();
    for line in io::stdin().lock().lines() {
        let seat_id = line?
            .chars()
            .fold(0, |acc, c| acc * 2 + (c == 'B' || c == 'R') as u32);
        seats_taken.insert(seat_id);
    }

    if let Some(max) = seats_taken.iter().max() {
        println!("Maximum is {:?}", max);
    }

    let mut seat_iter = seats_taken.iter();
    if let Some(mut last_seat) = seat_iter.next().map(|s| s.clone()) {
        for &seat in seat_iter {
            if seat > last_seat + 1 {
                println!("Seat {} is open", seat - 1);
            }
            last_seat = seat;
        }
    }

    Ok(())
}

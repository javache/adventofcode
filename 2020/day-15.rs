use std::collections::HashMap;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;

    let numbers: Vec<u32> = input
        .split(",")
        .filter_map(|n| n.parse::<u32>().ok())
        .collect();

    let starting_turn = numbers.len();
    let mut last_number = numbers[starting_turn - 1];
    let mut state: HashMap<u32, u32> = numbers
        .iter()
        .enumerate()
        .map(|(i, &n)| (n, i as u32))
        .collect();
    for turn in (starting_turn as u32).. {
        if turn == 2020 {
            println!("(1) The 2020th number spoken was {}", last_number);
        } else if turn == 30000000 {
            println!("(2) The 30000000th number spoken was {}", last_number);
            break;
        }

        let next_number;
        if let Some(last_spoken) = state.get(&last_number) {
            next_number = turn - last_spoken - 1;
        } else {
            next_number = 0;
        }
        state.insert(last_number, turn - 1);
        last_number = next_number;
    }

    Ok(())
}

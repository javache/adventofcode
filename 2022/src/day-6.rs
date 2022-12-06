use std::collections::HashSet;
use std::io::{self, BufRead};

fn first_window_of_n_distinct_characters(input: &Vec<char>, n: usize) -> usize {
    for (idx, window) in input.windows(n).enumerate() {
        let set: HashSet<&char> = window.iter().collect();
        if set.len() == n {
            return idx + n;
        }
    }
    panic!("No index found");
}

fn main() -> io::Result<()> {
    let mut line: String = String::new();
    io::stdin().lock().read_line(&mut line)?;

    let chars: Vec<char> = line.chars().collect();
    println!(
        "(1) Start-of-packet marker found at {}",
        first_window_of_n_distinct_characters(&chars, 4)
    );
    println!(
        "(2) Start-of-message marker found at {}",
        first_window_of_n_distinct_characters(&chars, 14)
    );

    Ok(())
}

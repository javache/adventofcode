use std::collections::VecDeque;
use std::io::{self, BufRead};

const MAX_AGE: usize = 8;

fn run_iterations(initial_state: Vec<usize>, num_iters: usize) -> usize {
    let mut list = VecDeque::<usize>::from(initial_state);
    for _ in 0..num_iters {
        let front = list.pop_front().unwrap();
        list.push_back(front);
        list[MAX_AGE - 2] += front;
    }
    list.into_iter().sum()
}

fn main() -> io::Result<()> {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line)?;

    let mut initial_state = vec![0; MAX_AGE + 1];
    line.split(',')
        .filter_map(|n| n.parse::<usize>().ok())
        .for_each(|n| initial_state[n] += 1);

    println!(
        "(1) After 80 days, there are {} lanternfish",
        run_iterations(initial_state.clone(), 80)
    );
    println!(
        "(2) After 256 days, there are {} lanternfish",
        run_iterations(initial_state.clone(), 256)
    );

    Ok(())
}

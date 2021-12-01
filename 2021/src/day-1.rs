use std::collections::LinkedList;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut last_measurement = None;
    let mut num_increasing = 0;

    let mut window = LinkedList::new();
    let mut last_window_sum = None;
    let mut num_increasing_windows = 0;

    for line in io::stdin().lock().lines() {
        let num = line?.parse::<i32>().unwrap();

        if let Some(last_measurement) = last_measurement {
            num_increasing += (num > last_measurement) as i32;
        }
        last_measurement = Some(num);

        window.push_back(num);
        if window.len() == 3 {
            let window_sum: i32 = window.iter().sum();
            if let Some(last_window_sum) = last_window_sum {
                num_increasing_windows += (window_sum > last_window_sum) as i32;
            }
            last_window_sum = Some(window_sum);
            window.pop_front();
        }
    }

    println!("(1) {} entries are increasing", num_increasing);
    println!("(2) {} windows are increasing", num_increasing_windows);

    Ok(())
}

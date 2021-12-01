use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let split_chars: Vec<char> = vec!['-', ' ', ':'];

    let mut count_1: u32 = 0;
    let mut count_2: u32 = 0;

    for line in io::stdin().lock().lines() {
        // 1-8 n: dpwpmhknmnlglhjtrbpx
        let line = line.unwrap();

        let mut it = line.split(&split_chars[..]).filter(|l| l != &"");
        let lower_bound: usize = it.next().unwrap().parse().unwrap();
        let upper_bound: usize = it.next().unwrap().parse().unwrap();
        let c: char = it.next().unwrap().parse().unwrap();
        let password: &str = it.next().unwrap();

        let c_count = password.matches(c).count();
        if c_count >= lower_bound && c_count <= upper_bound {
            count_1 += 1;
        }

        let password_bytes: &[u8] = password.as_bytes();
        if (password_bytes[lower_bound - 1] == c as u8)
            ^ (password_bytes[upper_bound - 1] == c as u8)
        {
            count_2 += 1
        }
    }

    println!("(1) {}", count_1);
    println!("(2) {}", count_2);

    Ok(())
}

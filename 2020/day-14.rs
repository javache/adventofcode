use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut mask_or: u64 = 0;
    let mut mask_and: u64 = 0;
    let mut floating_bits: Vec<usize> = Vec::new();

    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut floating_memory: HashMap<u64, u64> = HashMap::new();

    for line in io::stdin().lock().lines() {
        let line = line?;
        if line.starts_with("mask") {
            let mask = &line["mask = ".len()..];
            mask_or = u64::from_str_radix(&mask.replace("X", "0"), 2).unwrap();
            mask_and = u64::from_str_radix(&mask.replace("X", "1"), 2).unwrap();
            floating_bits = mask
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == 'X')
                .map(|(i, _)| mask.len() - i - 1)
                .collect();
        } else if line.starts_with("mem") {
            if let [address, value] = &line["mem[".len()..].split("] = ").collect::<Vec<&str>>()[..]
            {
                let address: u64 = address.parse().unwrap();
                let value: u64 = value.parse().unwrap();
                memory.insert(address, (value | mask_or) & mask_and);

                let mut floating_addresses: HashSet<u64> =
                    vec![address | mask_or].into_iter().collect();
                for pos in &floating_bits {
                    for addr in floating_addresses.clone() {
                        floating_addresses.insert(addr | (1_u64 << pos));
                        floating_addresses.insert(addr & !(1_u64 << pos));
                    }
                }
                for addr in floating_addresses {
                    floating_memory.insert(addr, value);
                }
            }
        }
    }

    let result: u64 = memory.values().sum();
    println!("(1) Sum of all values in memory is {}", result);

    let result: u64 = floating_memory.values().sum();
    println!("(2) Sum of all floating values in memory is {}", result);

    Ok(())
}

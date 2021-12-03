use std::io::{self, BufRead};

fn calculate_majority_bits(input: &Vec<u32>, num_bits: usize) -> Vec<bool> {
    let mut bit_count = vec![0; num_bits];
    for n in input {
        for i in 0..num_bits {
            bit_count[i] += (n & (1 << i) != 0) as u32;
        }
    }
    bit_count
        .iter()
        .map(|count| count >= &((input.len() + 1) as u32 / 2))
        .collect()
}

fn main() -> io::Result<()> {
    let mut num_bits = 0;
    let mut input = vec![];
    for line in io::stdin().lock().lines().flatten() {
        num_bits = line.len();
        input.push(u32::from_str_radix(&line, 2).unwrap());
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    let majority_bits = calculate_majority_bits(&input, num_bits);
    for (i, is_majority_one) in majority_bits.iter().enumerate() {
        gamma |= (*is_majority_one as u32) << i;
        epsilon |= (!*is_majority_one as u32) << i;
    }

    let mut oxygen_rating = input.clone();
    for i in (0..num_bits).rev() {
        if oxygen_rating.len() == 1 {
            break;
        }
        let is_majority_one = calculate_majority_bits(&oxygen_rating, num_bits)[i];
        oxygen_rating = oxygen_rating
            .into_iter()
            .filter(|n| is_majority_one ^ ((n & (1 << i)) == 0))
            .collect();
    }
    assert!(oxygen_rating.len() == 1);

    let mut co2_rating = input.clone();
    for i in (0..num_bits).rev() {
        if co2_rating.len() == 1 {
            break;
        }
        let is_majority_one = calculate_majority_bits(&co2_rating, num_bits)[i];
        co2_rating = co2_rating
            .into_iter()
            .filter(|n| is_majority_one ^ ((n & (1 << i)) != 0))
            .collect();
    }
    assert!(co2_rating.len() == 1);

    println!(
        "(1) Power consumption is {:b} * {:b} = {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
    println!(
        "(2) Life support rating is {} * {} = {}",
        oxygen_rating[0],
        co2_rating[0],
        oxygen_rating[0] * co2_rating[0]
    );

    Ok(())
}

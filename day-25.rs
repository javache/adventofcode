use std::io::{self, Read};

fn find_loop_size(target: u32) -> u32 {
    let mut loop_size = 0;
    let mut value = 1;
    while value != target {
        value = (value * 7) % 20201227;
        loop_size += 1;
    }
    loop_size
}

fn transform(input: u32, loop_size: u32) -> u32 {
    (0..loop_size).fold(1, |acc, _| (acc * (input as u64)) % 20201227) as u32
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;

    if let [card_public, door_public] = input
        .split("\n")
        .map(|line| line.parse().unwrap())
        .collect::<Vec<u32>>()[..]
    {
        let card_loop_size = find_loop_size(card_public);
        let door_loop_size = find_loop_size(door_public);

        println!("(1) Encryption key = {} / {}", transform(door_public, card_loop_size), transform(card_public, door_loop_size));
    }

    Ok(())
}

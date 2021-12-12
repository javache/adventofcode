use itertools::iproduct;
use std::io::{self, BufRead};

fn execute_program(base_memory: &Vec<u32>, noun: u32, verb: u32) -> u32 {
    let mut memory = base_memory.clone();
    memory[1] = noun;
    memory[2] = verb;

    let mut ip = 0;
    while ip < memory.len() {
        match memory[ip] {
            1 => {
                let op1 = memory[ip + 1] as usize;
                let op2 = memory[ip + 2] as usize;
                let dest = memory[ip + 3] as usize;
                memory[dest] = memory[op1] + memory[op2];
            }
            2 => {
                let op1 = memory[ip + 1] as usize;
                let op2 = memory[ip + 2] as usize;
                let dest = memory[ip + 3] as usize;
                memory[dest] = memory[op1] * memory[op2];
            }
            99 => break,
            _ => panic!("Unexpected opcode {}", memory[ip]),
        }
        ip += 4;
    }
    memory[0]
}

fn main() -> io::Result<()> {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line)?;

    let memory = line.split(',').flat_map(str::parse).collect();
    println!(
        "(1) The first entry in memory is {}",
        execute_program(&memory, 12, 2)
    );

    let answer = iproduct!(0..100, 0..100)
        .find(|(noun, verb)| execute_program(&memory, *noun, *verb) == 19690720);
    if let Some((noun, verb)) = answer {
        println!(
            "(2) Using noun {} and verb {} leads to the expected output",
            noun, verb
        );
    }

    Ok(())
}

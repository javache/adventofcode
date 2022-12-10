use itertools::Itertools;
use std::io::{self, BufRead};

fn resolve_op(memory: &Vec<i32>, val: i32, mode: i32) -> i32 {
    match mode {
        0 => memory[val as usize],
        1 => val,
        _ => panic!("Unsupported mode {}", mode),
    }
}

fn execute_program(memory: &mut Vec<i32>, ip: &mut usize, inputs: &Vec<i32>) -> Option<i32> {
    let mut input_it = inputs.iter();
    while *ip < memory.len() {
        let (opcode, op_mode) = (memory[*ip] % 100, memory[*ip] / 100);
        match opcode {
            1 => {
                let op1 = resolve_op(&memory, memory[*ip + 1], op_mode % 10);
                let op2 = resolve_op(&memory, memory[*ip + 2], (op_mode / 10) % 10);
                let dest = memory[*ip + 3] as usize;
                memory[dest] = op1 + op2;
                *ip += 4;
            }
            2 => {
                let op1 = resolve_op(&memory, memory[*ip + 1], op_mode % 10);
                let op2 = resolve_op(&memory, memory[*ip + 2], (op_mode / 10) % 10);
                let dest = memory[*ip + 3] as usize;
                memory[dest] = op1 * op2;
                *ip += 4;
            }
            3 => {
                let dest = memory[*ip + 1] as usize;
                memory[dest] = *input_it.next().unwrap();
                *ip += 2;
            }
            4 => {
                let op1 = resolve_op(&memory, memory[*ip + 1], op_mode % 10);
                *ip += 2;
                return Some(op1);
            }
            5 => {
                let op1 = resolve_op(&memory, memory[*ip + 1], op_mode % 10);
                let op2 = resolve_op(&memory, memory[*ip + 2], (op_mode / 10) % 10);
                if op1 != 0 {
                    *ip = op2 as usize;
                } else {
                    *ip += 3;
                }
            }
            6 => {
                let op1 = resolve_op(&memory, memory[*ip + 1], op_mode % 10);
                let op2 = resolve_op(&memory, memory[*ip + 2], (op_mode / 10) % 10);
                if op1 == 0 {
                    *ip = op2 as usize;
                } else {
                    *ip += 3;
                }
            }
            7 => {
                let op1 = resolve_op(&memory, memory[*ip + 1], op_mode % 10);
                let op2 = resolve_op(&memory, memory[*ip + 2], (op_mode / 10) % 10);
                let dest = memory[*ip + 3] as usize;
                memory[dest] = (op1 < op2) as i32;
                *ip += 4;
            }
            8 => {
                let op1 = resolve_op(&memory, memory[*ip + 1], op_mode % 10);
                let op2 = resolve_op(&memory, memory[*ip + 2], (op_mode / 10) % 10);
                let dest = memory[*ip + 3] as usize;
                memory[dest] = (op1 == op2) as i32;
                *ip += 4;
            }
            99 => break,
            _ => panic!("Unexpected opcode {}", memory[*ip]),
        }
    }

    None
}

fn main() -> io::Result<()> {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line)?;

    let memory: Vec<i32> = line.split(',').flat_map(str::parse).collect();

    let max = (0..=4)
        .permutations(5)
        .map(|settings| {
            (0..5).fold(0, |acc, i| {
                execute_program(&mut memory.clone(), &mut 0, &vec![settings[i], acc]).unwrap()
            })
        })
        .max()
        .unwrap();
    println!("(1) Optimal settings for the program return {}", max);

    let max = (5..=9)
        .permutations(5)
        .map(|settings| {
            let mut next_input = 0;
            let mut memory = vec![memory.clone(); 5];
            let mut ip = vec![0; 5];
            (0..5)
                .cycle()
                .find_map(|i| {
                    let input = if ip[i] == 0 {
                        vec![settings[i], next_input]
                    } else {
                        vec![next_input]
                    };
                    if let Some(output) = execute_program(&mut memory[i], &mut ip[i], &input) {
                        next_input = output;
                        None
                    } else {
                        assert_eq!(i, 0);
                        Some(next_input)
                    }
                })
                .unwrap()
        })
        .max()
        .unwrap();
    println!("(2) Optimal settings for the program return {}", max);

    Ok(())
}

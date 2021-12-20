use std::io::{self, BufRead};

fn resolve_op(memory: &Vec<i32>, val: i32, mode: i32) -> i32 {
    match mode {
        0 => memory[val as usize],
        1 => val,
        _ => panic!("Unsupported mode {}", mode),
    }
}

fn execute_program(base_memory: &Vec<i32>, input: i32) {
    let mut memory = base_memory.clone();
    let mut ip = 0;
    while ip < memory.len() {
        let (opcode, op_mode) = (memory[ip] % 100, memory[ip] / 100);
        match opcode {
            1 => {
                let op1 = resolve_op(&memory, memory[ip + 1], op_mode % 10);
                let op2 = resolve_op(&memory, memory[ip + 2], (op_mode / 10) % 10);
                let dest = memory[ip + 3] as usize;
                memory[dest] = op1 + op2;
                ip += 4;
            }
            2 => {
                let op1 = resolve_op(&memory, memory[ip + 1], op_mode % 10);
                let op2 = resolve_op(&memory, memory[ip + 2], (op_mode / 10) % 10);
                let dest = memory[ip + 3] as usize;
                memory[dest] = op1 * op2;
                ip += 4;
            }
            3 => {
                let dest = memory[ip + 1] as usize;
                memory[dest] = input;
                ip += 2;
            }
            4 => {
                let op1 = resolve_op(&memory, memory[ip + 1], op_mode % 10);
                println!("[OUT] {}", op1);
                ip += 2;
            }
            5 => {
                let op1 = resolve_op(&memory, memory[ip + 1], op_mode % 10);
                let op2 = resolve_op(&memory, memory[ip + 2], (op_mode / 10) % 10);
                if op1 != 0 {
                    ip = op2 as usize;
                } else {
                    ip += 3;
                }
            }
            6 => {
                let op1 = resolve_op(&memory, memory[ip + 1], op_mode % 10);
                let op2 = resolve_op(&memory, memory[ip + 2], (op_mode / 10) % 10);
                if op1 == 0 {
                    ip = op2 as usize;
                } else {
                    ip += 3;
                }
            }
            7 => {
                let op1 = resolve_op(&memory, memory[ip + 1], op_mode % 10);
                let op2 = resolve_op(&memory, memory[ip + 2], (op_mode / 10) % 10);
                let dest = memory[ip + 3] as usize;
                memory[dest] = (op1 < op2) as i32;
                ip += 4;
            }
            8 => {
                let op1 = resolve_op(&memory, memory[ip + 1], op_mode % 10);
                let op2 = resolve_op(&memory, memory[ip + 2], (op_mode / 10) % 10);
                let dest = memory[ip + 3] as usize;
                memory[dest] = (op1 == op2) as i32;
                ip += 4;
            }
            99 => break,
            _ => panic!("Unexpected opcode {}", memory[ip]),
        }
    }
}

fn main() -> io::Result<()> {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line)?;

    let memory = line.split(',').flat_map(str::parse).collect();

    println!("(1) Running the program with input 1");
    execute_program(&memory, 1);

    println!("(1) Running the program with input 5",);
    execute_program(&memory, 5);

    Ok(())
}

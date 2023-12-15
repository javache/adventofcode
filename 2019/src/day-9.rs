use std::io::{self, BufRead};

fn resolve_op(memory: &Vec<i64>, val: i64, mode: i64, rel_base: i64) -> i64 {
    match mode {
        0 => memory[val as usize],
        1 => val,
        2 => memory[(rel_base + val) as usize],
        _ => panic!("Unsupported mode {}", mode),
    }
}

fn resolve_addr(val: i64, mode: i64, rel_base: i64) -> usize {
    match mode {
        0 => val as usize,
        1 => panic!("Invalid addressing mode"),
        2 => (rel_base + val) as usize,
        _ => panic!("Unsupported mode {}", mode),
    }
}

fn execute_program(base_memory: &Vec<i64>, input: i64) {
    let mut memory = base_memory.clone();
    memory.resize(base_memory.len() + 2048, 0);

    let mut ip = 0;
    let mut rel_base = 0;
    while ip < memory.len() {
        let (opcode, op_mode) = (memory[ip] % 100, memory[ip] / 100);
        match opcode {
            1 => {
                let op1 = resolve_op(&memory, memory[ip + 1], op_mode % 10, rel_base);
                let op2 = resolve_op(&memory, memory[ip + 2], (op_mode / 10) % 10, rel_base);
                let dest = resolve_addr(memory[ip + 3], (op_mode / 100) % 10, rel_base);
                memory[dest] = op1 + op2;
                ip += 4;
            }
            2 => {
                let op1 = resolve_op(&memory, memory[ip + 1], op_mode % 10, rel_base);
                let op2 = resolve_op(&memory, memory[ip + 2], (op_mode / 10) % 10, rel_base);
                let dest = resolve_addr(memory[ip + 3], (op_mode / 100) % 10, rel_base);
                memory[dest] = op1 * op2;
                ip += 4;
            }
            3 => {
                let mut dest = memory[ip + 1];
                if op_mode == 2 {
                    dest += rel_base;
                }
                memory[dest as usize] = input;
                ip += 2;
            }
            4 => {
                let op1 = resolve_op(&memory, memory[ip + 1], op_mode % 10, rel_base);
                println!("[OUT] {}", op1);
                ip += 2;
            }
            5 => {
                let op1 = resolve_op(&memory, memory[ip + 1], op_mode % 10, rel_base);
                let op2 = resolve_op(&memory, memory[ip + 2], (op_mode / 10) % 10, rel_base);
                if op1 != 0 {
                    ip = op2 as usize;
                } else {
                    ip += 3;
                }
            }
            6 => {
                let op1 = resolve_op(&memory, memory[ip + 1], op_mode % 10, rel_base);
                let op2 = resolve_op(&memory, memory[ip + 2], (op_mode / 10) % 10, rel_base);
                if op1 == 0 {
                    ip = op2 as usize;
                } else {
                    ip += 3;
                }
            }
            7 => {
                let op1 = resolve_op(&memory, memory[ip + 1], op_mode % 10, rel_base);
                let op2 = resolve_op(&memory, memory[ip + 2], (op_mode / 10) % 10, rel_base);
                let dest = resolve_addr(memory[ip + 3], (op_mode / 100) % 10, rel_base);
                memory[dest] = (op1 < op2) as i64;
                ip += 4;
            }
            8 => {
                let op1 = resolve_op(&memory, memory[ip + 1], op_mode % 10, rel_base);
                let op2 = resolve_op(&memory, memory[ip + 2], (op_mode / 10) % 10, rel_base);
                let dest = resolve_addr(memory[ip + 3], (op_mode / 100) % 10, rel_base);
                memory[dest] = (op1 == op2) as i64;
                ip += 4;
            }
            9 => {
                let op1 = resolve_op(&memory, memory[ip + 1], op_mode % 10, rel_base);
                rel_base += op1;
                ip += 2;
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

    println!("(2) Running the program with input 2");
    execute_program(&memory, 2);

    Ok(())
}

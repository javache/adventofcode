use std::collections::HashSet;
use std::io::{self, BufRead};
use std::mem;

#[derive(Debug)]
struct Instruction {
    instr_type: String,
    value: i32,
}

fn run_program(instructions: &Vec<Instruction>) -> (i32, bool) {
    let (mut acc, mut ip) = (0_i32, 0);
    let mut instructions_seen = HashSet::new();
    while ip < instructions.len() && !instructions_seen.contains(&ip) {
        instructions_seen.insert(ip);
        let instruction = &instructions[ip];
        match instruction.instr_type.as_ref() {
            "nop" => {
                ip += 1;
            }
            "acc" => {
                acc += instruction.value;
                ip += 1;
            }
            "jmp" => {
                ip = (ip as i32 + instruction.value) as usize;
            }
            _ => unreachable!(),
        }
    }
    (acc, ip == instructions.len())
}

fn main() -> io::Result<()> {
    let mut instructions = Vec::new();
    for line in io::stdin().lock().lines() {
        if let [instr, value_str] = line?.split(' ').collect::<Vec<&str>>()[..] {
            if let Ok(value) = value_str.parse::<i32>() {
                instructions.push(Instruction {
                    instr_type: instr.to_string(),
                    value,
                });
            }
        }
    }

    let (acc, _) = run_program(&instructions);
    println!("(1) Accumulator is {}", acc);

    for i in 0..instructions.len() {
        let instr_type = &instructions[i].instr_type;
        if instr_type == "acc" {
            continue;
        }

        let new_instr = match instr_type.as_ref() {
            "jmp" => "nop",
            _ => "jmp",
        };
        let old_instr = mem::replace(&mut instructions[i].instr_type, new_instr.to_string());
        let (acc, did_terminate) = run_program(&instructions);
        instructions[i].instr_type = old_instr;

        if did_terminate {
            println!("(2) Accumulator is {}", acc);
            break;
        }
    }

    Ok(())
}

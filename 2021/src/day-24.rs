use std::collections::HashMap;
use std::io::{self, BufRead};

enum Param {
    Register(usize),
    Immediate(i64),
}

impl Param {
    fn from_str(input: &str) -> Param {
        input
            .parse()
            .map(|num| Param::Immediate(num))
            .unwrap_or_else(|_| Param::Register(Param::parse_register(input)))
    }

    fn parse_register(register: &str) -> usize {
        ((register.chars().next().unwrap() as u8) - b'w') as usize
    }

    fn evaluate(&self, mem: &[i64]) -> i64 {
        match self {
            Param::Register(reg) => mem[*reg],
            Param::Immediate(value) => *value,
        }
    }
}

enum Instruction {
    Inp(usize),
    Add(usize, Param),
    Mul(usize, Param),
    Div(usize, Param),
    Mod(usize, Param),
    Eql(usize, Param),
}

impl Instruction {
    fn parse(input: &str) -> Instruction {
        match &input.split(' ').collect::<Vec<&str>>()[..] {
            [op, reg] => {
                assert_eq!(op, &"inp");
                Instruction::Inp(Param::parse_register(reg))
            }
            [op, reg, param] => {
                let reg = Param::parse_register(reg);
                let param = Param::from_str(param);
                match *op {
                    "add" => Instruction::Add(reg, param),
                    "mul" => Instruction::Mul(reg, param),
                    "div" => Instruction::Div(reg, param),
                    "mod" => Instruction::Mod(reg, param),
                    "eql" => Instruction::Eql(reg, param),
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }

    fn evaluate(&self, mem: &mut [i64], input: Option<i64>) {
        match self {
            Instruction::Inp(reg) => mem[*reg] = input.unwrap(),
            Instruction::Add(reg, param) => mem[*reg] += param.evaluate(&mem),
            Instruction::Mul(reg, param) => mem[*reg] *= param.evaluate(&mem),
            Instruction::Div(reg, param) => mem[*reg] /= param.evaluate(&mem),
            Instruction::Mod(reg, param) => mem[*reg] %= param.evaluate(&mem),
            Instruction::Eql(reg, param) => {
                mem[*reg] = if mem[*reg] == param.evaluate(&mem) {
                    1
                } else {
                    0
                }
            }
        }
    }
}

fn solve(
    instructions: &Vec<Instruction>,
    pc: usize,
    mem: [i64; 4],
    biggest: bool,
    seen: &mut HashMap<([i64; 4], usize), Option<i64>>,
) -> Option<i64> {
    assert!(matches!(instructions[pc], Instruction::Inp(_)));

    if let Some(answer) = seen.get(&(mem, pc)) {
        return *answer;
    }

    let range = if biggest {
        [9, 8, 7, 6, 5, 4, 3, 2, 1]
    } else {
        [1, 2, 3, 4, 5, 6, 7, 8, 9]
    };

    let solution = range.into_iter().find_map(|input| {
        let mut mem = mem;
        instructions[pc].evaluate(&mut mem, Some(input));

        for (pc, instr) in instructions[..].iter().enumerate().skip(pc + 1) {
            if let Instruction::Inp(_) = instr {
                return solve(instructions, pc, mem, biggest, seen).map(|best| best * 10 + input);
            } else {
                instr.evaluate(&mut mem, None);
            }
        }

        return if mem[3] == 0 { Some(input) } else { None };
    });

    seen.insert((mem, pc), solution);
    solution
}

fn reverse_num(num: i64) -> String {
    format!("{}", num).chars().rev().collect()
}

fn main() -> io::Result<()> {
    let input = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| Instruction::parse(&line))
        .collect::<Vec<Instruction>>();

    println!(
        "(1) Biggest model number is {}",
        reverse_num(solve(&input, 0, [0; 4], true, &mut HashMap::new()).unwrap())
    );
    println!(
        "(2) Smallest model number is {}",
        reverse_num(solve(&input, 0, [0; 4], false, &mut HashMap::new()).unwrap())
    );

    Ok(())
}

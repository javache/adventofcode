use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
enum Entry {
    Num(i64),
    Op(char),
    Parentheses,
}

fn perform_op(input: &[Entry]) -> Entry {
    if let [Entry::Num(a), Entry::Op(c), Entry::Num(b)] = input {
        Entry::Num(if *c == '*' { a * b } else { a + b })
    } else {
        unreachable!();
    }
}

fn reduce_stack(stack: &mut Vec<Entry>, order_matters: bool) -> Entry {
    let mut input: Vec<Entry>;
    if let Some(parentheses_idx) = stack.iter().rposition(|e| e == &Entry::Parentheses) {
        input = stack.drain(parentheses_idx..).skip(1).collect();
    } else {
        input = stack.drain(0..).collect();
    }

    if order_matters {
        let mut idx = input.len() - 1;
        while idx >= 2 && input.len() >= 3 {
            if input[idx - 1] == Entry::Op('+') {
                let result = perform_op(input.drain((idx - 2)..(idx + 1)).as_slice());
                input.insert(idx - 2, result);
            }
            idx -= 2;
        }
    }

    while input.len() >= 3 {
        let result = perform_op(input.drain(0..3).as_slice());
        input.insert(0, result);
    }

    assert!(input.len() == 1);
    input.pop().unwrap()
}

fn main() -> io::Result<()> {
    let mut sum: [i64; 2] = [0, 0];
    for line in io::stdin().lock().lines() {
        let line = line?;
        for &order_matters in &[false, true] {
            let mut pos = 0;
            let mut stack: Vec<Entry> = Vec::new();
            for (idx, c) in line.chars().enumerate() {
                if c.is_ascii_whitespace() || c == ')' {
                    if idx > pos {
                        let num = line[pos..idx].parse().unwrap();
                        stack.push(Entry::Num(num));
                    }
                } else if c.is_digit(10) {
                    continue;
                }

                if c == '*' || c == '+' {
                    stack.push(Entry::Op(c));
                } else if c == '(' {
                    stack.push(Entry::Parentheses);
                } else if c == ')' {
                    let res = reduce_stack(&mut stack, order_matters);
                    stack.push(res);
                }
                pos = idx + 1;
            }
            if line.len() > pos {
                let num = line[pos..line.len()].parse().unwrap();
                stack.push(Entry::Num(num));
            }

            if let Entry::Num(res) = reduce_stack(&mut stack, order_matters) {
                sum[order_matters as usize] += res;
            }
        }
    }

    println!("(1) Sum of expressions without precedence is {}", sum[0]);
    println!("(2) Sum of expressions with precedence is {}", sum[1]);

    Ok(())
}

use std::collections::HashSet;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut any_sum = 0;
    let mut all_sum = 0;

    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    for group in input.split("\n\n") {
        let mut any_answers = HashSet::<char>::new();
        let mut all_answers = HashSet::<char>::new();

        for (i, member) in group.split_whitespace().enumerate() {
            let answers: HashSet<char> = member.chars().collect();
            any_answers.extend(&answers);
            if i == 0 {
                all_answers = answers;
            } else {
                all_answers = all_answers.intersection(&answers).cloned().collect();
            }
        }

        any_sum += any_answers.len();
        all_sum += all_answers.len();
    }

    println!("(1) Sum is {}", any_sum);
    println!("(2) Sum is {}", all_sum);

    Ok(())
}

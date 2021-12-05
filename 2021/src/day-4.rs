use std::io::{self, Read};
use std::iter::Iterator;

type Board = Vec<Vec<u32>>;

fn check_columns(board: &Board, called: &[u32]) -> bool {
    board
        .iter()
        .any(|row| row.iter().all(|n| called.contains(n)))
}

fn check_rows(board: &Board, called: &[u32]) -> bool {
    (0..board[0].len()).any(|i| board.iter().all(|row| called.contains(&row[i])))
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;

    let mut numbers: Vec<u32> = vec![];
    let mut boards: Vec<Board> = vec![];

    input.split("\n\n").enumerate().for_each(|(idx, block)| {
        if idx == 0 {
            numbers = block.split(',').filter_map(|n| n.parse().ok()).collect();
        } else {
            let mut board: Board = vec![];
            for row in block.split('\n') {
                board.push(
                    row.split_whitespace()
                        .filter_map(|n| n.parse().ok())
                        .collect(),
                )
            }
            boards.push(board);
        }
    });

    for (round, curr_number) in numbers.iter().enumerate() {
        let numbers_called = &numbers[0..=round];
        boards = boards
            .into_iter()
            .filter(|board| {
                let bingo =
                    check_columns(board, numbers_called) || check_rows(board, numbers_called);
                if bingo {
                    let uncalled_numbers: u32 = board
                        .iter()
                        .flatten()
                        .filter(|n| !numbers_called.contains(n))
                        .sum();
                    println!(
                        "Board won in round {} (score: {})",
                        round,
                        uncalled_numbers * curr_number
                    );
                }
                !bingo
            })
            .collect();
    }

    Ok(())
}

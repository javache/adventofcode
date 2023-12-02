use std::io::{self, BufRead};

fn main() {
    let games = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| {
            let input_start = line.find(":").unwrap();
            let game_id = line["Game ".len()..input_start].parse::<u32>().unwrap();

            let game_value = line[(input_start + 1)..]
                .split(";")
                .map(|round_input| {
                    let mut value: [u32; 3] = [0, 0, 0];
                    for cubes in round_input.split(", ") {
                        if let [num, cube_type] = cubes.trim().split(' ').collect::<Vec<&str>>()[..]
                        {
                            value[match cube_type {
                                "red" => 0,
                                "green" => 1,
                                "blue" => 2,
                                _ => panic!("Unsupported value {:?}", cube_type),
                            }] = num.parse::<u32>().unwrap();
                        }
                    }
                    value
                })
                .fold([0, 0, 0], |[r1, g1, b1], [r2, g2, b2]| {
                    [r1.max(r2), g1.max(g2), b1.max(b2)]
                });
            (game_id, game_value)
        })
        .collect::<Vec<_>>();

    let valid_games = games
        .iter()
        .filter(|(_, [r, g, b])| *r <= 12 && *g <= 13 && *b <= 14);
    println!(
        "(1) The sum of valid game IDs is {}",
        valid_games.map(|(id, _)| id).sum::<u32>()
    );

    let powers: u32 = games
        .iter()
        .map(|(_, cubes)| cubes.iter().product::<u32>())
        .sum();
    println!("(2) The sum of the powers of all games is {}", powers);
}

use std::collections::VecDeque;
use std::io::{self, Read};

type Image = Vec<Vec<bool>>;

fn read(image: &Image, i: i32, j: i32, default: bool) -> bool {
    (i >= 0 && (i as usize) < image.len() && j >= 0 && (j as usize) < image[0].len())
        .then(|| image[i as usize][j as usize])
        .unwrap_or(default)
}

fn enhance_image(input: &Image, algo: &Vec<bool>, default: bool) -> Image {
    let new_width = input[0].len() + 2 * 2;
    let new_height = input.len() + 2 * 2;

    let mut output = vec![vec![false; new_width]; new_height];
    let mut convolution = VecDeque::from([default; 9]);
    for i in 0..(new_height as i32) {
        convolution.drain(0..3);
        convolution.extend([default; 3]);

        for j in 0..(new_width as i32) {
            convolution.pop_front();
            convolution[2] = read(input, i - 1 - 2, j - 2, default);
            convolution[5] = read(input, i - 2, j - 2, default);
            convolution.push_back(read(input, i + 1 - 2, j - 2, default));

            let number: usize = convolution.iter().fold(0, |acc, v| acc * 2 + (*v as usize));
            output[i as usize][j as usize] = algo[number];
        }
    }
    output
}

fn solve(input: &Image, algo: &Vec<bool>, iterations: usize) -> usize {
    let result = (0..iterations).fold(input.clone(), |curr, i| {
        enhance_image(&curr, &algo, if i > 0 { curr[0][0] } else { false })
    });
    result
        .iter()
        .map(|row| row.iter().filter(|p| **p).count())
        .sum()
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;

    if let [algo, image] = input.split("\n\n").collect::<Vec<&str>>()[..] {
        let algo: Vec<bool> = algo.chars().map(|c| c == '#').collect();
        let input: Image = image
            .split("\n")
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();

        println!(
            "(1) After 2 enhancements, there are {} pixels lit",
            solve(&input, &algo, 2)
        );
        println!(
            "(2) After 50 enhancements, there are {} pixels lit",
            solve(&input, &algo, 50)
        );
    }

    Ok(())
}

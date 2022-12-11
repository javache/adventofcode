use itertools::Itertools;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line)?;

    let layers: Vec<String> = line
        .chars()
        .chunks(25 * 6)
        .into_iter()
        .map(|chars| chars.collect())
        .collect();

    let layer_with_fewest_0 = layers
        .iter()
        .map(|layer| layer.chars().filter(|c| c == &'0').count())
        .enumerate()
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(idx, _)| &layers[idx])
        .unwrap();

    let result = layer_with_fewest_0.chars().filter(|c| c == &'1').count()
        * layer_with_fewest_0.chars().filter(|c| c == &'2').count();
    println!(
        "(1) Layer with fewest 0 digits contains {} 1*2 digits",
        result
    );

    let output = layers
        .into_iter()
        .rev()
        .reduce(|prev_layer, curr_layer| {
            curr_layer
                .chars()
                .zip(prev_layer.chars())
                .map(|(curr, prev)| if curr == '2' { prev } else { curr })
                .collect()
        })
        .unwrap()
        .chars()
        .map(|c| if c == '1' { '█' } else { ' ' })
        .chunks(25)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .join("\n");
    println!("(2) The combined layer shows\n{}", output);

    Ok(())
}

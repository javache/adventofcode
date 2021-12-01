use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut map: Vec<Vec<bool>> = Vec::new();

    for line in io::stdin().lock().lines() {
        // 1-8 n: dpwpmhknmnlglhjtrbpx
        let line = line.unwrap();
        let row = line.chars().map(|c| c == '#');
        map.push(row.collect());
    }

    let slopes = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    let mut product = 1;
    for slope in slopes {
        let mut tree_count = 0;
        let mut offset = 0;
        for row in (0..map.len()).step_by(slope.0) {
            if map[row][offset] {
                tree_count += 1;
            }
            offset = (offset + slope.1) % map[row].len();
        }
        println!("Slope {:?} has {} trees", slope, tree_count);
        product *= tree_count;
    }
    println!("Product is {}", product);

    Ok(())
}

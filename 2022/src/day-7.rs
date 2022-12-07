use std::collections::HashMap;
use std::io::{self, BufRead};

type Contents = HashMap<String, FSEntry>;

#[derive(Debug)]
enum FSEntry {
    File(usize),
    Dir(Contents),
}

fn fs_get_subdir<'a>(dir: &'a mut Contents, name: &str) -> &'a mut Contents {
    match dir.get_mut(name) {
        Some(FSEntry::Dir(contents)) => contents,
        _ => panic!("Directory {} not found", name),
    }
}

fn read_fs() -> Contents {
    let mut fs_root: Contents = HashMap::new();
    let mut curr_dir = &mut fs_root;
    let mut dir_path: Vec<String> = vec![];

    for line in io::stdin().lock().lines().flatten() {
        match line.split(' ').collect::<Vec<&str>>()[..] {
            ["$", "cd", ".."] => {
                dir_path.pop();
                curr_dir = dir_path
                    .iter()
                    .fold(&mut fs_root, |curr, name| fs_get_subdir(curr, name));
            }
            ["$", "cd", dir_name] => {
                if dir_name == "/" {
                    curr_dir = &mut fs_root;
                    dir_path.clear();
                } else {
                    curr_dir = fs_get_subdir(curr_dir, dir_name);
                    dir_path.push(dir_name.to_string());
                }
            }
            ["$", "ls"] => {}
            ["$", ..] => panic!("Unexpected command {}", line),
            ["dir", name] => {
                curr_dir.insert(name.to_string(), FSEntry::Dir(HashMap::new()));
            }
            [size, name] => {
                curr_dir.insert(name.to_string(), FSEntry::File(size.parse().unwrap()));
            }
            _ => panic!("Unexpected input {}", line),
        }
    }
    fs_root
}

fn calculate_dir_sizes(root: &Contents, mut dir_sizes: &mut Vec<usize>) -> usize {
    let sum = root
        .values()
        .map(|entry| match entry {
            FSEntry::Dir(contents) => calculate_dir_sizes(contents, &mut dir_sizes),
            FSEntry::File(size) => *size,
        })
        .sum();
    dir_sizes.push(sum);
    sum
}

fn main() {
    let mut dir_sizes = vec![];
    let total_size = calculate_dir_sizes(&read_fs(), &mut dir_sizes);

    println!(
        "(1) Sum of small directory sizes is {}",
        dir_sizes.iter().filter(|s| *s <= &100_000).sum::<usize>(),
    );
    let target = total_size - (70_000_000 - 30_000_000);
    println!(
        "(2) Smallest dir larger than target is {}",
        dir_sizes.iter().filter(|s| *s > &target).min().unwrap()
    );
}

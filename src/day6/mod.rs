use std::collections::HashSet;

pub fn run() {
    let file_contents = include_str!("./input.txt");
    println!("part1: {}", part1(&file_contents));
    println!("part2: {}", part2(&file_contents));
}

fn part1(file_contents: &str) -> usize {
    // despite the trim, an extra empty shows up at the beginning and end - thus the filter
    let window_size = 4;
    let chars = file_contents
        .trim()
        .split("")
        .filter(|x| &x[..] != "")
        .collect::<Vec<_>>();
    let result = &chars[..]
        .windows(window_size)
        .map(|x| are_all_values_unique(&x))
        .enumerate()
        .find(|(_i, istarget)| *istarget)
        .unwrap();
    result.0 + window_size
}

fn part2(file_contents: &str) -> usize {
    // despite the trim, an extra empty shows up at the beginning and end - thus the filter
    let window_size = 14;
    let chars = file_contents
        .trim()
        .split("")
        .filter(|x| &x[..] != "")
        .collect::<Vec<_>>();
    let result = &chars[..]
        .windows(window_size)
        .map(|x| are_all_values_unique(&x))
        .enumerate()
        .find(|(_i, istarget)| *istarget)
        .unwrap();
    result.0 + window_size
}

fn are_all_values_unique(values: &[&str]) -> bool {
    let mut myset: HashSet<&str> = HashSet::new();
    for x in values {
        if myset.contains(x) {
            return false;
        }

        myset.insert(x);
    }

    true
}

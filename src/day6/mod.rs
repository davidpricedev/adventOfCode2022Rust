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

/**
Couldn't figure out how to import the [set](https://docs.rs/sets/latest/sets/) library, so I'm implementing my own duplicate detection
via looping over the list once, and for each item, counting the number of times that item appears.
This is O(n^2)
*/
fn are_all_values_unique(values: &[&str]) -> bool {
    for i in 0..values.len() {
        if values.iter().filter(|y| values[i] == **y).count() > 1 {
            return false;
        }
    }

    true
}

use std::collections::HashSet;

pub fn run() {
    let file_contents = include_str!("./input.txt");
    println!("part1: {}", part1(file_contents));
    // println!("part2: {}", part2(file_contents));
}

fn part1(contents: &str) -> i32 {
    contents.trim().lines().map(|x| find_overlap(x)).sum()
}

fn find_overlap(line: &str) -> i32 {
    let halves = split_in_half(line);
    let mut a: HashSet<char> = HashSet::new();
    halves[0].chars().for_each(|c| {
        a.insert(c);
    });
    let mut b: HashSet<char> = HashSet::new();
    halves[1].chars().for_each(|c| {
        b.insert(c);
    });
    item_to_value(*(a.intersection(&b).collect::<Vec<&char>>()[0]))
}

fn item_to_value(ch: char) -> i32 {
    let lower_shift = |x| (x as i32) - ('a' as i32 - 1);
    let upper_shift = |x| (x as i32) - ('A' as i32 - 27);
    if ch.is_uppercase() {
        upper_shift(ch)
    } else {
        lower_shift(ch)
    }
}

fn split_in_half(line: &str) -> [&str; 2] {
    let half = line.len() / 2;
    [&line[..half], &line[half..]]
}

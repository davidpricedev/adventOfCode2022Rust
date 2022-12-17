use std::collections::HashSet;

pub fn run() {
    let file_contents = include_str!("./input.txt");
    println!("part1: {}", part1(file_contents));
    println!("part2: {}", part2(file_contents));
}

fn part1(contents: &str) -> i32 {
    contents.trim().lines().map(|x| find_overlap_value(x)).sum()
}

fn part2(contents: &str) -> i32 {
    contents
        .trim()
        .lines()
        .collect::<Vec<_>>()
        .windows(3)
        .step_by(3)
        .map(find_3way_overlap_value)
        .sum()
}

fn find_overlap_value(line: &str) -> i32 {
    let halves = split_in_half(line);
    let a: HashSet<char> = halves[0].chars().collect();
    let b: HashSet<char> = halves[1].chars().collect();
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

/**
 * Couldn't find a way to stay sane and do this in a reducer/fold function
 * This is somewhat obnoxious, wish there was a more elegant way to do this
 */
fn find_3way_overlap_value(lines: &[&str]) -> i32 {
    let a: HashSet<char> = lines[0].chars().collect();
    let b: HashSet<char> = lines[1].chars().collect();
    let c: HashSet<char> = lines[2].chars().collect();
    let ab: HashSet<char> = a.intersection(&b).map(|x| (*x).clone()).collect();
    let bc: HashSet<char> = b.intersection(&c).map(|x| (*x).clone()).collect();
    let mut abc = ab.intersection(&bc);
    item_to_value(*(abc.next().unwrap()))
}

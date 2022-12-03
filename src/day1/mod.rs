use itertools::Itertools;
use std::fs;

pub fn run() {
    println!("part1: {}", day1_part1());
    println!("part2: {}", day1_part2());
}

fn day1_part1() -> i32 {
    let file_path = "src/day1/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let sums = get_sums_by_elf(contents.as_str());
    return sums.max().unwrap();
}

fn day1_part2() -> i32 {
    let file_path = "src/day1/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let sums = get_sums_by_elf(contents.as_str());
    let mut vsums: Vec<_> = sums.collect_vec();
    vsums.sort();
    return last_n(&vsums, 3).iter().sum();
}

fn get_sums_by_elf(file_content: &str) -> impl Iterator<Item = i32> + '_ {
    let chunks = file_content.split("\n\n");
    return chunks.map(|chunk| {
        let lines = chunk.split("\n");
        let int_lines = lines
            .map(|y| y.parse::<i32>())
            .filter(|y| y.is_ok())
            .map(|y| y.unwrap());
        return int_lines.sum();
    });
}

fn last_n<T>(v: &Vec<T>, n: usize) -> &[T] {
    return &v[(v.len() - n)..];
}

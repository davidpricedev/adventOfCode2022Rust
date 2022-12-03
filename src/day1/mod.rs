// use std::env;
use std::fs;

pub fn run() {
    println!("{}", day1_part1());
}

fn day1_part1() -> i32 {
    let file_path = "src/day1/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let chunks = contents.as_str().split("\n\n");
    let sums = chunks.map(|chunk| {
        let lines = chunk.split("\n");
        let int_lines = lines
            .map(|y| y.parse::<i32>())
            .filter(|y| y.is_ok())
            .map(|y| y.unwrap());
        return int_lines.reduce(|acc, z| acc + z).unwrap();
    });
    return sums.reduce(|acc, z| if acc > z { acc } else { z }).unwrap();
}

// fn day1_part2() {
//     println!("Hello, from part2!");
// }

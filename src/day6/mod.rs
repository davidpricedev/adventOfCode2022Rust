use std::fs;

const FILE_PATH: &str = "src/day6/input.txt";

pub fn run() {
    let file_contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
    println!("part1: {}", day6_part1(&file_contents));
    println!("part2: {}", day6_part2(&file_contents));
}

fn day6_part1(file_contents: &str) -> usize {
  // despite the trim, an extra empty shows up at the beginning and end - thus the filter
  let window_size = 4;
  let chars: Vec<&str> = file_contents.trim().split("").filter(|x| &x[..] != "").collect();
  let windows = &chars[..].windows(window_size).map(|x| (x, are_all_values_unique(&x))).collect::<Vec<_>>();
  for i in 0..windows.len() {
    if windows[i].1 == true { return i + window_size; }
  }

  9999
}

fn day6_part2(file_contents: &str) -> usize {
  // despite the trim, an extra empty shows up at the beginning and end - thus the filter
  let window_size = 14;
  let chars: Vec<&str> = file_contents.trim().split("").filter(|x| &x[..] != "").collect();
  let windows = &chars[..].windows(window_size).map(|x| (x, are_all_values_unique(&x))).collect::<Vec<_>>();
  for i in 0..windows.len() {
    if windows[i].1 == true { return i + window_size; }
  }

  9999
}

/**
Couldn't figure out how to import the [set](https://docs.rs/sets/latest/sets/) library, so I'm implementing my own duplicate detection
via looping over the list once, and for each item, counting the number of times that item appears.
This is O(n^2)
*/
fn are_all_values_unique(values: &[&str]) -> bool {
  for i in 0..values.len() {
    let x = values[i];
    // lol, why does this work? what is the double ampersand doing here, doesn't make sense?
    let count = values.iter().filter(|y| &&x == y).collect::<Vec<_>>().len();
    // println!("value at {}: {}, count: {}", i, x, count);
    if count > 1 { return false; }
  }

  true
}

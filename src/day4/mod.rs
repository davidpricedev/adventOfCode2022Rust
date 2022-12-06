use std::fs;

const FILE_PATH: &str = "src/day4/input.txt";

pub fn run() {
    println!("part1: {}", part1());
    println!("part2: {}", part2());
}

fn part1() -> i32 {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
    let range_pairs: Vec<RangePair> = get_range_pairs(contents.as_str()).collect();
    range_pairs.iter().map(|x| if x.has_full_containment() { 1 } else { 0 }).sum::<i32>()
}

fn part2() -> i32 {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
    let range_pairs: Vec<RangePair> = get_range_pairs(contents.as_str()).collect();
    range_pairs.iter().map(|x| if x.has_overlap() { 1 } else { 0 }).sum::<i32>()
}

fn get_range_pairs(file_content: &str) -> impl Iterator<Item = RangePair> + '_ {
    let lines = file_content.trim().split("\n");
    lines.map(|x| range_pair_from_string(x))
}

#[derive(Debug, Clone, Copy)]
struct Range {
    min: i32,
    max: i32,
}

fn range_from_string(rangestr: &str) -> Range {
    let range_pair: Vec<i32> = rangestr.split("-").map(|x| x.parse::<i32>().unwrap()).collect();
    Range {
        min: range_pair[0],
        max: range_pair[1],
    }
}

impl Range {
  fn is_contained_in(&self, other: Range) -> bool {
    other.min <= self.min && self.max <= other.max
  }

  fn has_overlap_with(&self, other: Range) -> bool {
    other.min <= self.max && self.min <= other.max
  }
}

#[derive(Debug)]
struct RangePair {
    a: Range,
    b: Range,
}

fn range_pair_from_string(linestr: &str) -> RangePair {
    let pairstr: Vec<&str> = linestr.split(",").collect();
    RangePair {
        a: range_from_string(&pairstr[0]),
        b: range_from_string(&pairstr[1]),
    }
}

impl RangePair {
  fn has_full_containment(&self) -> bool {
    self.a.is_contained_in(self.b) || self.b.is_contained_in(self.a)
  }

  fn has_overlap(&self) -> bool {
    self.a.has_overlap_with(self.b) || self.b.has_overlap_with(self.a)
  }
}

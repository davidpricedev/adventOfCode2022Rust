pub fn run() {
    let file_contents = include_str!("./input.txt");
    println!("part1: {}", part1(file_contents));
    println!("part2: {}", part2(file_contents));
}

fn part1(contents: &str) -> i32 {
    get_range_pairs(contents)
        .map(|x| if x.has_full_containment() { 1 } else { 0 })
        .sum::<i32>()
}

fn part2(contents: &str) -> i32 {
    get_range_pairs(contents)
        .map(|x| if x.has_overlap() { 1 } else { 0 })
        .sum::<i32>()
}

fn get_range_pairs(file_content: &str) -> impl Iterator<Item = RangePair> + '_ {
    file_content
        .trim()
        .lines()
        .map(|x| RangePair::from_string(x))
}

#[derive(Debug, Clone, Copy)]
struct Range {
    min: i32,
    max: i32,
}

impl Range {
    fn from_string(rangestr: &str) -> Range {
        let mut range_pair = rangestr.split("-").map(|x| x.parse::<i32>().unwrap());
        Range {
            min: range_pair.next().unwrap(),
            max: range_pair.next().unwrap(),
        }
    }

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

impl RangePair {
    fn from_string(linestr: &str) -> RangePair {
        let mut pairstr = linestr.split(",");
        RangePair {
            a: Range::from_string(&pairstr.next().unwrap()),
            b: Range::from_string(&pairstr.next().unwrap()),
        }
    }

    fn has_full_containment(&self) -> bool {
        self.a.is_contained_in(self.b) || self.b.is_contained_in(self.a)
    }

    fn has_overlap(&self) -> bool {
        self.a.has_overlap_with(self.b) || self.b.has_overlap_with(self.a)
    }
}

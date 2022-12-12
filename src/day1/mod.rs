pub fn run() {
    let file_contents = include_str!("./input.txt");
    println!("part1: {}", part1(file_contents));
    println!("part2: {}", part2(file_contents));
}

fn part1(contents: &str) -> i32 {
    let sums = get_sums_by_elf(contents);
    sums.max().unwrap()
}

fn part2(contents: &str) -> i32 {
    let mut sums: Vec<i32> = get_sums_by_elf(contents).collect::<_>();
    sums.sort();
    sums.reverse();
    sums[0..3].iter().sum::<i32>()
}

fn get_sums_by_elf(file_content: &str) -> impl Iterator<Item = i32> + '_ {
    file_content.split("\n\n").map(|chunk| {
        chunk.trim()
            .lines()
            .map(|x| x.parse::<i32>().unwrap())
            .sum()
    })
}

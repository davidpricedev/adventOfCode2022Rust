pub fn run() {
    let file_contents = include_str!("./input.txt");
    println!("part1: {}", part1(&file_contents));
    println!("part2: {}", part2(&file_contents));
}

fn part1(file_contents: &str) -> i32 {
    file_contents
        .trim()
        .lines()
        .map(|x| Round::parse_p1(x).get_score())
        .sum::<i32>()
}

fn part2(file_contents: &str) -> i32 {
    file_contents
        .trim()
        .lines()
        .map(|x| Round::parse_p2(x).get_score())
        .sum::<i32>()
}

#[derive(Debug, Clone, Copy)]
enum RpsMove {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, Clone, Copy)]
enum WinLoseDraw {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

#[derive(Debug, Clone, Copy)]
struct Round {
    their_move: RpsMove,
    my_move: RpsMove,
}

impl Round {
    fn result(&self) -> WinLoseDraw {
        let int_result = (3 + (self.my_move as i32) - (self.their_move as i32)) % 3;
        match int_result {
            0 => WinLoseDraw::Draw,
            1 => WinLoseDraw::Win,
            2 => WinLoseDraw::Lose,
            _ => panic!("unexpected round result value"),
        }
    }

    fn get_score(&self) -> i32 {
        self.result() as i32 + self.my_move as i32
    }

    fn parse_their_move(move_str: &str) -> RpsMove {
        match move_str {
            "A" => RpsMove::Rock,
            "B" => RpsMove::Paper,
            "C" => RpsMove::Scissors,
            _ => panic!("unexpected first part"),
        }
    }

    fn parse_p1(line: &str) -> Round {
        let parts = line.trim().split(" ").collect::<Vec<&str>>();
        let their_move = Round::parse_their_move(parts[0]);
        let my_move = match parts[1] {
            "X" => RpsMove::Rock,
            "Y" => RpsMove::Paper,
            "Z" => RpsMove::Scissors,
            _ => panic!("unexpected second part"),
        };
        Round {
            their_move,
            my_move,
        }
    }

    fn parse_p2(line: &str) -> Round {
        let parts = line.trim().split(" ").collect::<Vec<&str>>();
        let their_move = Round::parse_their_move(parts[0]);
        let target_result = match parts[1] {
            "X" => WinLoseDraw::Lose,
            "Y" => WinLoseDraw::Draw,
            "Z" => WinLoseDraw::Win,
            _ => panic!("unexpected second part"),
        };
        Round {
            their_move,
            my_move: Round::get_my_move(their_move, target_result),
        }
    }

    fn get_my_move(their_move: RpsMove, target_result: WinLoseDraw) -> RpsMove {
        let wldi = match target_result {
            WinLoseDraw::Draw => 0,
            WinLoseDraw::Win => 1,
            WinLoseDraw::Lose => 2,
        };
        let my_move_int = (wldi + (their_move as i32)) % 3;
        match my_move_int {
            0 => RpsMove::Scissors,
            1 => RpsMove::Rock,
            2 => RpsMove::Paper,
            _ => panic!("bad choice for my move"),
        }
    }
}

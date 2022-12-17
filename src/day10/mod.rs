pub fn run() {
    let file_contents = include_str!("./input.txt");
    println!("part1: {}", part1(file_contents));
    //println!("part2: {}", part2(file_contents));
}

fn part1(contents: &str) -> i32 {
    let cycles = contents
        .trim()
        .lines()
        .map(parse_instruction)
        .flat_map(expand_cycles)
        .fold(vec![CompState::new()], |mut acc, x| {
            let prev_state = &acc[acc.len() - 1];
            acc.push(prev_state.next_state(x));
            acc
        });

    [20, 60, 100, 140, 180, 220]
        .iter()
        .map(|x| cycles[((*x) as usize) - 1].sig_strength())
        .sum()
}

fn parse_instruction(line: &str) -> Instruction {
    let mut tokens = line.trim().split(" ");
    let token1 = tokens.next().unwrap();
    match token1 {
        "noop" => Instruction::Noop,
        "addx" => Instruction::Addx(tokens.next().unwrap().parse::<i32>().unwrap()),
        _ => panic!("unknown instruction: {}", token1),
    }
}

fn expand_cycles(instr: Instruction) -> Vec<Instruction> {
    match instr {
        Instruction::Noop => vec![Instruction::Noop],
        Instruction::Addx(x) => vec![Instruction::Noop, Instruction::Addx(x)],
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    Addx(i32),
}

#[derive(Debug)]
struct CompState {
    clock: i32,
    reg_x: i32,
    instr: Instruction,
}

impl CompState {
    fn new() -> CompState {
        CompState {
            reg_x: 1,
            clock: 1,
            instr: Instruction::Noop,
        }
    }

    fn next_state(&self, instr: Instruction) -> CompState {
        match instr {
            Instruction::Noop => CompState {
                reg_x: self.reg_x,
                clock: 1 + self.clock,
                instr,
            },
            Instruction::Addx(x) => CompState {
                reg_x: self.reg_x + x,
                clock: 1 + self.clock,
                instr,
            },
        }
    }

    fn sig_strength(&self) -> i32 {
        self.clock * self.reg_x
    }
}

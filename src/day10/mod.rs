pub fn run() {
    let file_contents = include_str!("./input.txt");
    println!("part1: {}", part1(file_contents));
    part2(file_contents);
}

fn part1(contents: &str) -> i32 {
    let cycles = parse_and_simulate(contents);
    let samples = [20, 60, 100, 140, 180, 220].iter().map(|x| (*x) as usize);
    samples.map(|x| cycles[x - 1].sig_strength()).sum()
}

fn part2(contents: &str) {
    let simulation = parse_and_simulate(contents);
    print_crt_lines(simulation);
}

fn parse_and_simulate(contents: &str) -> Vec<CompState> {
    contents
        .trim()
        .lines()
        .flat_map(parse_instruction)
        .fold(vec![], |mut acc, x| {
            if acc.len() == 0 {
                let prev_state = CompState::new();
                acc.push(prev_state.next_state(x));
                acc
            } else {
                let prev_state = &acc[acc.len() - 1];
                acc.push(prev_state.next_state(x));
                acc
            }
        })
}

fn parse_instruction(line: &str) -> Vec<Instruction> {
    let mut tokens = line.trim().split(" ");
    let token1 = tokens.next().unwrap();
    match token1 {
        "noop" => vec![Instruction::Noop],
        "addx" => vec![
            Instruction::Busy,
            Instruction::Addx(tokens.next().unwrap().parse::<i32>().unwrap()),
        ],
        _ => panic!("unknown instruction: {}", token1),
    }
}

fn print_crt_lines(simulation: Vec<CompState>) {
    let screen_lines = simulation.windows(40).step_by(40);
    screen_lines.for_each(|line| {
        let line_txt = line.iter().fold(String::from(""), |mut acc, x| {
            acc.push_str(&x.pixel[..]);
            acc
        });
        println!("{line_txt}");
    });
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    Busy,
    Addx(i32),
}

#[derive(Debug)]
struct CompState {
    clock: i32,
    reg_x: i32,
    pixel: String,
    prev_x: i32,
}

impl CompState {
    fn new() -> CompState {
        CompState {
            reg_x: 1,
            clock: 0,
            pixel: String::from("#"),
            prev_x: 1,
        }
    }

    fn next_state(&self, instr: Instruction) -> CompState {
        match instr {
            Instruction::Noop => self.next_noop(),
            Instruction::Busy => self.next_noop(),
            Instruction::Addx(x) => self.next_addx(x),
        }
    }

    fn next_noop(&self) -> CompState {
        let new_clock = self.clock + 1;
        CompState {
            reg_x: self.reg_x,
            clock: new_clock,
            pixel: CompState::get_pixel_value(new_clock, self.reg_x),
            prev_x: self.reg_x,
        }
    }

    fn next_addx(&self, x: i32) -> CompState {
        let new_clock = self.clock + 1;
        let new_reg_x = self.reg_x + x;
        CompState {
            reg_x: new_reg_x,
            clock: new_clock,
            pixel: CompState::get_pixel_value(new_clock, self.reg_x),
            prev_x: self.reg_x,
        }
    }

    fn sig_strength(&self) -> i32 {
        self.clock * self.prev_x
    }

    fn get_pixel_value(clock: i32, reg_x: i32) -> String {
        // not clear in the instructions, but each line must start the sprite at
        // position zero within that line
        let pos = (clock - 1) % 40;
        let sprite_range = (reg_x - 1, reg_x + 1);
        if sprite_range.0 <= pos && pos <= sprite_range.1 {
            String::from("#")
        } else {
            String::from(".")
        }
    }
}

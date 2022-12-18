use std::collections::HashMap;
#[macro_use]
use nom::{
    branch::alt, bytes::complete::tag, combinator::value, character::complete,
    multi::separated_list1, IResult, Parser,
};

pub fn run() {
    let file_contents = include_str!("./input.txt");
    run_tests();
    println!("part1: {}", part1(file_contents));
}

fn part1(contents: &str) -> i32 {
    let monkeys = parse_all(contents);
    dbg!(monkeys);

    42
}

fn parse_all(input: &str) -> HashMap<usize, Monkey> {
    let (_, monkey_vec) = separated_list1(tag("\n\n"), Monkey::parse)(input).unwrap();
    monkey_vec
        .iter()
        .map(|x| (x.id, x.clone()))
        .collect::<HashMap<usize, Monkey>>()
}

// fn run_round(mut monkeys: HashMap<usize, Monkey>) -> HashMap<usize, Monkey> {
//     for i in 0..monkeys.len() {
//         monkeys[&i].log_item_count();
//         let moves = monkeys[&i]
//             .items
//             .iter()
//             .map(|x| monkeys[&i].dest_val_pair(*x));
//     }

//     monkeys
// }

// fn log_all_item_counts(mut monkeys: HashMap<usize, Monkey>) -> HashMap<usize, Monkey> {
//     monkeys.iter().for_each(|(i, m)| m.log_item_count());
//     monkeys
//     for (i, m) in monkeys {
//         i
//     }
// }

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Operator {
    Plus,
    Mult,
}

impl Operator {
    fn parse(input: &str) -> IResult<&str, Operator> {
        alt((
            value(Operator::Plus, tag("+")),
            value(Operator::Mult, tag("*")),
        ))
        .parse(input)
    }

    fn apply(&self, operand1: i32, operand2: i32) -> i32 {
        match self {
            Operator::Plus => operand1 + operand2,
            Operator::Mult => operand1 * operand2,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Operand {
    Int(i32),
    Old,
}

impl Operand {
    fn parse(input: &str) -> IResult<&str, Operand> {
        alt((
            value(Operand::Old, tag("old")),
            complete::i32.map(|x| Operand::Int(x)),
        ))
        .parse(input)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Monkey {
    id: usize,
    items: Vec<i32>,
    operator: Operator,
    operand1: Operand,
    operand2: Operand,
    test_div: i32,
    true_dest: usize,
    false_dest: usize,
    item_count: i32,
}

impl Monkey {
    fn parse(input: &str) -> IResult<&str, Monkey> {
        let (input, _) = tag("Monkey ")(input)?;
        let (input, id) = complete::u32(input)?;
        let (input, _) = tag(":\n  Starting items: ")(input)?;
        let (input, items) = separated_list1(tag(", "), complete::i32)(input)?;
        let (input, _) = tag("\n  Operation: new = ")(input)?;
        let (input, operand1) = Operand::parse(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, operator) = Operator::parse(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, operand2) = Operand::parse(input)?;
        let (input, _) = tag("\n  Test: divisible by ")(input)?;
        let (input, test_div) = complete::i32(input)?;
        let (input, _) = tag("\n    If true: throw to monkey ")(input)?;
        let (input, true_dest_u) = complete::u32(input)?;
        let (input, _) = tag("\n    If false: throw to monkey ")(input)?;
        let (input, false_dest_u) = complete::u32(input)?;
        Ok((
            input,
            Monkey {
                id: id as usize,
                items,
                operator,
                operand1,
                operand2,
                test_div,
                true_dest: true_dest_u as usize,
                false_dest: false_dest_u as usize,
                item_count: 0,
            },
        ))
    }

    fn next_worry_level(&self, old_worry_level: i32) -> i32 {
        let operand1 = match self.operand1 {
            Operand::Old => old_worry_level,
            Operand::Int(x) => x,
        };
        let operand2 = match self.operand1 {
            Operand::Old => old_worry_level,
            Operand::Int(x) => x,
        };
        self.operator.apply(operand1, operand2)
    }

    fn test(&self, worry_level: i32) -> bool {
        ((self.next_worry_level(worry_level) / 3) % self.test_div) == 0
    }

    fn get_next_dest(&self, worry_level: i32) -> usize {
        if self.test(worry_level) {
            self.true_dest
        } else {
            self.false_dest
        }
    }

    fn dest_val_pair(&self, old_worry_level: i32) -> (usize, i32) {
        (
            self.get_next_dest(old_worry_level),
            self.next_worry_level(old_worry_level),
        )
    }

    fn log_item_count(&mut self) {
        self.item_count += self.items.len() as i32;
    }

    fn add_item(&mut self, worry_level: i32) {
        self.items.push(worry_level);
    }
}

fn run_tests() {
    let one_monkey: &str = "Monkey 4:
  Starting items: 68
  Operation: new = old * 5
  Test: divisible by 13
    If true: throw to monkey 6
    If false: throw to monkey 5
";
    assert_eq!(Operator::Plus, Operator::parse("+").unwrap().1);
    assert_eq!(Operator::Mult, Operator::parse("*").unwrap().1);
    assert_eq!(Operand::Old, Operand::parse("old").unwrap().1);
    assert_eq!(Operand::Int(42), Operand::parse("42").unwrap().1);
    assert_eq!(
        Monkey::parse(one_monkey).unwrap().1,
        Monkey {
            id: 4,
            items: vec![68],
            operator: Operator::Mult,
            operand1: Operand::Old,
            operand2: Operand::Int(5),
            test_div: 13,
            true_dest: 6,
            false_dest: 5,
            item_count: 0,
        }
    );
}

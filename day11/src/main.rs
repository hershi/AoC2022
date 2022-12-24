use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug, Clone)]
struct ItemTest {
    divisor: usize,
    true_action: usize,
    false_action: usize,
}

#[derive(Debug, Clone)]
enum Operation {
    Add(usize),
    Mult(usize),
    Square,
}


impl Operation {
    fn from_line(l: &str) -> Operation {
        lazy_static! {
            static ref ADD_RE: Regex = Regex::new(r"old \+ (\d+)").unwrap();
            static ref MULT_RE: Regex = Regex::new(r"old \* (\d+)").unwrap();
            static ref SQUARE_RE: Regex = Regex::new(r"old \* old").unwrap();
        }

        if let Some(c) = ADD_RE.captures(&l) {
            Operation::Add(c.get(1).unwrap().as_str().parse::<usize>().unwrap())
        } else if let Some(c) = MULT_RE.captures(&l) {
            Operation::Mult(c.get(1).unwrap().as_str().parse::<usize>().unwrap())
        } else if SQUARE_RE.is_match(&l) {
            Operation::Square
        } else {
            panic!("Unknown operation {:?}", l);
        }
    }

    fn eval(&self, i: usize) -> usize {
        match self {
            Operation::Add(x) => i + x,
            Operation::Mult(x) => i * x,
            Operation::Square => i * i,
        }
    }
}


#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    op: Operation,
    test: ItemTest,
    inspected: usize,
}

impl Monkey {
    // Return a vector of (worry_level, target_monkey)
    fn play_turn(&mut self) -> Vec<(usize, usize)> {
        self.inspected += self.items.len();
        self.items.drain(0..)
            .map(|i| self.op.eval(i) / 3)
            .map(|i| if i % self.test.divisor == 0 { (i, self.test.true_action)} else {(i, self.test.false_action)})
            .collect()
    }

    fn play_turn2(&mut self, reducer: usize) -> Vec<(usize, usize)> {
        self.inspected += self.items.len();
        self.items.drain(0..)
            .map(|i| self.op.eval(i) % reducer)
            .map(|i| if i % self.test.divisor == 0 { (i, self.test.true_action)} else {(i, self.test.false_action)})
            .collect()
    }
}

type Input = Vec<Monkey>;

fn parse_items(l: &str) -> Vec<usize> {
    l.split(":").nth(1).unwrap()
        .split(",")
        .map(|n| n.trim().parse::<usize>().unwrap())
        .collect()
}

fn parse_num(l: &str) -> usize {
    lazy_static! {
        static ref NUM_RE: Regex = Regex::new(r"(\d+)").unwrap();
    }

    NUM_RE.captures(&l).unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap()
}

fn read_input() -> Input {
    let input_file = File::open("src/input.txt").unwrap();
    let lines = BufReader::new(input_file)
        .lines()
        .flat_map(|l| l.ok())
        .collect::<Vec<_>>();

    let mut res = vec![];
    let mut i = 0;
    loop {
        if i >= lines.len() {
            break;
        }

        if lines[i].trim().is_empty() {
            i += 1;
            continue;
        }

        res.push(Monkey{
            items: parse_items(&lines[i+1]),
            op: Operation::from_line(&lines[i+2]),
            test: ItemTest {
                divisor: parse_num(&lines[i+3]),
                true_action: parse_num(&lines[i+4]),
                false_action: parse_num(&lines[i+5]),
            },
            inspected: 0,
        });
        i += 6;
    }

    res
}

fn part_1(input: &Input) {
    let mut monkeys = input.clone();
    for _ in 0..20 {
        for m in 0..monkeys.len() {
            for (i, to) in monkeys[m].play_turn() {
                monkeys[to].items.push(i);
            }
        }
    }

    monkeys.sort_by_key(|m| m.inspected);
    let monkey_business = monkeys.iter().rev().take(2)
        .map(|m| m.inspected)
        .fold(1, |acc, x| acc * x);
    println!("{:?}", monkey_business);
}


fn part_2(input: &Input) {
    let mut monkeys = input.clone();
    let reducer = monkeys.iter()
        .map(|m| m.test.divisor)
        .fold(1, |acc, x| acc * x);

    for _ in 0..10000 {
        for m in 0..monkeys.len() {
            for (i, to) in monkeys[m].play_turn2(reducer) {
                monkeys[to].items.push(i);
            }
        }
    }

    monkeys.sort_by_key(|m| m.inspected);
    let monkey_business = monkeys.iter().rev().take(2)
        .map(|m| m.inspected)
        .fold(1, |acc, x| acc * x);
    println!("{:?}", monkey_business);
}


fn main() {
    let input = read_input();
    part_1(&input);
    part_2(&input);
}

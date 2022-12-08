use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

#[derive(Debug)]
struct Round {
    me: u8,
    them: u8,
}

impl Round {
    fn score(&self) -> usize {
        let me = self.me as usize + 1;
        if self.me == (self.them + 1)%3 {
            return me + 6;
        }

        if self.them == self.me {
            return 3 + me;
        }

        me
    }
}

fn convert(s: &str) -> u8 {
    match s {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        "X" => 0,
        "Y" => 1,
        "Z" => 2,
        _ => panic!("Invalid input {}", s),
    }
}

fn convert_me(s: &str, them: u8) -> u8 {
    match s {
        "X" => (them + 2) % 3,
        "Y" => them,
        "Z" => (them + 1) %3,
        _ => panic!("Invalid input {}", s),
    }
}

fn convert_them(s: &str) -> u8 {
    match s {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        _ => panic!("Invalid input {}", s),
    }
}

fn read_input_1() -> Vec<Round> {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    reader.lines()
        .map(|x|x.unwrap())
        .map(|l| l.split(' ').take(2).map(|x|x.to_string()).collect::<Vec<_>>())
        .map(|v| {
            let them = convert(&v[0]);
            let me = convert(&v[1]);
            Round{me,them}
        })
        .collect()
}

fn read_input_2() -> Vec<Round> {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    reader.lines()
        .map(|x|x.unwrap())
        .map(|l| l.split(' ').take(2).map(|x|x.to_string()).collect::<Vec<_>>())
        .map(|v| {
            let them = convert_them(&v[0]);
            let me = convert_me(&v[1], them);
            Round{me,them}
        })
        .collect()
}

fn main() {
    let input = read_input_1();

    let score = input.iter()
        .map(|x|x.score())
        .sum::<usize>();
    println!("{:?}", score);

    let input = read_input_2();

    let score = input.iter()
        .map(|x|x.score())
        .sum::<usize>();
    println!("{:?}", score);
}

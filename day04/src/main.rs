use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

#[derive(Debug, Copy, Clone)]
struct Range {
    beginning: usize,
    end: usize,
}

impl Range {
    fn new(beginning: usize, end: usize) -> Range {
        Range{beginning, end}
    }

    fn from_str(s: &str) -> Range {
        let v = s.split("-")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Range::new(v[0], v[1])
    }

    fn contains(&self, other: &Range) -> bool {
        other.beginning >= self.beginning &&
            other.end <= self.end
    }

    fn overlap(&self, other: &Range) -> bool {
        (self.end >= other.beginning && self.end <= other.end) ||
            (other.end >= self.beginning && other.end <= self.end)
    }
}

fn read_input() -> Vec<(Range, Range)> {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    reader.lines()
        .map(|x|x.unwrap())
        .map(|l| l.split(",").map(|x| Range::from_str(x)).collect::<Vec<_>>())
        .map(|v| (v[0], v[1]))
        .collect()
}

fn part_1(input: &Vec<(Range,Range)>) -> usize {
    input.iter()
        .filter(|p| p.0.contains(&p.1) || p.1.contains(&p.0))
        .count()
}

fn part_2(input: &Vec<(Range,Range)>) -> usize {
    input.iter()
        .filter(|p| p.0.overlap(&p.1))
        .count()
}


fn main() {
    let input = read_input();
    let s1 = part_1(&input);
    println!("Result 1: {}", s1);

    let s2 = part_2(&input);
    println!("Result 2: {}", s2);
}

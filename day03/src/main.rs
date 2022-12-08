use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashSet;

#[derive(Debug)]
struct Rucksack {
    h1: Vec<usize>,
    h2: Vec<usize>,
}

impl Rucksack {
    fn new(r: (&[usize], &[usize])) -> Rucksack {
        Rucksack { h1: r.0.to_vec(), h2: r.1.to_vec() }
    }
}

fn convert(c: &char) -> usize {
    if c.is_uppercase() {
        *c as usize - 'A' as usize + 27
    } else {
        *c as usize - 'a' as usize + 1
    }
}


fn read_input() -> Vec<Rucksack> {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    reader.lines()
        .map(|x|x.unwrap())
        .map(|l| l.chars().map(|c| convert(&c)).collect::<Vec<_>>())
        .map(|l| Rucksack::new(l.split_at(l.len() / 2)))
        .collect()
}

fn part_1(input: &Vec<Rucksack>) -> usize {
    input.iter()
        .map(|rs|
             rs.h1.iter().collect::<HashSet<_>>().intersection(
                 &rs.h2.iter().collect::<HashSet<_>>())
             .cloned().nth(0).unwrap())
        .fold(0, |acc, p| acc + p)
}

fn extract_group_member(input: &Vec<Rucksack>, i: usize) -> HashSet<usize> {
    let member = &input[i];
    member.h1.iter().chain(member.h2.iter()).cloned().collect::<HashSet<_>>()
}

fn part_2(input: &Vec<Rucksack>) -> usize {
    let mut v = Vec::new();
    for i in 0..input.len()/3 {
        let e1 = extract_group_member(input, i*3);
        let e2 = extract_group_member(input, i*3 + 1);
        let e3 = extract_group_member(input, i*3 + 2);

        v.push(e1.intersection(&e2).cloned().collect::<HashSet<_>>().intersection(&e3).cloned().nth(0).unwrap());
    }

    v.iter().sum()
}


fn main() {
    let input = read_input();
    let s1 = part_1(&input);
    println!("Sum of priorities: {}", s1);

    let s2 = part_2(&input);
    println!("Sum of priorities: {}", s2);
}

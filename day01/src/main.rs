use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn read_input() -> Vec<usize> {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    reader.lines()
        .map(|x|x.unwrap())
        .fold(vec![vec![]], |mut acc, l| {
            if l.trim().is_empty() {
                acc.push(vec![]);
            } else {
                acc.last_mut().unwrap().push(l.parse::<usize>().unwrap());
            }
            acc
        })
        .iter()
        .map(|e| e.iter().sum())
        .collect()
}

fn part_1(elves: &Vec<usize>) {
    println!("Max is {:?}", elves.iter().max().unwrap());
}

fn part_2(elves: &Vec<usize>) {
    let mut elves = elves.clone();
    elves.sort_unstable();
    println!("Max is {:?}", elves.iter().rev().take(3).sum::<usize>());
}

fn main() {
    let input = read_input();
    part_1(&input);
    part_2(&input);
    //println!("{:?}", input);
}

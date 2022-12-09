use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Move {
    quantity: usize,
    from: usize,
    to: usize,
}

type Input = (HashMap<usize, Vec<char>>, Vec<Move>);

fn convert_line(l: &str) -> Vec<(usize, char)> {
    let l = l.as_bytes();
    (1..l.len()).step_by(4)
        .filter_map(|i| if l[i] != ' ' as u8 { Some((i/4 + 1, l[i] as char)) } else { None } )
        .collect()
}

fn read_input() -> Input {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    let stacks = reader.lines()
        .filter_map(|l| l.ok())
        .take_while(|l| !l.trim().is_empty())
        .collect::<Vec<_>>()
        .iter()
        .rev()
        .skip(1)
        .flat_map(|l| convert_line(l))
        .fold(HashMap::new(), |mut stacks, x| { stacks.entry(x.0).or_insert(vec![]).push(x.1); stacks});

    let MOVE_REGEX = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    let moves = reader.lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| {
            let captures = MOVE_REGEX.captures(&l);
            match captures {
                Some(c) => Some(vec![
                                c[1].to_string(),
                                c[2].to_string(),
                                c[3].to_string()]),
                _ => None
            }
        })
        .map(|c| Move{
            quantity: c[0].parse::<usize>().unwrap(),
            from: c[1].parse::<usize>().unwrap(),
            to: c[2].parse::<usize>().unwrap()
        })
        .collect();

    (stacks, moves)
}

fn part_1(input: &Input) {
    let mut stacks = input.0.clone();
    for m in input.1.iter() {
        for i in 0..m.quantity {
            let item = stacks.get_mut(&m.from).unwrap().pop().unwrap();
            stacks.get_mut(&m.to).unwrap().push(item);
        }
    }

    let res = (1..stacks.len()+1)
        .filter_map(|i| stacks[&i].last())
        .collect::<String>();

    println!("Tops {:?}", res);
}

fn part_2(input: &Input) {
    let mut stacks = input.0.clone();
    for m in input.1.iter() {
        let mut interim = vec![];
        for i in 0..m.quantity {
            interim.push(stacks.get_mut(&m.from).unwrap().pop().unwrap());
        }

        while !interim.is_empty() {
            stacks.get_mut(&m.to).unwrap().push(interim.pop().unwrap());
        }
    }

    let res = (1..stacks.len()+1)
        .filter_map(|i| stacks[&i].last())
        .collect::<String>();

    println!("Tops {:?}", res);
}


fn main() {
    let input = read_input();
    let s1 = part_1(&input);
    let s2 = part_2(&input);
}

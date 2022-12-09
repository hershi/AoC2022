use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum DirEntry {
    Dir(String),
    File(usize, String),
}

type DirListing = Vec<DirEntry>;

#[derive(Debug, Clone)]
enum Command {
    List(DirListing),
    ChangeDirectory(String),
}

type Input = Vec<Command>;

fn read_input() -> Input {
    //let input_file = File::open("src/input.txt").unwrap();
    //let reader = BufReader::new(input_file);
    //let stacks = reader.lines()

    Vec::new()
}

fn part_1(input: &Input) {
    println!("Input {:?}", input);
}

fn part_2(input: &Input) {
}


fn main() {
    let input = read_input();
    part_1(&input);
    part_2(&input);
}

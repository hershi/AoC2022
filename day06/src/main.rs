use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashSet;

type Input = Vec<char>;

fn read_input() -> Input {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    reader.lines()
        .filter_map(|l| l.ok())
        .flat_map(|l| l.into_bytes())
        .map(|b| b as char)
        .collect()
}

fn part_1(input: &Input) {
    let first_packet_marker =
        input.as_slice()
         .windows(4)
         .enumerate()
         .find(|x| HashSet::<&char>::from_iter(x.1.iter()).len() == 4)
         .unwrap();

    println!("First location {:?}", first_packet_marker.0 + 4);
}

fn part_2(input: &Input) {
    let first_packet_marker =
        input.as_slice()
         .windows(14)
         .enumerate()
         .find(|x| HashSet::<&char>::from_iter(x.1.iter()).len() == 14)
         .unwrap();

    println!("First location {:?}", first_packet_marker.0 + 14);
}


fn main() {
    let input = read_input();
    let s1 = part_1(&input);
    let s2 = part_2(&input);
}

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::str;
use std::cmp::Ordering;
use std::iter::once;

#[derive(Debug, Clone)]
enum Packet {
    Value(u8),
    List(Vec<Packet>),
}

const OPEN :u8 = '[' as u8;
const CLOSE :u8 = ']' as u8;
const COMMA :u8 = ',' as u8;

impl Packet {
    fn from_line(l: &[u8], p: &mut usize) -> Packet {
        let mut contents = Vec::new();

        while *p < l.len() {
            if l[*p] == COMMA {
                *p += 1;
                continue;
            } else if l[*p] == OPEN {
                *p += 1;
                contents.push(Packet::from_line(l, p));
                continue;
            } else if l[*p] == CLOSE {
                *p += 1;
                return Packet::List(contents);
            }

            assert!(l[*p].is_ascii_digit());

            // Find the end of the number
            let s = *p;
            while l[*p].is_ascii_digit() {
                *p += 1;
            }

            contents.push(Packet::Value(str::from_utf8(&l[s..*p]).unwrap().parse::<u8>().unwrap()));

            // Skip the comma
            assert!(l[*p] == COMMA || l[*p] == CLOSE);
        }

        Packet::List(contents)
    }

    fn as_list(&self) -> Packet {
        Packet::List(vec![self.clone()])
    }
}

impl Eq for Packet {}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}


impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Packet::Value(x) => {
                match other {
                    Packet::Value(y) => x.cmp(y),
                    Packet::List(_) => self.as_list().cmp(other),
                }
            },
            Packet::List(sl) => {
                match other {
                    Packet::Value(_) => self.cmp(&other.as_list()),
                    Packet::List(ol) => {
                        for i in 0..usize::min(sl.len(), ol.len()) {
                            if sl[i] < ol[i] {
                                return Ordering::Less;
                            }

                            if ol[i] < sl[i] {
                                return Ordering::Greater;
                            }

                            // Equal items, continue
                        }

                        if sl.len() < ol.len() {
                            Ordering::Less
                        } else if sl.len() > ol.len() {
                            Ordering::Greater
                        } else {
                            Ordering::Equal
                        }
                    },
                }
            }
        }
    }
}

type Input = Vec<(Packet,Packet)>;

fn read_input() -> Input {
    let input_file = File::open("src/input.txt").unwrap();
    let lines = BufReader::new(input_file)
        .lines()
        .flat_map(|l| l.ok())
        .collect::<Vec<_>>();

    lines.iter().zip(lines.iter().skip(1))
        .step_by(3)
        .map(|(l,r)| (Packet::from_line(l.as_bytes(), &mut 1), Packet::from_line(r.as_bytes(), &mut 1)))
        .collect()
}

fn part_1(input: &Input) {
    let r1 = input.iter().enumerate()
        .filter(|(_,p)| p.0 < p.1)
        .map(|(i,_)| i+1)
        .sum::<usize>();

    println!("Result 1: {}", r1);
}


fn part_2(input: Input) {
    let mut input = input.into_iter()
        .flat_map(|x| once(x.0).chain(once(x.1)))
        .collect::<Vec<_>>();

    let del1 = Packet::Value(2).as_list().as_list();
    let del2 = Packet::Value(6).as_list().as_list();
    input.push(del1.clone());
    input.push(del2.clone());
    input.sort();

    let r2 = input.iter().enumerate()
        .filter(|(_, p)| **p == del1 || **p == del2)
        .map(|(i,_)| i + 1)
        .fold(1, |acc, x| acc * x);
    println!("Result 2: {}", r2);
}


fn main() {
    let input = read_input();
    part_1(&input);
    part_2(input);
}

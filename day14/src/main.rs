use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashSet;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y:isize) -> Point {
        Point{x,y}
    }

    fn from_str(s: &str) -> Point {
        let parts = s.split(",")
            .map(|n| n.parse::<isize>().unwrap())
            .collect::<Vec<_>>();

        Point {x: parts[0], y: parts[1] }
    }
}

type Input = HashSet<Point>;

fn line(p1: &Point, p2: &Point) -> HashSet<Point> {
    if p1.x == p2.x {
        let l = isize::min(p1.y, p2.y);
        let u = isize::max(p1.y, p2.y);
        (l..=u)
            .map(|y| Point::new(p1.x, y))
            .collect()
    } else {
        assert!(p1.y == p2.y);
        let l = isize::min(p1.x, p2.x);
        let u = isize::max(p1.x, p2.x);
        (l..=u)
            .map(|x| Point::new(x, p1.y))
            .collect()
    }
}

fn rock_from_str(s: &str) -> HashSet<Point> {
    let points = s.split(" -> ")
        .map(|p| Point::from_str(p))
        .collect::<Vec<_>>();

    points.iter().zip(points.iter().skip(1))
        .map(|(p1, p2)| line(p1, p2))
        .fold(HashSet::new(), |acc, hs| acc.union(&hs).cloned().collect())
}

fn read_input() -> Input {
    lazy_static! {
        static ref PARSER_RE: Regex = Regex::new(r"\b(\d+),(\d+)\s*(->)?\s*").unwrap();
    }

    let input_file = File::open("src/input.txt").unwrap();
    BufReader::new(input_file)
        .lines()
        .flat_map(|l| l.ok())
        .flat_map(|l| rock_from_str(&l))
        .collect()
}

fn resting_place(blocked: &HashSet<Point>, lowest_rock: isize) -> Option<Point> {
    let mut s = Point::new(500,0);

    loop {
        assert!(s.y < lowest_rock);

        if s.y == lowest_rock-1 {
            return Some(s);
        }

        let new_s = vec![
            Point::new(s.x, s.y+1),
            Point::new(s.x-1, s.y+1),
            Point::new(s.x+1, s.y+1),].into_iter()
                .filter(|p| !blocked.contains(p))
                .nth(0);

        match new_s {
            Some(p)=> s = p,
            None=> return Some(s),
        }
    }
}

fn part_1(input: &Input) {
    let mut blocked = input.clone();

    let lowest_rock = input.iter()
        .map(|p| p.y)
        .max()
        .unwrap() + 2;

    println!("Lowest rock is at y = {}", lowest_rock);

    loop {
        if blocked.contains(&Point::new(500,0)) {
            break;
        }

        match resting_place(&blocked, lowest_rock) {
            Some(p) => {
                println!("Resting place {:?}", p);
                blocked.insert(p);
            },
            None => break,
        }
    }

    println!("Part 1: {}", blocked.len() - input.len());
}


fn part_2(input: &Input) {
}


fn main() {
    let input = read_input();
    part_1(&input);
    part_2(&input);
}

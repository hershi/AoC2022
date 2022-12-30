use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashSet;
use regex::Regex;
use lazy_static::lazy_static;
use itertools::iproduct;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y:isize) -> Point {
        Point{x,y}
    }

    fn dist(&self, other: &Point) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug, Clone)]
struct Sensor {
    loc: Point,
    beacon: Point,
}

impl Sensor {
    fn from_line(l: &str) -> Sensor {
        lazy_static! {
            static ref PARSER_RE: Regex =
                Regex::new(r".*x=(-?\d+).*y=(-?\d+).*x=(-?\d+).*y=(-?\d+).*").unwrap();
        }

        let cap = PARSER_RE.captures(l).unwrap()
            .iter()
            .flat_map(|c| c)
            .flat_map(|c| c.as_str().parse::<isize>())
            .collect::<Vec<_>>();

        Sensor{
            loc: Point::new(cap[0], cap[1]),
            beacon: Point::new(cap[2], cap[3]),
        }
    }

    fn no_beacon_at_span(&self, line: isize) -> Option<(isize, isize)> {
        let dist = self.loc.dist(&self.beacon);
        let line_dist = (line - self.loc.y).abs();
        let rem = dist - line_dist;

        if rem >= 0 {
            Some((self.loc.x-rem, self.loc.x+rem))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct Spans {
    spans: Vec<(isize, isize)>,
    min: isize,
    max: isize,
}

impl Spans {
    fn new(min: isize, max: isize) -> Spans {
        Spans { spans: Vec::new(), min, max }
    }

    fn add(&mut self, span: (isize, isize)) {
        let span = (span.0.clamp(self.min, self.max), span.1.clamp(self.min, self.max));
        self.spans.push(span);
    }

    fn compact(&mut self) {
        // Sort spans by start point, descending - that way we can iterate from
        // lowest to highest by popping from the end (to avoid costly vector
        // operations)
        self.spans.sort_by_key(|span| -span.0);

        let mut compact = Vec::new();
        let mut prev = self.spans.pop().unwrap();
        while !self.spans.is_empty() {
            let current = self.spans.pop().unwrap();

            // Spans overlap. Merge them and continue
            // continue
            if current.0 <= prev.1 + 1 {
                //print!("Merging {:?} and {:?} --> ", prev, current);
                prev.1 = isize::max(prev.1, current.1);
                continue;
            }

            // Spans don't overlay - push the previous one and continue
            // with the current one
            compact.push(prev);
            prev = current;
        }

        compact.push(prev);
        self.spans = compact;
    }
}

type Input = Vec<Sensor>;

fn read_input() -> Input {
    let input_file = File::open("src/input.txt").unwrap();
    BufReader::new(input_file)
        .lines()
        .flat_map(|l| l.ok())
        .map(|l| Sensor::from_line(&l))
        .collect()
}

fn part_1(input: &Input) {
    const INTERESTING_LINE :isize = 2000000;
    let mut spans = input.iter()
        .flat_map(|s| s.no_beacon_at_span(INTERESTING_LINE))
        .fold(Spans::new(isize::MIN, isize::MAX), |mut acc, span| { acc.add(span); acc });

    spans.compact();
    let sum = spans.spans.iter()
        .map(|s| (s.1-s.0+1).abs())
        .sum::<isize>();
    let beacons_on_line = input.iter()
        .filter(|s| s.beacon.y == INTERESTING_LINE)
        .map(|s| s.beacon.clone())
        .collect::<HashSet<_>>()
        .len();

    println!("Count:{} ({} - {})", sum - beacons_on_line as isize,
        sum, beacons_on_line);
}


fn part_2(input: &Input) {
    const MAX : isize = 4000000;

    for line in 0..=MAX {
        let mut spans = input.iter()
            .flat_map(|s| s.no_beacon_at_span(line))
            .fold(Spans::new(0, MAX), |mut acc, span| { acc.add(span); acc });

        spans.compact();
        if spans.spans.len() > 1 {
            assert!(spans.spans.len() == 2);
            println!("Result: {} ({},{})",
                4000000 * (spans.spans[0].1 + 1) + line,
                spans.spans[0].1+1, line);
            return;
        }
    }

    println!("Not found!");
}

fn main() {
    let input = read_input();
    part_1(&input);
    part_2(&input);
}

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

const START: u8 = 0;
const END: u8 = 27;
const VISITED: u8 = u8::MAX;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y:usize) -> Point {
        Point{x,y}
    }

    fn add(&self, p: &(isize, isize)) -> Option<Point> {
        let new_x = self.x as isize + p.0;
        let new_y = self.y as isize + p.1;

        if new_x < 0 || new_y < 0 {
            None
        } else {
            Some(Point::new(new_x as usize, new_y as usize))
        }
    }
}


#[derive(Debug, Clone)]
struct Map {
    grid: Vec<(u8, Option<usize>)>,
    cols: usize,
    rows: usize,
    start: Point,
    end: Point,
}

impl Map {
    fn neighbors(&self, p: &Point) -> Vec<Point> {
        [(0,1), (0,-1), (1,0), (-1,0)]
            .iter()
            .filter_map(|m| p.add(m))
            .filter(|n| n.x < self.cols && n.y < self.rows)
            .filter(|n| self.get(n).0 + 1 >= self.get(p).0)
            .collect()

    }

    fn set_route(&mut self, p: &Point, shortest_route: usize) {
        let idx = p.x + p.y * self.cols;
        self.grid[p.x + p.y * self.cols].1 = Some(shortest_route);
    }

    fn get(&self, p: &Point) -> (u8, Option<usize>) {
        self.grid[p.x + p.y * self.cols]
    }

    fn find_shortest_route(&mut self, from: &Point, mut path: usize, start_val: u8) -> Option<usize> {
        let (current_val, current_path) = self.get(&from);
        let current_path = current_path.unwrap_or(usize::MAX);

        if path >= current_path {
            return None;
        }

        self.set_route(&from, path);
        path += 1;

        if current_val <= start_val {
            return Some(path);
        }

        self.neighbors(&from).into_iter()
            .flat_map(|n| self.find_shortest_route(&n, path, start_val))
            .min()
    }
}

type Input = Map;

fn parse_char(c: u8) -> u8 {
    match c {
        x if x == 'S' as u8 => START,
        x if x == 'E' as u8 => END,
        c => c - 'a' as u8 + 1
    }
}

fn read_input() -> Input {
    let input_file = File::open("src/input.txt").unwrap();
    let lines = BufReader::new(input_file)
        .lines()
        .flat_map(|l| l.ok())
        .collect::<Vec<_>>();

    let grid = lines.iter()
        .flat_map(|l| l.as_bytes().iter().map(|c| (parse_char(*c), None)))
        .collect::<Vec<_>>();

    let rows = lines.len();
    let cols = lines[0].len();
    let start = grid.iter().enumerate()
        .filter(|x| x.1.0 == START)
        .map(|x| Point::new(x.0 % cols, x.0 / cols))
        .nth(0)
        .unwrap();

    let end = grid.iter().enumerate()
        .filter(|x| x.1.0 == END)
        .map(|x| Point::new(x.0 % cols, x.0 / cols))
        .nth(0)
        .unwrap();

    Map { grid, cols, rows, start, end }
}

fn part_1(input: &Input) {
    let mut map : Input = input.clone();
    let route = map.find_shortest_route(&input.end, 0, START).unwrap();
    println!("Result: {}", route - 1);
}


fn part_2(input: &Input) {
    let mut map : Input = input.clone();
    let route = map.find_shortest_route(&input.end, 0, 1).unwrap();
    println!("Result: {}", route - 1);
}


fn main() {
    let input = read_input();
    part_1(&input);
    part_2(&input);
}

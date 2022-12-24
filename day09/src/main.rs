use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Action {
    vector: (isize, isize),
    count: isize,
}

impl Action {
    fn new(line: &str) -> Action {
        let parts = line.trim().split(" ").collect::<Vec<_>>();
        let count = parts[1].parse::<isize>().unwrap();
        match parts[0].as_bytes()[0] as char {
            'U'=> Action{ vector: (-1, 0), count },
            'D'=> Action{ vector: (1, 0), count },
            'L'=> Action{ vector: (0, -1), count },
            'R'=> Action{ vector: (0, 1), count },
            _=>panic!("Unknown direction {}", parts[0]),
        }
    }
}

#[derive(Debug, Clone)]
struct Rope {
    head: (isize, isize),
    tail: (isize, isize),
}

impl Rope {
    fn new() -> Rope {
        Rope { head: (0,0), tail:(0,0) }
    }

    fn process(&mut self, action: &Action) -> HashSet<(isize, isize)> {
        (0..action.count)
            .map(|_| self._process_single(action).0)
            .collect()
    }

    fn process2(&mut self, action: &Action) -> Vec<Action> {
        (0..action.count)
            .map(|_| self._process_single(action).1)
            .map(|v| Action { vector: v, count: 1 })
            .collect()
    }

    fn _get_dir(&mut self, action: &Action) -> (isize, isize) {
        self.head.0 += action.vector.0;
        self.head.1 += action.vector.1;

        let dist = (self.head.0 - self.tail.0, self.head.1 - self.tail.1);
        if dist.0 == 0 && dist.1.abs() > 1 {
            (0, dist.1.abs() / dist.1)
        } else if dist.1 == 0 && dist.0.abs() > 1 {
            (dist.0.abs() / dist.0, 0)
        } else if dist.0.abs() + dist.1.abs() > 2 {
            (dist.0.abs() / dist.0,  dist.1.abs() / dist.1)
        } else {
            (0,0)
        }
    }

    fn _process_single(&mut self, action: &Action) -> ((isize, isize), (isize,isize)) {
        let dir = self._get_dir(action);

        self.tail.0 += dir.0;
        self.tail.1 += dir.1;
        (self.tail, dir)
    }
}

type Input = Vec<Action>;

fn read_input() -> Input {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);

    reader.lines()
        .flat_map(|l| l.ok())
        .map(|l| Action::new(&l))
        .collect::<Vec<_>>()

}

fn part_1(input: &Input) {
    let mut rope = Rope::new();

    let mut visited = input.iter()
        .map(|a| rope.process(a))
        .fold(HashSet::new(), |s,e| s.union(&e).cloned().collect());

    visited.insert((0,0));
    println!("Part 1: Tail visited {} location", visited.len());
}


fn part_2(input: &Input) {
    let mut ropes = vec![Rope::new(); 9];
    let mut visited : HashSet<(isize, isize)> = HashSet::new();

    for action in input.iter() {
        let mut actions = ropes[0].process2(action);
        for i in 1..8 {
            actions = actions.iter()
                .flat_map(|a| ropes[i].process2(a))
                .collect();
        }

        visited = actions.iter()
            .map(|a| ropes[8].process(a))
            .fold(visited, |s,e| s.union(&e).cloned().collect());
    }

    visited.insert((0,0));
    println!("Part 2: Tail visited {} location", visited.len());
}


fn main() {
    let input = read_input();
    part_1(&input);
    part_2(&input);
}

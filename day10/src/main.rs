use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

#[derive(Debug, Clone)]
enum Instruction {
    Noop,
    AddX(isize),
}

impl Instruction {
    fn from_input_line(l: &str) -> Instruction {
        if l.trim() == "noop" {
            Instruction::Noop
        } else {
            let parts = l.trim().split(" ").take(2).collect::<Vec<_>>();
            Instruction::AddX(parts[1].parse::<isize>().unwrap())
        }
    }
}

struct CpuState {
    x: isize,
    time: usize,
    history: Vec<isize>,
}

impl CpuState {
    fn new() -> CpuState {
        CpuState {x: 1, time: 1, history: vec![]}
    }

    fn get_signal_strength(&self, time: usize) -> isize {
        if time > self.time {
            panic!("Asking about future time ({} > {})", time, self.time);
        }

        self.history[time - 1] * time as isize
    }

    fn _state(&self) -> String {
        format!("time,x,hist_len {},{},{}",
                self.time,
                self.x,
                self.history.len())
    }

    fn execute(&mut self, i: &Instruction) {
        match i {
            Instruction::Noop => {
                self.history.push(self.x);
                self.time += 1;
            },
            Instruction::AddX(x) => {
                self.history.push(self.x);
                self.history.push(self.x);
                self.time += 2;
                self.x += x;
            }
        }
    }
}

struct Display {
    pos: isize,
    pixels: Vec<char>,
}

impl Display {
    fn new() -> Display {
        Display { pos: 0, pixels: vec![] }
    }

    fn draw(&mut self, sprite_loc: isize) {
        if self.pos >= sprite_loc-1 && self.pos <= sprite_loc+1 {
            self.pixels.push('#');
        } else {
            self.pixels.push('.');
        }

        self.pos = (self.pos + 1) % 40;
    }

    fn print(&self) {
        const ROWS : usize = 6;
        const COLS : usize = 40;

        for row in 0..ROWS {
            for col in 0..COLS {
                print!("{}", self.pixels[row * COLS + col]);
            }
            println!("");
        }
    }
}

type Input = Vec<Instruction>;

fn read_input() -> Input {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);

    reader.lines()
        .flat_map(|l| l.ok())
        .map(|l| Instruction::from_input_line(&l))
        .collect()

}

fn part_1(input: &Input) {
    let mut cpu = CpuState::new();
    for i in input.iter() {
        cpu.execute(i);
    }

    let sum = [20,60,100,140,180,220].into_iter()
        .map(|c| cpu.get_signal_strength(c))
        .sum::<isize>();

    println!("Part 1: {}", sum);

    let mut display = Display::new();
    for t in 1..=240 {
        display.draw(cpu.history[t-1]);
    }
    display.print();
}


fn main() {
    let input = read_input();
    part_1(&input);
}

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

#[derive(Debug, Clone)]
struct Tree {
    height: usize,
    visible: bool,
}


impl Tree {
    fn new(height: usize) -> Tree {
        Tree { height, visible: false }
    }
}


#[derive(Debug, Clone)]
struct Grid {
    trees: Vec<Tree>,
    cols: usize,
    rows: usize,
}

impl Grid {
    fn new(trees: Vec<Tree>, cols: usize) -> Grid {
        let rows = trees.len() / cols;
        Grid { trees, cols, rows }
    }

    fn get(&self, x: usize, y: usize) -> &Tree {
        &self.trees[ x + y * self.cols ]
    }

    fn set_visible(&mut self, x:usize, y:usize) {
        self.trees[ x + y * self.cols ].visible = true;
    }
}

type Input = Grid;

fn read_input() -> Input {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);

    let lines = reader.lines()
        .flat_map(|l| l.ok())
        .collect::<Vec<_>>();

    let cols = lines[0].len();
    Grid::new(
        lines.iter()
            .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap()))
            .map(|t| Tree::new(t as usize))
            .collect(),
        cols)
}

fn foo(input: &Input) {
    let mut grid = input.clone();
    for c in 0..grid.cols {
        let mut max_height = -1;
        for r in 0..grid.rows {
            if grid.get(c,r).height as isize > max_height {
                max_height = grid.get(c,r).height as isize;
                grid.set_visible(c,r);
            }
        }

        max_height = -1;
        for r in (0..grid.rows).rev() {
            if grid.get(c,r).height as isize > max_height {
                max_height = grid.get(c,r).height as isize;
                grid.set_visible(c,r);
            }
        }
    }


    for r in 0..grid.rows {
        let mut max_height = -1;
        for c in 0..grid.cols {
            if grid.get(c,r).height as isize > max_height {
                max_height = grid.get(c,r).height as isize;
                grid.set_visible(c,r);
            }
        }

        max_height = -1;
        for c in (0..grid.cols).rev() {
            if grid.get(c,r).height as isize > max_height {
                max_height = grid.get(c,r).height as isize;
                grid.set_visible(c,r);
            }
        }
    }

    let c = grid.trees.iter().filter(|t| t.visible).count();
    println!("Part 1: {:?}",  c);

    for j in 0..grid.rows {
        for i in 0..grid.cols {
            if grid.get(i,j).visible {
                print!("{}", grid.get(i,j).height);
            } else {
                print!(" ");
            }
        }
        println!("");
    }

    let mut max_scenic_score = 0;
    for j in 0..grid.rows {
        for i in 0..grid.cols {
            let s = calc_scenic_score(&grid, i,j);
            if s > max_scenic_score {
                println!("Shifting {}-->{} ({},{})", max_scenic_score, s, i,j);
                max_scenic_score = s;
            }
        }
    }

    println!("Max scenic score: {}", max_scenic_score);
}

fn calc_scenic_score(grid: &Grid, x: usize, y:usize) -> usize {
    let h = grid.get(x,y).height;

    let top = if y == 0 { 0 } else {
        let mut j = y-1;
        let mut count = 0;
        loop {
            count += 1;
            if grid.get(x,j).height >= h {
                break;
            }

            if j == 0 {
                break;
            }
            j-=1;
        }

        count
    };

    let bottom = {
        let mut j = y+1;
        let mut count = 0;
        loop {
            if j >= grid.rows {
                break;
            }

            count += 1;
            if grid.get(x,j).height >= h {
                break;
            }

            j+=1;
        }

        count
    };

    let left = if x == 0 { 0 } else {
        let mut i = x-1;
        let mut count = 0;
        loop {
            count += 1;
            if grid.get(i,y).height >= h {
                break;
            }

            if i == 0 {
                break;
            }
            i-=1;
        }

        count
    };

    let right = {
        let mut i = x+1;
        let mut count = 0;
        loop {
            if i >= grid.cols {
                break;
            }

            count += 1;
            if grid.get(i,y).height >= h {
                break;
            }

            i+=1;
        }

        count
    };

    //println!("calc_scenic_score({},{}) {} * {} * {} * {} = {}",
        //x,y,
        //top,bottom,left,right,
        //top * bottom * left * right);

    top * bottom * left * right
}


fn main() {
    let input = read_input();
    foo(&input);
}

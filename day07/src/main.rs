use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use regex::Regex;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::{Cell,RefCell};
use lazy_static::lazy_static;
use log::*;

#[derive(Debug, Clone)]
struct FileDetails {
    name: String,
    size: usize,
}

#[derive(Debug, Clone)]
enum DirEntry {
    Dir(String),
    File(FileDetails),
}

#[derive(Debug, Clone)]
enum Command {
    List(Vec<DirEntry>),
    ChangeDirectory(String),
}


#[derive(Clone)]
struct Node {
    name: String,
    files: Vec<DirEntry>,
    dirs: Vec<Rc<RefCell<Node>>>,
    parent: Weak<RefCell<Node>>,
}

impl Node {
    fn new(name: &str) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { name: name.to_string(), files: vec![], dirs: vec![], parent: Weak::new()}))
    }

    fn new_with_parent(name: &str, parent: Weak<RefCell<Node>>) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { name: name.to_string(), files: vec![], dirs: vec![], parent}))
    }
}


impl DirEntry {
    fn new(s: &str) -> DirEntry {
        lazy_static! {
            static ref DIR_REGEX: Regex = Regex::new(r"dir (.*)").unwrap();
            static ref FILE_REGEX: Regex = Regex::new(r"(\d+) (.*)").unwrap();
        }

        if let Some(c) = DIR_REGEX.captures(s) {
            DirEntry::Dir(c.get(1).unwrap().as_str().to_string())
        } else if let Some(c) = FILE_REGEX.captures(s) {
            DirEntry::File(FileDetails::new(
                c.get(1).unwrap().as_str().parse::<usize>().unwrap(), c.get(2).unwrap().as_str()
            ))
        } else {
            panic!("Can't process dir entry {}", s);
        }
    }
}

impl FileDetails {
    fn new(size: usize, name: &str) -> FileDetails {
        FileDetails{size, name: name.to_string()}
    }
}

type Input = Vec<Command>;

fn read_input() -> Input {
    lazy_static! {
        static ref CD_REGEX: Regex = Regex::new(r"\$\s+cd\s+(.*)").unwrap();
        static ref LS_REGEX: Regex = Regex::new(r"\$\s*ls.*").unwrap();
    }

    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);

    let mut res = Vec::new();
    let mut lines = reader.lines().flat_map(|l| l.ok()).collect::<Vec<_>>();

    let mut contents = Vec::new();
    while let Some(line) = lines.pop() {
        if let Some(c) = CD_REGEX.captures(&line) {
            info!("Changing directory: {}", line);
            res.push(Command::ChangeDirectory(c.get(1).unwrap().as_str().to_string()));
            continue;
        }

        if LS_REGEX.is_match(&line) {
            info!("Processing {}: Contents {:?}", line, contents);
            res.push(Command::List(contents));
            contents = Vec::new();
            continue;
        }

        // Content of a dir-listing - accumulate
        info!("Adding {} to contents", line);
        let de = DirEntry::new(&line);
        debug!("  DE: {:?}", de);
        contents.push(de);
    }

    res.into_iter().rev().collect()
}

fn build_tree(input: &Input) -> Rc<RefCell<Node>> {
    let tree = Node::new("/");
    let mut current_node = tree.clone();
    for x in input.iter().skip(1) {
        match x {
            Command::ChangeDirectory(n) => {
                if n == ".." {
                    let new_current = current_node.borrow().parent.upgrade().unwrap();
                    current_node = new_current;
                } else {
                    let new_node = Node::new_with_parent(n, Rc::downgrade(&current_node));
                    current_node.borrow_mut().dirs.push(new_node.clone());
                    current_node = new_node;
                }
            },
            Command::List(dl) => {
                current_node.borrow_mut().files.extend(dl.clone());
            },
        }
    }

    tree
}

fn print_tree(tree: &Rc<RefCell<Node>>, indent: u8) {
    let mut tabs = "".to_string();
    for _ in 0..indent {
        tabs += " ";
    }

    println!("{}{}", tabs, tree.borrow().name);

    if let Some(p) = tree.borrow().parent.upgrade() {
        println!("{} Parent is {}", tabs, p.borrow().name);
    } else {
        println!("{} No parent", tabs);
    }

    println!("{} Files:", tabs);
    for f in tree.borrow().files.iter() {
        println!("{}  {:?}", tabs, f);
    }

    println!("{} Subdirs:", tabs);
    for d in tree.borrow().dirs.iter() {
        print_tree(d, indent+1);
    }
}

fn part_1(input: &Input) {
    //println!("Input {:?}", input);
    let tree = build_tree(input);
    print_tree(&tree, 0);
}

fn part_2(input: &Input) {
}


fn main() {
    env_logger::init();

    let input = read_input();
    part_1(&input);
    part_2(&input);
}

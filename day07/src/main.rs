use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use regex::Regex;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::{Cell,RefCell};
use lazy_static::lazy_static;
use log::*;
use std::iter::once;

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
    total_size: usize,
}

impl Node {
    fn new(name: &str) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { name: name.to_string(), files: vec![], dirs: vec![], parent: Weak::new(), total_size: 0}))
    }

    fn new_with_parent(name: &str, parent: Weak<RefCell<Node>>) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { name: name.to_string(), files: vec![], dirs: vec![], parent, total_size: 0}))
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

fn calc_sizes(tree: &Rc<RefCell<Node>>) -> usize {
    let mut total_size = tree.borrow().files.iter()
        .map(|f| match f {
            DirEntry::File(fd) => fd.size,
            _ => 0,
        })
        .sum::<usize>();

    total_size += tree.borrow().dirs.iter()
        .map(|d| calc_sizes(d))
        .sum::<usize>();

    tree.borrow_mut().total_size = total_size;
    total_size
}

fn print_tree(tree: &Rc<RefCell<Node>>, indent: u8) {
    let mut tabs = "".to_string();
    for _ in 0..indent {
        tabs += " ";
    }

    println!("{}{}:{}", tabs, tree.borrow().name, tree.borrow().total_size);

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

fn part_1_impl(node: &Node) -> usize {
    let mut me = 0;
    if node.total_size <= 100000 {
        me = node.total_size;
        println!(">> {} has total size {} <= 100000", node.name, node.total_size);
    }

    node.dirs.iter()
        .map(|d| part_1_impl(&d.borrow()))
        .sum::<usize>() + me
}

fn part_1(input: &Input) {
    let tree = build_tree(input);
    calc_sizes(&tree);
    println!("Part 1: {}", part_1_impl(&tree.borrow()));
}

fn part_2_impl(node: &Node, needed_space: usize, min_size: usize) -> usize {
    if node.total_size < needed_space {
        // No point continuing
        return min_size;
    }

    let mut min_size = min_size;
    if node.total_size < min_size {
        println!("Switching min {}-->{} ({})", min_size, node.total_size, node.name);
        min_size = node.total_size;
    }

    node.dirs.iter()
        .map(|d| part_2_impl(&d.borrow(), needed_space, min_size))
        .chain(once(min_size))
        .min()
        .unwrap()
}

fn part_2(input: &Input) {
    const DISK_SIZE :usize = 70000000;
    const NEEDED_SIZE  :usize = 30000000;

    let tree = build_tree(input);
    calc_sizes(&tree);

    let free_space = DISK_SIZE - tree.borrow().total_size;
    let needed_space = NEEDED_SIZE - free_space;

    println!("Disk size: {}, Free space: {}, Needed space: {}", DISK_SIZE, free_space, needed_space);
    println!("Part 2: {}", part_2_impl(&tree.borrow(), needed_space, usize::MAX));
}


fn main() {
    env_logger::init();

    let input = read_input();
    part_1(&input);
    part_2(&input);
}

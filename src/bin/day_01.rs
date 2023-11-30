#[allow(unused_imports, dead_code)]
use std::string;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
fn main() {
    println!("Running Day 1");
    part_1();
}

const DATA: &str = include_str!("../../input/day_01_sample.txt");

#[allow(unused_variables, dead_code)]
fn load_input() -> Vec<String> {
    println!("Loading input");
    let file_path = "./input/day_01_sample.txt";
    print!("Loading file: {:?}", file_path);
    let file = File::open(file_path).expect("File not found");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    lines
}

#[allow(unused_variables, dead_code)]
fn part_1() -> i32 {
    // let data = load_input();
    let data = DATA.clone();
    print!("data: {:?}", data);

    // Build blocks with iterators
    // let blocks: Vec<Vec<i32>> = data
    //     .trim()
    //     .split("\n\n")
    //     .map(|block| block.split("\n").map(|s| s.parse().unwrap()).collect())
    //     .collect();

    // Build blocks with loops
    let mut blocks: Vec<Vec<i32>> = Vec::new();
    let mut current_block = Vec::<i32>::new();
    for line in data.lines() {
        if line != "" {
            current_block.push(line.parse().unwrap());
        } else {
            blocks.push(current_block);
            current_block = Vec::<i32>::new();
        }
    }
    blocks.push(current_block);

    println!("blocks: {:?}", blocks);
    10
}

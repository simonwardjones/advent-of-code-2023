// use std::vec;

const RAW_DATA: &str = include_str!("../../input/day_01.txt");

fn main() {
    println!("Running Day 1");
    // part_1();
    part_2();
}

#[allow(unused_variables, dead_code)]
fn part_1() -> u32 {
    println!("Part 1");

    let data: Vec<&str> = RAW_DATA.lines().collect();
    // println!("data: {:?}", data);

    let total = data
        .iter()
        .map(|line| get_first_last(line))
        .reduce(|a, b| a + b)
        .expect("Can't reduce");
    println!("total: {:?}", total);
    total
}

fn get_first_last(line: &str) -> u32 {
    let mut number = String::new();
    for char in line.chars() {
        if char.is_digit(10) {
            number.push(char);
            break;
        }
    }
    for char in line.chars().rev() {
        if char.is_digit(10) {
            number.push(char);
            break;
        }
    }
    let value: u32 = number.parse().expect("can't parse to u32");
    value
}

#[allow(unused_variables, dead_code)]
fn part_2() -> u32 {
    println!("Part 2");

    let data: Vec<&str> = RAW_DATA.lines().collect();
    // println!("data: {:?}", data);

    let values = data
        .iter()
        .map(|line| replace_numbers(line))
        .collect::<Vec<String>>();
    // println!("values: {:?}", values);

    let numbers: Vec<u32> = values.iter().map(|line| get_first_last(line)).collect();
    // println!("numbers: {:?}", numbers);

    let total: u32 = numbers.iter().sum();
    println!("total: {:?}", total);
    total
}

fn replace_numbers(line: &str) -> String {
    let mut number = String::new();
    let numbers: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for (char_id, char) in line.chars().enumerate() {
        if char.is_digit(10) {
            number.push(char);
        }
        for (i, pat) in numbers.iter().enumerate() {
            if line[char_id..].starts_with(pat) {
                number.push_str(format!("{}", i + 1).as_str());
            };
        }
    }
    number
}

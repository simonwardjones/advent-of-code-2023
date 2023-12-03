use std::collections::{HashMap, HashSet};

const RAW_DATA: &str = include_str!("../../input/day_03.txt");

fn main() {
    part_one();
    part_two();
}

fn load_data() -> Vec<Vec<char>> {
    let data: Vec<Vec<char>> = RAW_DATA
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    data
}
const STEPS: [(i32, i32); 8] = [
    (1, 0),
    (1, 1),
    (1, -1),
    (0, 1),
    (0, -1),
    (-1, 0),
    (-1, -1),
    (-1, 1),
];

#[allow(dead_code, unused_variables, unused_mut)]
fn part_one() {
    println!("Part 1");
    let data = load_data();
    let n_rows = data.len() as i32;
    let n_cols = data[0].len() as i32;
    let mut valid_numbers: Vec<i32> = Vec::new();
    data.iter().enumerate().for_each(|(i, line)| {
        let mut current_number = String::new();
        let mut valid = false;
        for (j, char) in line.iter().enumerate() {
            if char.is_ascii_digit() {
                current_number.push(*char);
                for step in STEPS {
                    let row = step.0 + i as i32;
                    let column = step.1 + j as i32;
                    if 0 < row && row < n_rows && 0 < column && column < n_cols {
                        let char_check = data[row as usize][column as usize];
                        if char_check != '.' && char_check.is_ascii_punctuation() {
                            valid = true
                        }
                    }
                }
            } else {
                if valid {
                    valid_numbers.push(current_number.parse().unwrap());
                }
                current_number.clear();
                valid = false
            }
        }
        if valid {
            valid_numbers.push(current_number.parse().unwrap());
            current_number.clear();
        }
    });
    let sum_valid = valid_numbers.iter().sum::<i32>();
    println!("sum_valid = {sum_valid:}")
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Gear {
    value: i32,
    row: i32,
    cols: Vec<i32>,
}

#[allow(dead_code, unused_variables)]
fn part_two() {
    println!("Part 2");
    let data = load_data();
    let data = load_data();
    let n_rows = data.len() as i32;
    let n_cols = data[0].len() as i32;
    let mut gears: Vec<Gear> = Vec::new();
    data.iter().enumerate().for_each(|(i, line)| {
        let mut valid = false;
        let mut cols = Vec::<i32>::new();
        let mut current_number = String::new();
        for (j, char) in line.iter().enumerate() {
            if char.is_ascii_digit() {
                current_number.push(*char);
                cols.push(j as i32);
                for step in STEPS {
                    let row = step.0 + i as i32;
                    let column = step.1 + j as i32;
                    if 0 < row && row < n_rows && 0 < column && column < n_cols {
                        let char_check = data[row as usize][column as usize];
                        if char_check != '.' && char_check.is_ascii_punctuation() {
                            valid = true
                        }
                    }
                }
            } else {
                if valid {
                    gears.push(Gear {
                        value: current_number.parse().unwrap(),
                        row: i as i32,
                        cols: cols.to_owned(),
                    })
                }
                current_number.clear();
                cols.clear();
                valid = false
            }
        }
        if valid {
            gears.push(Gear {
                value: current_number.parse().unwrap(),
                row: i as i32,
                cols: cols.to_owned(),
            });
            current_number.clear();
            cols.clear();
        }
    });
    // let's build a convenience hash
    let gears_by_row_col =
        gears
            .iter()
            .fold(HashMap::<(i32, i32), &Gear>::new(), |mut acc, gear| {
                for col in gear.cols.iter() {
                    acc.insert((gear.row, *col), gear);
                }
                acc
            });

    // println!("gears_by_row_col = {gears_by_row_col:?}");
    let mut gear_values = Vec::<i32>::new();
    data.iter().enumerate().for_each(|(i, line)| {
        for (j, char) in line.iter().enumerate() {
            if *char != '*' {
                continue;
            }
            // println!("* {}, {}", i, j);
            // create local_gears set
            let mut local_gears: HashSet<&Gear> = HashSet::new();
            for step in STEPS {
                let row = step.0 + i as i32;
                let column = step.1 + j as i32;
                if let Some(gear) = gears_by_row_col.get(&(row, column)) {
                    local_gears.insert(gear);
                }
            }
            if local_gears.len() == 2 {
                let local_gears: Vec<&Gear> = local_gears.drain().collect();
                gear_values.push(local_gears[0].value * local_gears[1].value);
            }
        }
    });
    println!("gear_values = {gear_values:?}");
    let gear_sum = gear_values.iter().sum::<i32>();
    println!("gear_values.iter().sum::<i32>() = {gear_sum}");
}

use std::collections::HashMap;

const RAW_DATA: &str = include_str!("../../input/day_18.txt");

fn main() {
    part_one();
    part_two();
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Tile {
    colour: &'static str,
    y: i32,
    x: i32,
    is_vertical: bool,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    #[allow(dead_code)]
    fn from_str(string: &str) -> Direction {
        match string {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
    fn as_step(&self) -> (i32, i32) {
        match &self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

impl From<char> for Direction {
    fn from(item: char) -> Direction {
        match item {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

fn load_data() -> Vec<(Direction, i32, &'static str)> {
    let data: Vec<(Direction, i32, &'static str)> = RAW_DATA
        .lines()
        .map(|line| {
            let mut lint = line.split_whitespace();
            let instruction = lint.next().unwrap().chars().next().unwrap().into();
            let distance = lint.next().unwrap().parse::<i32>().unwrap();
            let colour = lint
                .next()
                .unwrap()
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap();
            (instruction, distance, colour)
        })
        .collect();
    data
}

fn print_board(lines: &Vec<Vec<char>>) {
    for line in lines {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
}

#[allow(dead_code, unused_variables)]
fn part_one() {
    use Direction::*;
    println!("Part 1");
    let data = load_data();
    let (mut y, mut x) = (0, 0);
    let mut tiles = HashMap::<(i32, i32), Tile>::new();
    for (ii, (instruction, distance, colour)) in data.iter().enumerate() {
        let next_instruction = data.get(ii + 1).map(|(i, _, _)| i).unwrap_or(&data[0].0);
        let (dy, dx) = instruction.as_step();
        (1..(distance + 1)).for_each(|i| {
            x += dx;
            y += dy;

            let tile = Tile {
                colour,
                y: y,
                x: x,
                is_vertical: match (instruction, next_instruction, i) {
                    (Up, Right, i) | (Right, Down, i) | (Down, Left, i) | (Left, Up, i)
                        if &i == distance =>
                    {
                        true
                    }
                    (Up | Down, _, i) if &i < distance => true,
                    _ => false,
                },
            };

            tiles.insert((tile.y, tile.x), tile);
        });
    }
    // println!("{:?}", tiles);

    // perform flood fill from every point round the edge
    let (min_y, max_y) = (
        tiles.iter().map(|(k, _)| k.0).min().unwrap(),
        tiles.iter().map(|(k, _)| k.0).max().unwrap(),
    );
    let (min_x, max_x) = (
        tiles.iter().map(|(k, _)| k.1).min().unwrap(),
        tiles.iter().map(|(k, _)| k.1).max().unwrap(),
    );
    let mut total = 0;
    // println!("{} {} {} {}", min_y, max_y, min_x, max_x);
    let mut lines: Vec<Vec<char>> = Vec::new();
    for i in min_y..(max_y + 1) {
        let mut line = Vec::new();
        let mut j = min_x;
        let mut in_shape = false;
        // println!("{:?}", tiles.get(&(i, j)).expect("No tile"));
        while j < max_x + 1 {
            if let Some(tile) = tiles.get(&(i, j)) {
                if tile.is_vertical {
                    // println!("Swap {} to {}  at ({} {})", in_shape, !in_shape, i, j);
                    in_shape = !in_shape;
                }
                total += 1;
                line.push('#');
            } else {
                if in_shape {
                    line.push('-');
                    total += 1;
                } else {
                    line.push('.');
                }
            }

            j += 1;
        }
        lines.push(line);
    }
    print_board(&lines);
    println!("Total: {}", total);
    // 29627 too high
}

#[allow(dead_code, unused_variables)]
fn part_two() {
    println!("Part 2");
    let data = load_data();
}

const RAW_DATA: &str = include_str!("../../input/day_18.txt");

fn main() {
    part_two();
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
    fn as_step(&self) -> (i64, i64) {
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

fn load_data() -> Vec<(Direction, i64, &'static str)> {
    let data: Vec<(Direction, i64, &'static str)> = RAW_DATA
        .lines()
        .map(|line| {
            let line = line.split_whitespace();
            let colour = line
                .skip(2)
                .next()
                .unwrap()
                .strip_prefix('(')
                .unwrap()
                .strip_prefix('#')
                .unwrap()
                .strip_suffix(')')
                .unwrap();
            let distance = i64::from_str_radix(&colour[0..5], 16).unwrap();
            let decoded_colour = i64::from_str_radix(&colour[5..6], 16).unwrap();
            let instruction: char = match decoded_colour {
                0 => 'R',
                1 => 'D',
                2 => 'L',
                3 => 'U',
                _ => panic!("Invalid colour"),
            };
            let instruction: Direction = instruction.into();
            // println!("{:?} {:?}", distance, decoded_colour);
            (instruction, distance, colour)
        })
        .collect();
    data
}

fn calculate_area(polygon: &Vec<(i64, i64)>) -> i64 {
    (0..polygon.len())
        .map(|i| {
            let p1 = polygon[i];
            let p2 = polygon[(i + 1) % polygon.len()];

            p1.1 * p2.0 - p1.0 * p2.1 + ((p1.0 + p1.1) - (p2.0 + p2.1)).abs()
        })
        .sum::<i64>()
        / 2
        + 1
}

#[allow(dead_code, unused_variables)]
fn part_two() {
    println!("Part 1");
    let data = load_data();
    let (mut y, mut x) = (0, 0);
    let points: Vec<(i64, i64)> = data
        .iter()
        .map(|(direction, distance, colour)| {
            let (dy, dx) = direction.as_step();
            y += dy * distance;
            x += dx * distance;
            (y, x)
        })
        .collect();

    let total = calculate_area(&points);
    println!("Total: {}", total);
}

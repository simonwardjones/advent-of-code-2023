const RAW_DATA: &str = include_str!("../../input/day_10_sample.txt");

fn main() {
    part_one();
}

fn load_data() -> Vec<Vec<char>> {
    let data: Vec<Vec<char>> = RAW_DATA
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    data
}

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_directions(tile: char) -> Vec<Direction> {
    match tile {
        '|' => vec![Direction::Up, Direction::Down],
        '-' => vec![Direction::Right, Direction::Left],
        'J' => vec![Direction::Up, Direction::Left],
        'F' => vec![Direction::Right, Direction::Down],
        'L' => vec![Direction::Up, Direction::Right],
        '7' => vec![Direction::Down, Direction::Left],
        'S' => vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ],
        _ => vec![],
    }
}

fn get_steps(current_position: &(i32, i32), data: &Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let n_rows = data.len() as i32;
    let n_cols = data[0].len() as i32;
    let (row, col) = current_position.clone();
    let current_tile = data[row as usize][col as usize];

    let mut steps = Vec::new();
    let available_directions = get_directions(current_tile);
    for direction in available_directions {
        let (new_row, new_col) = match direction {
            Direction::Up => (row - 1, col),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
        };
        if new_row < 0 || new_row >= n_rows || new_col < 0 || new_col >= n_cols {
            continue;
        }
        let new_tile = data[new_row as usize][new_col as usize];
        let flipped_direction = match direction {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        };
        let new_tile_directions = get_directions(new_tile);
        if new_tile_directions.contains(&flipped_direction) {
            steps.push((new_row, new_col));
        }
    }
    steps
}

#[allow(dead_code, unused_variables)]
fn part_one() {
    println!("Part 1");
    let data = load_data();
    println!("data = {:?}", data);
    let start_row = data.iter().position(|line| line.contains(&'S')).unwrap();
    let start_col = data[start_row].iter().position(|&c| c == 'S').unwrap();
    let start = (start_row as i32, start_col as i32);
    let mut position = start;
    let mut previous_position = position.clone();
    let mut all_positions = vec![position];
    loop {
        let steps = get_steps(&position, &data);
        // println!(
        //     "position = {:?},previous_position {:?} steps={:?}",
        //     position, previous_position, steps
        // );
        for step in steps.into_iter() {
            if step == previous_position {
                continue;
            } else {
                previous_position = position;
                position = step;
                all_positions.push(position);
                break;
            }
        }
        if position == start {
            println!("Found start");
            break;
        }
    }
    let loop_size = all_positions.len();
    println!("loop_size = {:?}", (loop_size - 1) / 2);
}

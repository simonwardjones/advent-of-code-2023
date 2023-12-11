use std::collections::{HashMap, HashSet};

const RAW_DATA: &str = include_str!("../../input/day_10.txt");

fn main() {
    part_two();
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
    North,
    South,
    West,
    East,
}

fn get_directions(tile: char) -> Vec<Direction> {
    match tile {
        '|' => vec![Direction::North, Direction::South],
        '-' => vec![Direction::East, Direction::West],
        'J' => vec![Direction::North, Direction::West],
        'F' => vec![Direction::East, Direction::South],
        'L' => vec![Direction::North, Direction::East],
        '7' => vec![Direction::South, Direction::West],
        'S' => vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ],
        _ => vec![],
    }
}

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Copy, Clone)]
pub enum Turn {
    Left,
    Right,
    Straight,
}
impl Turn {
    fn flip(&self) -> Turn {
        match self {
            Turn::Left => Turn::Right,
            Turn::Right => Turn::Left,
            Turn::Straight => Turn::Straight,
        }
    }
}

fn add_left_right(
    tile: char,
    position: (i32, i32),
    direction: Direction,
) -> (Turn, Vec<(i32, i32)>, Vec<(i32, i32)>) {
    let (r, c) = position;
    match (direction, tile) {
        (Direction::North, 'F') => (
            Turn::Right,
            vec![(r, c + 1)],
            vec![(r - 2, c - 1), (r - 1, c - 1), (r - 2, c)],
        ),
        (Direction::North, '7') => (
            Turn::Left,
            vec![(r, c - 1)],
            vec![(r - 2, c + 1), (r - 1, c + 1), (r - 2, c)],
        ),
        (Direction::South, 'J') => (
            Turn::Right,
            vec![(r, c - 1)],
            vec![(r + 2, c + 1), (r + 1, c + 1), (r + 2, c)],
        ),
        (Direction::South, 'L') => (
            Turn::Left,
            vec![(r, c + 1)],
            vec![(r + 2, c - 1), (r + 1, c - 1), (r + 2, c)],
        ),
        (Direction::West, 'F') => (
            Turn::Left,
            vec![(r + 1, c)],
            vec![(r - 1, c - 2), (r - 1, c - 1), (r, c - 2)],
        ),
        (Direction::West, 'L') => (
            Turn::Right,
            vec![(r - 1, c)],
            vec![(r + 1, c - 2), (r + 1, c - 1), (r, c - 2)],
        ),
        (Direction::East, 'J') => (
            Turn::Left,
            vec![(r - 1, c)],
            vec![(r + 1, c + 2), (r + 1, c + 1), (r, c + 2)],
        ),
        (Direction::East, '7') => (
            Turn::Right,
            vec![(r + 1, c)],
            vec![(r - 1, c + 2), (r - 1, c + 1), (r, c + 2)],
        ),
        (Direction::North, '|') => (Turn::Straight, vec![(r - 1, c + 1)], vec![(r - 1, c - 1)]),
        (Direction::South, '|') => (Turn::Straight, vec![(r + 1, c - 1)], vec![(r + 1, c + 1)]),
        (Direction::East, '-') => (Turn::Straight, vec![(r + 1, c + 1)], vec![(r - 1, c + 1)]),
        (Direction::West, '-') => (Turn::Straight, vec![(r - 1, c - 1)], vec![(r + 1, c - 1)]),
        (_, 'S') => (Turn::Straight, vec![], vec![]),
        _ => panic!("Unexpected tile {:?} at {:?}", tile, position),
    }
}

fn get_steps(
    current_position: &(i32, i32),
    data: &Vec<Vec<char>>,
) -> (
    Vec<(i32, i32)>,
    Vec<(Turn, Vec<(i32, i32)>, Vec<(i32, i32)>)>,
) {
    let n_rows = data.len() as i32;
    let n_cols = data[0].len() as i32;
    let (row, col) = current_position.clone();
    let current_tile = data[row as usize][col as usize];

    let mut steps = Vec::new();
    let available_directions = get_directions(current_tile);
    let mut turns = Vec::new();
    for direction in available_directions {
        let (new_row, new_col) = match direction {
            Direction::North => (row - 1, col),
            Direction::South => (row + 1, col),
            Direction::West => (row, col - 1),
            Direction::East => (row, col + 1),
        };
        if new_row < 0 || new_row >= n_rows || new_col < 0 || new_col >= n_cols {
            continue;
        }
        let new_tile = data[new_row as usize][new_col as usize];
        let flipped_direction = match direction {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        };
        let new_tile_directions = get_directions(new_tile);

        if new_tile_directions.contains(&flipped_direction) {
            let (turn, inners, outers) = add_left_right(new_tile, (row, col), direction);
            steps.push((new_row, new_col));
            turns.push((turn, inners, outers));
        }
    }
    (steps, turns)
}

#[allow(dead_code, unused_variables)]
fn part_two() {
    println!("Part 2");
    let data = load_data();
    let start_row = data.iter().position(|line| line.contains(&'S')).unwrap();
    let start_col = data[start_row].iter().position(|&c| c == 'S').unwrap();
    let start = (start_row as i32, start_col as i32);
    let mut position = start;
    let mut previous_position = position.clone();
    let mut all_positions = vec![position];
    let mut all_turns: Vec<(Turn, Vec<(i32, i32)>, Vec<(i32, i32)>)> = Vec::new();
    loop {
        let (steps, turns) = get_steps(&position, &data);
        for (i, step) in steps.into_iter().enumerate() {
            if step == previous_position {
                continue;
            } else {
                previous_position = position;
                position = step;
                all_positions.push(position);
                let turn_i = turns[i].clone();
                all_turns.push(turn_i);

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
    // println!("all_positions = {:?}", all_positions);

    let turn_counts = all_turns.clone().into_iter().fold(
        HashMap::new(),
        |mut acc, (turn, inner_corner, outer_corner)| {
            let count = acc.entry(turn).or_insert(0);
            *count += 1;
            acc
        },
    );
    let right_turns = turn_counts.get(&Turn::Right).unwrap_or(&0);
    let left_turns = turn_counts.get(&Turn::Left).unwrap_or(&0);
    let rotation: Turn;
    if right_turns > left_turns {
        rotation = Turn::Right;
    } else {
        rotation = Turn::Left;
    }
    println!("rotation = {:?}", rotation);
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    let mut inner: HashSet<(i32, i32)> = HashSet::new();
    let border: HashSet<(i32, i32)> = all_positions.clone().into_iter().collect();
    let inside: Vec<(i32, i32)> = all_turns
        .iter()
        .filter_map(|(turn, inners, outers)| {
            if *turn == rotation {
                Some(inners.clone())
            } else if *turn == rotation.flip() {
                Some(outers.clone())
            } else {
                None
            }
        })
        .flatten()
        .filter(|point| !border.contains(point))
        .collect();
    // println!("inside = {:?}", inside);
    let mut to_check: HashSet<_> = inside.into_iter().collect();

    while to_check.len() > 0 {
        let mut next_to_check = HashSet::new();
        for point in to_check {
            inner.insert(point);
            seen.insert(point);
            for neighbour in [
                (point.0 + 1, point.1),
                (point.0 - 1, point.1),
                (point.0, point.1 + 1),
                (point.0, point.1 - 1),
            ] {
                if !seen.contains(&neighbour) && !border.contains(&neighbour) {
                    next_to_check.insert(neighbour);
                }
            }
        }
        to_check = next_to_check;
    }
    println!("inner = {:?}", inner.len());
}

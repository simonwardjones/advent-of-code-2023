const RAW_DATA: &str = include_str!("../../input/day_14.txt");

fn main() {
    part_one();
    part_two();
}
type Board = Vec<Vec<char>>;

fn load_data() -> Board {
    let data: Board = RAW_DATA
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    data
}

fn tilt2(board: Board, direction: (i32, i32)) -> Board {
    let mut new_board = board.clone();

    let n = board.len();
    let m = board[0].len();
    // get all points in order of along row down columns
    let points: Vec<(usize, usize)>;
    // let directions = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    match direction {
        (-1, 0) => {
            points = (0..n)
                .flat_map(|i| (0..m).map(move |j| (i, j)))
                .collect::<Vec<_>>()
        }
        (0, -1) => {
            points = (0..m)
                .flat_map(|j| (0..n).map(move |i| (i, j)))
                .collect::<Vec<_>>()
        }
        (1, 0) => {
            points = (0..n)
                .rev()
                .flat_map(|i| (0..m).map(move |j| (i, j)))
                .collect::<Vec<_>>()
        }
        (0, 1) => {
            points = (0..m)
                .rev()
                .flat_map(|j| (0..n).map(move |i| (i, j)))
                .collect::<Vec<_>>()
        }
        _ => panic!("Invalid direction"),
    }
    for (i, j) in points {
        let c = board[i][j];
        if c == 'O' {
            let (x, y) = (i as i32, j as i32);
            let (dx, dy) = direction;
            let (mut nx, mut ny) = (x, y);
            let mut dot_count = 0;
            while (nx + dx >= 0 && ny + dy >= 0)
                && (nx + dx < n as i32 && ny + dy < m as i32)
                && new_board[(nx + dx) as usize][(ny + dy) as usize] != '#'
            {
                if new_board[(nx + dx) as usize][(ny + dy) as usize] == '.' {
                    dot_count += 1;
                }
                nx += dx;
                ny += dy;
            }
            let new_x = x + dx * dot_count;
            let new_y = y + dy * dot_count;
            new_board[x as usize][y as usize] = '.';
            new_board[new_x as usize][new_y as usize] = 'O';
        } else {
            new_board[i][j] = c;
        }
    }
    new_board
}

#[allow(dead_code, unused_variables)]
fn tilt(board: Board, direction: (i32, i32)) -> Board {
    let mut new_board = board.clone();

    for (i, line) in board.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'O' {
                let (x, y) = (i as i32, j as i32);
                let (dx, dy) = direction;
                let (mut nx, mut ny) = (x, y);
                let mut dot_count = 0;
                while (nx + dx >= 0 && ny + dy >= 0)
                    && (nx + dx < board.len() as i32 && ny + dy < line.len() as i32)
                    && new_board[(nx + dx) as usize][(ny + dy) as usize] != '#'
                {
                    if new_board[(nx + dx) as usize][(ny + dy) as usize] == '.' {
                        dot_count += 1;
                    }
                    nx += dx;
                    ny += dy;
                }
                let new_x = x + dx * dot_count;
                let new_y = y + dy * dot_count;
                new_board[x as usize][y as usize] = '.';
                new_board[new_x as usize][new_y as usize] = 'O';
            } else {
                new_board[i][j] = *c;
            }
        }
    }
    new_board
}

#[allow(dead_code, unused_variables)]
fn part_one() {
    println!("Part 1");
    let board = load_data();
    // println!("{board:?}");
    let direction = (-1, 0);
    let new_board = tilt2(board, direction);
    let total =
        new_board
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, line)| {
                acc + line.iter().enumerate().fold(0, |acc, (j, c)| {
                    if *c == 'O' {
                        acc + (1 * (i + 1))
                    } else {
                        acc
                    }
                })
            });
    // println!("{new_board:?}");
    println!("total = {total:?}");
}

#[allow(dead_code, unused_variables)]
fn print_board(board: &Board) {
    for line in board.iter() {
        for c in line.iter() {
            print!("{}", c);
        }
        println!();
    }
    println!();
}
fn get_load(board: Board) -> i32 {
    board.iter().rev().enumerate().fold(0, |acc, (i, line)| {
        acc + line.iter().enumerate().fold(
            0,
            |acc, (_, c)| {
                if *c == 'O' {
                    acc + (1 * (i + 1))
                } else {
                    acc
                }
            },
        ) as i32
    })
}

fn find_pattern(sequence: Vec<i32>) -> Option<Vec<i32>> {
    // starting at the end of the sequence look for a pattern
    let n = sequence.len();
    for i in 1..300 {
        if sequence[(n - i)..] == sequence[(n - (2 * i))..(n - i)] {
            return Some(sequence[(n - i)..].to_vec());
        }
    }
    None
}

#[allow(dead_code, unused_variables)]
fn part_two() {
    println!("Part 2");
    let mut board = load_data();
    let directions = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let mut load = get_load(board.clone());
    let mut loads = vec![load];
    let mut i = 0;
    let burn = 500;
    for _ in 0..burn {
        i += 1;
        for direction in directions.iter() {
            board = tilt2(board.clone(), *direction);
        }
        load = get_load(board.clone());
        loads.push(load);
    }
    println!("loads = {:?}", loads);
    let sequence = find_pattern(loads).expect("No pattern found");
    println!("sequence = {:?}", sequence);
    let remainder = (1000000000 - burn) % sequence.len();
    println!("remainder = {:?}", remainder);
    let final_load = sequence[remainder - 1];
    println!("final_load = {:?}", final_load);
}

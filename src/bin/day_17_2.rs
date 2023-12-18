use std::collections::{hash_map::Entry, HashMap, VecDeque};

const RAW_DATA: &str = include_str!("../../input/day_17.txt");

fn main() {
    part_one();
    part_two();
}

fn load_data() -> Vec<Vec<i32>> {
    let data: Vec<Vec<i32>> = RAW_DATA
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();
    data
}

#[allow(unused)]
fn next_directions(d: (i32, i32), run: i32) -> Vec<((i32, i32), i32)> {
    match (d, run) {
        ((0, b), run) if run < 4 => vec![((0, b), run + 1)],
        ((0, b), run) if run < 10 => vec![((0, b), run + 1), ((1, 0), 1), ((-1, 0), 1)],
        ((0, b), run) if run == 10 => vec![((1, 0), 1), ((-1, 0), 1)],
        ((a, 0), run) if run < 4 => vec![((a, 0), run + 1)],
        ((a, 0), run) if run < 10 => vec![((a, 0), run + 1), ((0, 1), 1), ((0, -1), 1)],
        ((a, 0), run) if run == 10 => vec![((0, 1), 1), ((0, -1), 1)],
        ((-1, -1), run) => vec![((1, 0), 1), ((0, 1), 1)],
        _ => panic!("Bad d"),
    }
}

#[allow(dead_code, unused_variables)]
fn part_one() {
    println!("Part 1");
    let data = load_data();
    let n = data.len() as i32;
    let m = data[0].len() as i32;
    let mut points: VecDeque<((i32, i32), (i32, i32), i32, i32)> = VecDeque::new();
    points.push_back(((0, 0), (-1, -1), 0, 0));
    let mut min_run_heat_loss = HashMap::<(i32, i32, i32, i32, i32), i32>::new();
    let mut min_run = HashMap::<(i32, i32), i32>::new();
    let mut res = 1_000_000;
    while points.len() > 0 {
        let ((y, x), (dy, dx), run, heat_loss) = points.pop_front().unwrap();
        // println!("{:?} {:?} {:?} {:?}", (y, x), (dy, dx), run, heat_loss);
        if y == n - 1 && x == m - 1 && run >= 4 {
            res = res.min(heat_loss)
        }

        match min_run.entry((y, x)) {
            Entry::Occupied(o) => {
                let heat_loss_min = o.into_mut();
                if heat_loss > *heat_loss_min + 100 {
                    continue;
                } else {
                    *heat_loss_min = heat_loss.min(*heat_loss_min);
                }
            }
            Entry::Vacant(v) => {
                v.insert(heat_loss);
            }
        };

        match min_run_heat_loss.entry((y, x, dy, dx, run)) {
            Entry::Occupied(o) => {
                let heat_loss_min = o.into_mut();
                if heat_loss < *heat_loss_min {
                    *heat_loss_min = heat_loss;
                } else {
                    continue;
                }
            }
            Entry::Vacant(v) => {
                v.insert(heat_loss);
            }
        };
        for ((ndy, ndx), n_run) in next_directions((dy, dx), run) {
            let next_point = (y + ndy, x + ndx);
            if y + ndy < 0 || y + ndy >= n || x + ndx < 0 || x + ndx >= m {
                continue;
            }
            let next_status: ((i32, i32), (i32, i32), i32, i32) = (
                next_point,
                (ndy, ndx),
                n_run,
                heat_loss + data[next_point.0 as usize][next_point.1 as usize],
            );
            points.push_back(next_status)
        }
    }
    println!("{res:?}")
}

#[allow(dead_code, unused_variables)]
fn part_two() {
    println!("Part 2");
    let data = load_data();
}

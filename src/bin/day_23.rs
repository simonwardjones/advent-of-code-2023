use std::collections::{HashMap, HashSet, VecDeque};

const RAW_MAP: &str = include_str!("../../input/day_23.txt");

fn main() {
    part_one();
    part_two();
}

fn load_map() -> Vec<&'static str> {
    let map: Vec<&'static str> = RAW_MAP.lines().collect();
    map
}
type Point = (i32, i32);

#[allow(dead_code, unused_variables)]
fn part_one() {
    println!("Part 1");
    let map = load_map();
    let n = map.len() as i32;
    let m = map[0].len() as i32;
    let start: Point = (0, 1);
    let end: Point = (n - 1, m - 2);
    println!("start: {:?}, end: {:?}", start, end);
    let mut todo = VecDeque::new();
    let previous: Point = (-1, 1);
    todo.push_back((previous, start, 0));
    let mut walks = Vec::new();

    while let Some((previous, point, distance)) = todo.pop_front() {
        // println!(
        //     "previous, point, distance = {:?}, {:?}, {:?}",
        //     previous, point, distance
        // );
        if point == end {
            walks.push(distance);
            // println!("Found end at distance {}", distance);
            continue;
        }
        let (y, x) = point;
        for next_point in [(y, x + 1), (y, x - 1), (y + 1, x), (y - 1, x)] {
            if next_point == previous {
                continue;
            }
            if next_point.0 < 0 || next_point.0 >= n || next_point.1 < 0 || next_point.1 >= m {
                continue;
            }
            let next_char = map[next_point.0 as usize]
                .chars()
                .nth(next_point.1 as usize)
                .unwrap();
            match (next_char, next_point.0 - y, next_point.1 - x) {
                ('#', _, _) => {
                    continue;
                }
                ('<', 0, -1) | ('>', 0, 1) | ('v', 1, 0) | ('.', _, _) => {
                    todo.push_back((point, next_point, distance + 1));
                }
                _ => {
                    // panic!("next char {}, next point {:?}", next_char, next_point);
                    continue;
                }
            }
        }
    }
    // println!("{map:?}");
    // println!("{walks:?}");
    println!("{:?}", walks.iter().max());
}

fn get_neighbours(p: Point, map: &Vec<&'static str>) -> Vec<Point> {
    let n = map.len() as i32;
    let m = map[0].len() as i32;
    let (y, x) = p;
    [(y, x + 1), (y, x - 1), (y + 1, x), (y - 1, x)]
        .into_iter()
        .filter(|next_point| {
            next_point.0 >= 0 && next_point.0 < n && next_point.1 >= 0 && next_point.1 < m
        })
        .filter(|p| map[p.0 as usize].chars().nth(p.1 as usize).unwrap() != '#')
        .collect()
}

#[allow(dead_code, unused_variables)]
fn part_two() {
    println!("Part 2");
    let map = load_map();
    let n = map.len() as i32;
    let m = map[0].len() as i32;
    let start: Point = (0, 1);
    let end: Point = (n - 1, m - 2);
    println!("start: {:?}, end: {:?}", start, end);
    let mut todo = VecDeque::new();
    let mut seen = HashSet::new();
    seen.insert(start);
    todo.push_back(start);
    let mut graph = HashMap::<Point, Vec<(Point, i32)>>::new();
    let mut done = HashSet::new();
    done.insert(start);
    while let Some(point) = todo.pop_front() {
        // println!("Handling point = {:?}", point);
        let mut inner_to_do = VecDeque::new();
        let mut seen = HashSet::new();
        seen.insert(point);
        get_neighbours(point, &map)
            .iter()
            .for_each(|n| inner_to_do.push_back((*n, 1)));
        while let Some((current, d)) = inner_to_do.pop_front() {
            // println!("point, current, d = {:?}, {:?}, {}", point, current, d);
            let neighbours = get_neighbours(current, &map);
            if neighbours.len() > 2 || current == end {
                graph.entry(point).or_insert(vec![]).push((current, d));
                if !done.contains(&current) {
                    todo.push_back(current)
                }
                done.insert(current);
            } else {
                for n in neighbours {
                    if !seen.contains(&n) {
                        inner_to_do.push_back((n, d + 1));
                        seen.insert(n);
                    }
                }
            }
        }
    }

    let mut todo = VecDeque::new();
    let mut walks: Vec<i32> = Vec::new();
    let mut seen = HashSet::new();
    seen.insert(start);
    todo.push_back((seen, start, 0));
    println!("{:?}", graph);
    while let Some((seen, point, d)) = todo.pop_front() {
        // println!("point = {:?}, d = {}", point, d);
        if point == end {
            walks.push(d);
            continue;
        }
        for (n, dd) in graph.get(&point).unwrap() {
            if !seen.contains(n) {
                let mut new_seen = seen.clone();
                new_seen.insert(*n);
                todo.push_back((new_seen, *n, d+dd))
            }
        }
    }
    // println!("walks: {walks:?}");
    println!("walk max {}", walks.iter().max().expect("no max"));
}

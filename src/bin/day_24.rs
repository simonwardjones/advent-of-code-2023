use std::{collections::btree_set::Intersection, cmp};

const RAW_DATA: &str = include_str!("../../input/day_24.txt");

fn main() {
    part_one();
    part_two();
}

fn to_int(a: &str) -> i128 {
    match a {
        a if a.starts_with("-") => -a[1..].parse::<i128>().unwrap(),
        b => b.trim().parse().unwrap(),
    }
}

fn load_data() -> Vec<(Vec<i128>, Vec<i128>)> {
    let data: Vec<(Vec<i128>, Vec<i128>)> = RAW_DATA
        .lines()
        .map(|line| {
            let (position, velocity) = line.split_once(" @ ").unwrap();
            let position: Vec<i128> = position.split(", ").map(to_int).collect();
            let velocity: Vec<i128> = velocity.split(", ").map(to_int).collect();
            (position, velocity)
        })
        .collect();
    data
}

#[allow(dead_code, unused_variables)]
fn part_one() {
    println!("Part 1");
    // x1 + m1 * t1 = x2 + m2 * t2
    // => (x2 - x1) + mx2*t2 - mx1*t1 = 0
    // => (y2 - y1) + my2*t2 - my1*t1 = 0
    // => (x2 - x1)*my1 + mx2*t2*my1 - (y2 - y1)*mx1 - my2*t2*mx1 = 0
    // => t2 = ((x2 - x1)*my1 - (y2 - y1)*mx1) / (my2*mx1 - mx2*my1)
    let data = load_data();
    let n = data.len();
    let mut intersections = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            let a = &data[i];
            let b = &data[j];
            let intersection = get_intersection_x_y(a, b);
            if intersection {
                intersections += 1
            }
            // println!("Intersection = {}", intersection);
        }
    }

    println!("{intersections:?}");
}

fn get_intersection_x_y(
    point_a: &(Vec<i128>, Vec<i128>),
    point_b: &(Vec<i128>, Vec<i128>),
) -> bool {
    // println!("A: {:?}, ", point_a);
    // println!("B: {:?}, ", point_b);
    let (x1, y1, mx1, my1) = (point_a.0[0], point_a.0[1], point_a.1[0], point_a.1[1]);
    let (x2, y2, mx2, my2) = (point_b.0[0], point_b.0[1], point_b.1[0], point_b.1[1]);
    if mx2 * my1 == my2 * mx1 {
        // println!("Parallel");
        return false;
    }
    let t2: f64 = ((x2 - x1) * my1 - (y2 - y1) * mx1) as f64 / (my2 * mx1 - mx2 * my1) as f64;
    let t1 = (x2 as f64 + mx2 as f64 * t2 - x1 as f64) / mx1 as f64;
    let x11 = x1 as f64 + mx1 as f64 * t1;
    let y11 = y1 as f64 + my1 as f64 * t1;
    // let x22 = x2 as f64 + mx2 as f64 * t2;
    // let y22 = y2 as f64 + my2 as f64 * t2;
    if t1 < 0.0 || t2 < 0.0 {
        // println!("Past!");
        return false;
    }
    // println!(
    //     "x11 {:?}, y11 {}, x22 {:?}, y22 {} t1 {} t2 {}",
    //     x11, y11, x22, y22, t1, t2
    // );
    // let ret = 7.0 <= x11 && x11 <= 27.0 && 7.0 <= y11 && y11 <= 27.0;
    let ret = 200000000000000.0 <= x11
        && x11 <= 400000000000000.0
        && 200000000000000.0 <= y11
        && y11 <= 400000000000000.0;
    if ret && x11.fract() == 0.0 && y11.fract() == 0.0 {
        // println!("{}, {}", x11, y11);
    }

    ret
}

fn get_intersection_x_y_val(
    point_a: &(Vec<i128>, Vec<i128>),
    point_b: &(Vec<i128>, Vec<i128>),
) -> Option<Vec<i128>> {
    // println!("A: {:?}, ", point_a);
    // println!("B: {:?}, ", point_b);
    let (x1, y1, mx1, my1) = (point_a.0[0], point_a.0[1], point_a.1[0], point_a.1[1]);
    let (x2, y2, mx2, my2) = (point_b.0[0], point_b.0[1], point_b.1[0], point_b.1[1]);
    if mx2 * my1 == my2 * mx1 {
        // println!("Parallel");
        return None;
    }
    let t2: f64 = ((x2 - x1) * my1 - (y2 - y1) * mx1) as f64 / (my2 * mx1 - mx2 * my1) as f64;
    let t1 = (x2 as f64 + mx2 as f64 * t2 - x1 as f64) / mx1 as f64;
    let x11 = x1 as f64 + mx1 as f64 * t1;
    let y11 = y1 as f64 + my1 as f64 * t1;
    let z11 = point_a.0[2] as f64 + point_a.1[2] as f64 * t1;
    // let x22 = x2 as f64 + mx2 as f64 * t2;
    // let y22 = y2 as f64 + my2 as f64 * t2;
    if t1 < 0.0 || t2 < 0.0 {
        // println!("Past!");
        return None;
    }
    // println!(
    //     "x11 {:?}, y11 {}, x22 {:?}, y22 {} t1 {} t2 {}",
    //     x11, y11, x22, y22, t1, t2
    // );
    // let ret = 7.0 <= x11 && x11 <= 27.0 && 7.0 <= y11 && y11 <= 27.0;
    let ret = 200000000000000.0 <= x11
        && x11 <= 400000000000000.0
        && 200000000000000.0 <= y11
        && y11 <= 400000000000000.0;
    if ret && x11.fract() == 0.0 && y11.fract() == 0.0 && z11.fract() == 0.0 {
        // println!("{}, {}", x11, y11);
        return Some(vec![x11 as i128, y11 as i128, z11 as i128]);
    }
    None
}

#[allow(dead_code, unused_variables)]
fn part_two() {
    println!("Part 2");
    let data = load_data();
    let n = data.len();
    let mut intersections = vec![];
    for i in 0..n {
        for j in (i + 1)..n {
            let a = &data[i];
            let b = &data[j];
            if let Some(intersection) = get_intersection_x_y_val(a, b){
                intersections.push(intersection)
            }
        }
    }
    intersections.sort_by(|a, b| Ord::cmp(&a[0], &b[0]));

    println!("{intersections:?}")
}

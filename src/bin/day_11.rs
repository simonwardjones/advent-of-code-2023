const RAW_DATA: &str = include_str!("../../input/day_11.txt");

fn main() {
    part_one();
    part_two();
}

fn load_data() -> Vec<Vec<i32>> {
    let data: Vec<Vec<i32>> = RAW_DATA
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => 0,
                    '#' => 1,
                    _ => panic!("Unknown character: {}", c),
                })
                .collect()
        })
        .collect();
    data
}

#[allow(dead_code, unused_variables)]
fn total_distance(part: i32) {
    let data = load_data();
    let row_counts: Vec<i32> = data.iter().map(|r| r.iter().sum()).collect();
    let col_counts: Vec<i32> = (0..(data[0].len()))
        .map(|c| data.iter().map(|r| r[c]).sum())
        .collect();
    let points: Vec<Point> = data
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &v)| v == 1)
                .map(move |(x, _)| (y as i32, x as i32))
        })
        .collect();
    let mut total_distance = 0;
    let mut combos = 0;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let p1 = points[i];
            let p2 = points[j];
            let d = distance(p1, p2, &row_counts, &col_counts, part);
            total_distance += d;
            combos += 1;
            // println!("{:?} -> {:?} = {:?}", p1, p2, d);
        }
    }
    println!("{total_distance:?}, combos: {combos:?}");
}

type Point = (i32, i32);

#[allow(dead_code, unused_variables)]
fn distance(
    from: Point,
    to: Point,
    row_counts: &Vec<i32>,
    col_counts: &Vec<i32>,
    part: i32,
) -> i128 {
    let (y1, x1) = from;
    let (y2, x2) = to;
    let dy = (y1 - y2).abs() as i128;
    let dx = (x1 - x2).abs() as i128;
    let min_y = y1.min(y2);
    let max_y = y1.max(y2);
    let min_x = x1.min(x2);
    let max_x = x1.max(x2);
    let extra_x: i128 = (min_x..max_x)
        .filter(|&x| col_counts[x as usize] == 0)
        .count() as i128;
    let extra_y: i128 = (min_y..max_y)
        .filter(|&y| row_counts[y as usize] == 0)
        .count() as i128;
    let multiplier: i128 = if part == 1 { 1 } else { 1000000 - 1 };
    return dx + dy + (extra_x + extra_y) * multiplier;
}

#[allow(dead_code, unused_variables)]
fn part_one() {
    println!("Part 1");
    total_distance(1);
}
#[allow(dead_code, unused_variables)]
fn part_two() {
    println!("Part 2");
    total_distance(2);
}

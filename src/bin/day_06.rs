const RAW_DATA: &str = include_str!("../../input/day_06_sample.txt");

fn main() {
    part_one();
    part_two();
}

fn load_data() -> (Vec<i32>, Vec<i32>) {
    let mut data = RAW_DATA.lines();
    let times = data
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|e| e.parse().unwrap())
        .collect();
    let distances = data
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|e| e.parse().unwrap())
        .collect();

    (times, distances)
}

#[allow(dead_code, unused_variables)]
fn part_one() {
    println!("Part 1");
    let (times, distances) = load_data();
    println!("Times: {:?}, Distance {:?}", times, distances);
    let results: i32 = times
        .iter()
        .zip(distances)
        .map(|(time, distance)| {
            (1..(time - 1))
                .filter(|speed| (time - speed) * speed > distance)
                .count() as i32
        })
        .product();
    println!("results= {:?}", results);
}

#[allow(dead_code, unused_variables)]
fn part_two() {
    println!("Part 2");
    let time: i64 = 44899691;
    let distance: i64 = 277113618901768;
    let winners = (1..(time - 1))
        .filter(|speed| (time - speed) * speed > distance)
        .count();
    println!("winners = {winners}");
}

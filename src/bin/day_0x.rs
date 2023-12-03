const RAW_DATA: &str = include_str!("../../input/day_03_sample.txt");

fn main() {
    part_one();
    part_two();
}

fn load_data() -> Vec<&'static str> {
    let data: Vec<&'static str> = RAW_DATA.lines().collect();
    data
}

#[allow(dead_code, unused_variables)]
fn part_one() {
    println!("Part 1");
    let data = load_data();
    println!("{data:?}")
}

#[allow(dead_code, unused_variables)]
fn part_two() {
    println!("Part 2");
    let data = load_data();
}

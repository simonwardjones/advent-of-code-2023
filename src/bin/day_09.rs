const RAW_DATA: &str = include_str!("../../input/day_09.txt");

fn main() {
    part_one();
    part_two();
}

fn load_data() -> Vec<Vec<i32>> {
    let data: Vec<Vec<i32>> = RAW_DATA
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    data
}

#[allow(dead_code, unused_variables)]
fn part_one() {
    println!("Part 1");
    let data = load_data();
    let mut total: i32 = 0;
    for line in data {
        let mut line_total: i32 = line.last().expect("no last difference").clone();
        let mut differences: Vec<i32> = line
            .iter()
            .zip(line.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect();

        while !differences.iter().all(|d| *d == 0) {
            line_total += differences.last().expect("no last difference");
            differences = differences
                .iter()
                .zip(differences.iter().skip(1))
                .map(|(a, b)| b - a)
                .collect();
            // println!("{:?} {:?}, {:?}", line, differences, line_total);
        }
        // println!("{:?} {:?}", line, differences);
        // println!("---");
        total += line_total;
    }
    println!("{total:?}")
}

#[allow(dead_code, unused_variables)]
fn part_two() {
    println!("Part 2");
    let data = load_data();
    let data = load_data();
    let mut total: i32 = 0;
    for line in data {
        let mut line_starts: Vec<i32> = vec![line.first().expect("no last difference").clone()];
        let mut differences: Vec<i32> = line
            .iter()
            .zip(line.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect();

        while !differences.iter().all(|d| *d == 0) {
            line_starts.push(differences.first().expect("no last difference").clone());
            differences = differences
                .iter()
                .zip(differences.iter().skip(1))
                .map(|(a, b)| b - a)
                .collect();
        }
        let line_total = line_starts.into_iter().rev().reduce(|a, b| b  - a ).expect("no total");
        // println!("{:?} {:?}", line, line_total);
        total += line_total;
    }
    println!("first total = {total:?}")
}

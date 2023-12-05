use itertools::Itertools;

const RAW_DATA: &str = include_str!("../../input/day_05.txt");

fn main() {
    // part_one();
    part_two();
}

fn load_data() -> (Vec<i64>, Vec<Vec<Vec<i64>>>) {
    let data: Vec<&'static str> = RAW_DATA.lines().collect();
    let seeds: Vec<i64> = data[0]
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|seed| seed.parse().unwrap())
        .collect();
    let data: String = data.iter().skip(2).fold(String::new(), |a, b| a + b + "\n");
    // println!("data: {data:?}");
    let maps: Vec<Vec<Vec<i64>>> = data
        .split("\n\n")
        .map(|map| {
            map.lines()
                .skip(1)
                .map(|line| {
                    line.split_whitespace()
                        .map(|e| e.parse::<i64>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();
    (seeds, maps)
}

#[allow(dead_code, unused_variables)]
fn part_one() {
    let (seeds, maps) = load_data();
    let total: Vec<_> = seeds
        .iter()
        .map(|seed| {
            let mut value: i64 = *seed;
            for map in &maps {
                for map_value in map {
                    let (out_start, in_start, step) = (map_value[0], map_value[1], map_value[2]);
                    if (in_start..(in_start + step)).contains(&value) {
                        value = out_start + (value - in_start);
                        break;
                    }
                }
            }
            value
        })
        .collect();
    println!("total: {:?}", total.iter().min());
}

#[allow(dead_code, unused_variables)]
fn part_two() {
    println!("Part 2");
    let (seeds, maps) = load_data();
    let mut ranges: Vec<(i64, i64)> = seeds.into_iter().tuples().collect();
    println!("ranges: {:?}", ranges);
    for mut map in maps {
        let mut new_ranges = Vec::<(i64, i64)>::new();
        map.sort_by_key(|x| x[1]);
        let min_map = map[0][1];
        let max_map = map[map.len() - 1][1] + map[map.len() - 1][2];
        let full_range = (min_map, min_map, max_map - min_map);
        println!("full_range: {:?}", full_range);
        for range in &ranges {
            for map_value in &map {
                let b = (map_value[0], map_value[1], map_value[2]);
                if let Some(intersection) = intersection(range, &b) {
                    new_ranges.push(intersection);
                }
            }
            if let Some(remainder) = remainder(range, &full_range) {
                new_ranges.push(remainder);
            }
        }
        ranges = new_ranges;
    }
    println!("ranges: {:?}", ranges);
    let out = ranges.iter().min_by_key(|(a, _)| a);
    println!("out: {:?}", out)
}

#[allow(dead_code, unused_variables)]
fn intersection(a: &(i64, i64), b: &(i64, i64, i64)) -> Option<(i64, i64)> {
    if a.0 < b.1 + b.2 && a.0 + a.1 > b.1 {
        let intersection = (a.0.max(b.1), (a.0 + a.1).min(b.1 + b.2) - a.0.max(b.1));
        let shifted_intersection = Some((b.0 + (intersection.0 - b.1), intersection.1));
        return shifted_intersection;
    }
    None
}

#[allow(dead_code, unused_variables)]
fn remainder(a: &(i64, i64), b: &(i64, i64, i64)) -> Option<(i64, i64)> {
    if a.0 < b.0 {
        let top = (a.0 + a.1).min(b.1);
        return Some((a.0, top - a.0));
    } else if a.0 + a.1 > b.1 + b.2 {
        let bottom = a.0.max(b.1 + b.2);
        return Some((bottom, (a.0 + a.1) - bottom));
    }
    None
}

use std::collections::HashSet;

const RAW_DATA: &str = include_str!("../../input/day_21.txt");

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
    let mut start = (0, 0);
    for (row, line) in data.iter().enumerate() {
        for (column, element) in line.chars().enumerate() {
            if element == 'S' {
                start = (row as i32, column as i32);
            }
        }
    }
    let offsets = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut positions: Vec<(i32, i32)> = vec![start];
    let n = data.len() as i32;
    let m = data[0].len() as i32;
    let rounds = 64;
    for round in 0..rounds {
        let unique_positions: HashSet<(i32, i32)> = positions
            .iter()
            .flat_map(|(row, column)| {
                offsets
                    .clone()
                    .into_iter()
                    .map(|(row_offset, column_offset)| {
                        (*row as i32 + row_offset, *column as i32 + column_offset)
                    })
                    .filter(|(r, c)| {
                        r >= &0
                            && r < &n
                            && c >= &0
                            && c < &m
                            && data[*r as usize].chars().nth(*c as usize).unwrap() != '#'
                    })
            })
            .collect();
        positions = unique_positions.into_iter().collect();
    }
    println!("{:?}", positions.len());
}

#[allow(dead_code, unused_variables)]
fn part_two() {
    println!("Part 2");
    let data = load_data();
    let steps = 26501365;
    let mut start = (0, 0);
    for (row, line) in data.iter().enumerate() {
        for (column, element) in line.chars().enumerate() {
            if element == 'S' {
                start = (row as i32, column as i32);
            }
        }
    }
    let offsets = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut positions: Vec<(i32, i32)> = vec![start];
    let n = data.len() as i32;
    let m = data[0].len() as i32;
    // Ok so new thought
    // Every point is gettable to in either an even number e or odd o, then they are
    // gettable every e + 2n or o + 2n.

    // 132 to get to diagonal external corners
    // 66 to get to  middle or 131 to get to next S
    // (26501365 - 65) / 131 = 202300

    // 130 -- 7770
    // 131 - 7627
    // 132 - 7490
    println!("m: {}, n: {}", m, n);
    let rounds = 131 * 4 + 65;
    let mut odd_seen = 0;
    let mut even_seen = 0;
    let mut seen_total;
    let mut seen = HashSet::new();
    let mut seens = Vec::new();
    for round in 0..rounds {
        let unique_positions: HashSet<(i32, i32)> = positions
            .iter()
            .flat_map(|(row, column)| {
                offsets
                    .clone()
                    .into_iter()
                    .map(|(row_offset, column_offset)| {
                        (*row as i32 + row_offset, *column as i32 + column_offset)
                    })
                    .filter(|(r, c)| {
                        !seen.contains(&(*r, *c))
                            && data[r.rem_euclid(n) as usize]
                                .chars()
                                .nth(c.rem_euclid(m) as usize)
                                .unwrap()
                                != '#'
                    })
            })
            .collect();
        positions = unique_positions.into_iter().collect();
        seen.extend(positions.clone());
        if round % 2 != 0 {
            even_seen += positions.len();
            seen_total = even_seen;
        } else {
            odd_seen += positions.len();
            seen_total = odd_seen;
        }
        if (round - 64) % (131 * 2) == 0 {
            seens.push(seen_total);
            println!("round {}, seen {:?}", round, seen_total);
        }
    }
    println!("{:?}", seens);
    let first_differences: Vec<_> = seens
        .iter()
        .zip(seens.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect();
    let second_differences: Vec<_> = first_differences
        .iter()
        .zip(first_differences.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect();
    println!("{:?}", first_differences);
    println!("{:?}", second_differences);
    let a = (second_differences[0] / 2) as i128;
    let b = first_differences[0] as i128 - (3 * a);
    let c = seens[0] as i128 - (a + b);
    println!("a: {}, b: {}, c: {}", a, b, c);
    let x = (steps - 65) / (131 * 2) + 1;
    println!("x: {}", x);
    let total = a * x * x + b * x + c;
}

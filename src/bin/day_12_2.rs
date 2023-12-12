use std::collections::HashMap;

const RAW_DATA: &str = include_str!("../../input/day_12.txt");

fn main() {
    part_two();
}

fn load_data() -> Vec<(Vec<i64>, Vec<i64>)> {
    let data: Vec<&str> = RAW_DATA.lines().collect();
    let data = data
        .iter()
        .map(|&line| {
            let (springs, counts) = line.split_once(" ").expect("Invalid input");
            let springs: Vec<i64> = springs
                .chars()
                .map(|c| match c {
                    '#' => 1,
                    '.' => 0,
                    '?' => -1,
                    _ => panic!("Invalid spring"),
                })
                .collect();
            let counts: Vec<i64> = counts
                .split(',')
                .map(|c| c.parse().expect("can't pass count"))
                .collect();
            (springs, counts)
        })
        .collect();
    data
}

//    ???.###    1,1,3

#[allow(dead_code, unused_variables)]
fn get_valid_springs(
    springs: Vec<i64>,
    counts: Vec<i64>,
    current_run: i64,
    hashes_required: i64,
    mut cache: &mut HashMap<(Vec<i64>, Vec<i64>, i64, i64), i64>,
) -> i64 {
    let key = (
        springs.clone(),
        counts.clone(),
        current_run,
        hashes_required,
    );
    if let Some(value) = cache.get(&key) {
        return *value;
    }
    if springs.len() == 0 {
        if (counts.len() == 0 && hashes_required == 0)
            || (counts.len() == 1 && counts[0] == current_run && hashes_required == 0)
        {
            return 1;
        }
        return 0;
    }

    if hashes_required < 0 {
        // println!("Too many hashes");
        return 0;
    }
    if hashes_required > springs.iter().filter(|&&x| x == -1).count() as i64 {
        // println!("Not enough space for hashes");
        return 0;
    }

    let spring = springs[0];
    match spring {
        0 => {
            if current_run > 0 {
                // closing a run
                if counts.len() == 0 {
                    // println!("Found a run but no counts left");
                    return 0;
                } else if counts[0] != current_run {
                    // println!(
                    //     "Closed run mismatch got {} expected {}",
                    //     current_run, counts[0]
                    // );
                    return 0;
                } else {
                    return get_valid_springs(
                        springs[1..].to_vec(),
                        counts[1..].to_vec(),
                        0,
                        hashes_required,
                        &mut cache,
                    );
                }
            } else {
                return get_valid_springs(
                    springs[1..].to_vec(),
                    counts.clone(),
                    0,
                    hashes_required,
                    &mut cache,
                );
            }
        }
        1 => {
            if counts.len() > 0 && current_run >= counts[0] {
                // println!("Longer run than expected");
                return 0;
            } else {
                return get_valid_springs(
                    springs[1..].to_vec(),
                    counts.clone(),
                    current_run + 1,
                    hashes_required,
                    &mut cache,
                );
            }
        }
        -1 => {
            let mut total_split = 0;
            if hashes_required > 0 {
                let mut springs_hash = springs.clone();
                springs_hash[0] = 1;
                total_split += get_valid_springs(
                    springs_hash,
                    counts.clone(),
                    current_run,
                    hashes_required - 1,
                    &mut cache,
                );
            }
            let mut springs_dot = springs.clone();
            springs_dot[0] = 0;
            total_split += get_valid_springs(
                springs_dot,
                counts.clone(),
                current_run,
                hashes_required,
                &mut cache,
            );
            cache.insert(key, total_split);
            return total_split;
        }
        _ => panic!("Invalid spring"),
    }
}

#[allow(dead_code, unused_variables)]
fn part_two() {
    println!("Part 2");
    let data = load_data();
    let mut all_total = 0;
    for (springs, counts) in data.iter() {
        let mut new_springs = springs.clone();
        let mut new_counts = counts.clone();
        for i in 0..4 {
            new_springs.push(-1);
            new_springs.extend(springs);
            new_counts.extend(counts)
        }
        let mut cache = HashMap::<(Vec<i64>, Vec<i64>, i64, i64), i64>::new();

        let total_hashes = new_counts.iter().sum::<i64>();
        let fixed_hashes = new_springs.iter().filter(|&&x| x == 1).count() as i64;
        let hashes_required = total_hashes - fixed_hashes;

        let total = get_valid_springs(
            new_springs.clone(),
            new_counts.to_vec(),
            0,
            hashes_required,
            &mut cache,
        );
        println!("Total: {}", total);
        all_total += total;
    }
    println!("All Total: {}", all_total);
}

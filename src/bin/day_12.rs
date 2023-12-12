const RAW_DATA: &str = include_str!("../../input/day_12_sample.txt");

fn main() {
    part_one();
    part_two();
}

fn load_data() -> Vec<(Vec<i32>, Vec<i32>)> {
    let data: Vec<&str> = RAW_DATA.lines().collect();
    let data = data
        .iter()
        .map(|&line| {
            let (springs, counts) = line.split_once(" ").expect("Invalid input");
            let springs: Vec<i32> = springs
                .chars()
                .map(|c| match c {
                    '#' => 1,
                    '.' => 0,
                    '?' => -1,
                    _ => panic!("Invalid spring"),
                })
                .collect();
            let counts: Vec<i32> = counts
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
fn get_valid_springs(springs: Vec<i32>, counts: &[i32]) -> i32 {
    let mut working_springs = Vec::new();
    let total_hashes = counts.iter().sum::<i32>();
    let fixed_hashes = springs.iter().filter(|&&x| x == 1).count() as i32;
    let hashes_required = total_hashes - fixed_hashes;
    let (springs, pos, current_run, counts, hashes_required) =
        (springs, 0, 0, counts, hashes_required);
    working_springs.push((springs, pos, current_run, counts, hashes_required));
    let mut total = 0;
    while working_springs.len() > 0 {
        let (springs, pos, mut current_run, counts, hashes_required) =
            working_springs.pop().unwrap();
        // println!(
        //     "{:?} {:?} {:?} {:?} {:?}",
        //     springs, pos, current_run, counts, hashes_required
        // );
        if pos == springs.len() {
            if (counts.len() == 0 && hashes_required == 0)
                || (counts.len() == 1 && counts[0] == current_run && hashes_required == 0)
            {
                // println!("Found a full run!");
                total += 1;
            }
            // println!("Got to the end but didn't find a full run");
            continue;
        }
        // TODO  early exit for hashes required
        if hashes_required < 0 {
            // println!("Too many hashes");
            continue;
        }
        if hashes_required > springs.len() as i32 - pos as i32 {
            // println!("Not enough space for hashes");
            continue;
        }
        if hashes_required > springs[pos..].iter().filter(|&&x| x == -1).count() as i32 {
            // println!("Not enough space for hashes");
            continue;
        }
        if counts.iter().sum::<i32>() > springs.len() as i32 - pos as i32 + r {
            // println!("Not enough space for counts");
            continue;
        }

        let spring = springs[pos];
        match spring {
            0 => {
                if current_run > 0 {
                    // closing a run
                    if counts.len() == 0 {
                        // println!("Found a run but no counts left");
                    } else if counts[0] != current_run {
                        // println!(
                        //     "Closed run mismatch got {} expected {}",
                        //     current_run, counts[0]
                        // );
                    } else {
                        working_springs.push((springs, pos + 1, 0, &counts[1..], hashes_required));
                    }
                } else {
                    working_springs.push((springs, pos + 1, current_run, counts, hashes_required));
                }
            }
            1 => {
                current_run += 1;
                if counts.len() > 0 && current_run > counts[0] {
                    // println!("Longer run than expected");
                } else {
                    working_springs.push((springs, pos + 1, current_run, counts, hashes_required));
                }
            }
            -1 => {
                if hashes_required > 0 {
                    let mut springs_hash = springs.clone();
                    springs_hash[pos] = 1;
                    working_springs.push((
                        springs_hash,
                        pos,
                        current_run,
                        counts,
                        hashes_required - 1,
                    ));
                }
                let mut springs_dot = springs.clone();
                springs_dot[pos] = 0;
                working_springs.push((springs_dot, pos, current_run, counts, hashes_required));
            }
            _ => panic!("Invalid spring"),
        }
    }
    // println!("Total: {}", total)
    total
}

#[allow(dead_code, unused_variables)]
fn part_one() {
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
        // let total = get_valid_springs(springs.clone(), counts);
        println!("{:?} {:?}", new_springs, new_counts);
        let total = get_valid_springs(new_springs, &new_counts);
        println!("Total: {}", total);
        all_total += total;
    }
    println!("All Total: {}", all_total);
}

#[allow(dead_code, unused_variables)]
fn part_two() {
    println!("Part 2");
    let data = load_data();
}

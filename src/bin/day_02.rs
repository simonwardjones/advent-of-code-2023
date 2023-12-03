const RAW_DATA: &str = include_str!("../../input/day_02.txt");

fn main() {
    part_one();
    part_two();
}

fn load_data() -> Vec<(u32, Vec<Vec<(u32, &'static str)>>)> {
    let data: Vec<(u32, Vec<Vec<(u32, &str)>>)> = RAW_DATA
        .lines()
        .map(|line| {
            let mut parts = line.split(":");
            let game = parts.next().unwrap();
            let subsets_raw = parts.next().unwrap();

            let id: u32 = game.split(" ").last().unwrap().parse().unwrap();
            let subsets: Vec<Vec<(u32, &str)>> = subsets_raw
                .trim()
                .split(";")
                .map(|subset| {
                    let cubes = subset
                        .trim()
                        .split(",")
                        .map(|cube| {
                            let mut parts = cube.trim().split(" ");
                            (
                                parts.next().unwrap().parse::<u32>().unwrap(),
                                parts.next().unwrap(),
                            )
                        })
                        .collect();
                    cubes
                })
                .collect();
            // println!("game: {:?}, subsets: {:?}", game, subsets);
            (id, subsets)
        })
        .collect();
    // println!("data: {:?}", data);
    data
}

fn part_one() {
    println!("Part 1");
    let data = load_data();

    let sum_valid_groups = data
        .iter()
        .filter_map(|(id, subsets)| {
            let mut valid = true;
            for subset in subsets {
                // 12 red cubes, 13 green cubes, and 14 blue cubes
                for (number, colour) in subset {
                    if *colour == "red" && *number > 12 {
                        valid = false;
                        break;
                    } else if *colour == "green" && *number > 13 {
                        valid = false;
                        break;
                    } else if *colour == "blue" && *number > 14 {
                        valid = false;
                        break;
                    }
                }
                if !valid {
                    break;
                }
            }
            if valid {
                Some(id)
            } else {
                None
            }
        })
        .sum::<u32>();
    println!("sum_valid_groups: {:?}", sum_valid_groups);
}

#[allow(dead_code, unused_variables)]
fn part_two() {
    println!("Part 2");
    let data = load_data();

    let power_max_groups = data
        .iter()
        .map(|(id, subsets)| {
            let mut maxes: (u32, u32, u32) = (0, 0, 0);
            for subset in subsets {
                // 12 red cubes, 13 green cubes, and 14 blue cubes
                for (number, colour) in subset {
                    if *colour == "red" && *number > maxes.0 {
                        maxes.0 = *number;
                    } else if *colour == "green" && *number > maxes.1 {
                        maxes.1 = *number;
                    } else if *colour == "blue" && *number > maxes.2 {
                        maxes.2 = *number;
                    }
                }
            }
            maxes.0 * maxes.1 * maxes.2
        })
        .sum::<u32>();
    println!("power_max_groups: {:?}", power_max_groups)
}

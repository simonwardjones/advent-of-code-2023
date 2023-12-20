use std::collections::{HashMap, VecDeque};

const RAW_DATA: &str = include_str!("../../input/day_20.txt");

fn main() {
    part_one();
    part_two();
}

fn load_data() -> (
    HashMap<String, Vec<String>>,
    HashMap<String, bool>,
    HashMap<String, HashMap<String, bool>>,
) {
    let mut down_streams = HashMap::<String, Vec<String>>::new();
    let mut flip_flops = HashMap::<String, bool>::new();
    let mut conjunctions = HashMap::<String, HashMap<String, bool>>::new();
    for line in RAW_DATA.lines() {
        let (module, destinations) = line.split_once(" -> ").unwrap();
        let module_name = module[1..].to_string();
        let destinations = destinations
            .split(", ")
            .map(|d| d.to_string())
            .collect::<Vec<String>>();
        down_streams.insert(module_name.clone(), destinations);
        match &module[..1] {
            "%" => {
                flip_flops.insert(module_name.clone(), false);
            }
            "&" => {
                conjunctions.insert(module_name.clone(), HashMap::new());
            }
            _ => {
                println!("skipping broadcast");
            }
        }
    }
    for (from, tos) in down_streams.iter() {
        for to in tos {
            if conjunctions.contains_key(to) {
                conjunctions
                    .get_mut(to)
                    .unwrap()
                    .insert(from.clone(), false);
            }
        }
    }
    (down_streams, flip_flops, conjunctions)
}

#[allow(dead_code, unused_variables)]
fn part_one() {
    println!("Part 1");
    let (down_streams, mut flip_flops, mut conjunctions) = load_data();
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    let mut loops = 0;
    let mut to_process = VecDeque::new();
    loop {
        if loops == 1000 {
            break;
        }
        loops += 1;
        to_process.push_back(("roadcaster".to_string(), false, "".to_string()));
        while let Some((module, pulse, from)) = to_process.pop_front() {
            if pulse {
                high_pulses += 1;
            } else {
                low_pulses += 1;
            }
            if &module == "roadcaster" {
                for downstream in down_streams.get(&module).unwrap() {
                    // println!("{} -low-> {}", module, downstream);
                    to_process.push_back((downstream.clone(), pulse, module.clone()));
                }
            } else if flip_flops.contains_key(&module) {
                if pulse {
                    continue;
                } else {
                    *flip_flops.get_mut(&module).unwrap() = !flip_flops[&module];
                    for downstream in down_streams.get(&module).unwrap() {
                        let new_pulse = flip_flops[&module];
                        let display = if new_pulse { "high" } else { "low" };
                        // println!("{} -{}-> {}", module, display, downstream);
                        to_process.push_back((downstream.clone(), new_pulse, module.clone()));
                    }
                }
            } else if conjunctions.contains_key(&module) {
                *conjunctions
                    .get_mut(&module)
                    .unwrap()
                    .get_mut(&from)
                    .unwrap() = pulse; // update
                let conjunction_values = conjunctions.get(&module).unwrap().values();
                // println!("conjunction_values: {:?}", conjunction_values);
                let all_true = !conjunctions.get(&module).unwrap().values().all(|v| *v);
                for downstream in down_streams.get(&module).unwrap() {
                    let display = if all_true { "high" } else { "low" };
                    // println!("{} -{}-> {}", module, display, downstream);
                    to_process.push_back((downstream.clone(), all_true, module.clone()));
                }
            }
        }
    }
    println!("loops: {},  pulses: {}, {}", loops, high_pulses, low_pulses);
    let meta_loops: i128 = 1000 / loops;
    let total = meta_loops.pow(2) * high_pulses * low_pulses;
    println!("total: {}", total);
}

#[allow(dead_code, unused_variables)]
fn part_two() {
    println!("Part 2");
    let (down_streams, mut flip_flops, mut conjunctions) = load_data();
    let mut loops = 0;
    let mut to_process = VecDeque::new();
    let up_streams = down_streams
        .iter()
        .map(|(k, v)| {
            v.iter()
                .map(|v| (v.clone(), k.clone()))
                .collect::<Vec<(String, String)>>()
        })
        .flatten()
        .collect::<Vec<(String, String)>>();
    let rx_up_stream = up_streams
        .iter()
        .filter(|(k, _)| k == "rx")
        .map(|(_, v)| v.clone())
        .collect::<Vec<String>>()[0]
        .clone();
    let mut rx_up_up_streams = up_streams
        .iter()
        .filter(|(k, _)| k == &rx_up_stream)
        .map(|(_, v)| (v.clone(), -1))
        .collect::<HashMap<String, i32>>();
    let mut found = 0;
    loop {
        loops += 1;
        if found == 4 {
            break;
        }
        to_process.push_back(("roadcaster".to_string(), false, "".to_string()));
        while let Some((module, pulse, from)) = to_process.pop_front() {
            if &module == "roadcaster" {
                for downstream in down_streams.get(&module).unwrap() {
                    // println!("{} -low-> {}", module, downstream);
                    to_process.push_back((downstream.clone(), pulse, module.clone()));
                }
            } else if flip_flops.contains_key(&module) {
                if pulse {
                    continue;
                } else {
                    *flip_flops.get_mut(&module).unwrap() = !flip_flops[&module];
                    for downstream in down_streams.get(&module).unwrap() {
                        let new_pulse = flip_flops[&module];
                        let display = if new_pulse { "high" } else { "low" };
                        // println!("{} -{}-> {}", module, display, downstream);
                        to_process.push_back((downstream.clone(), new_pulse, module.clone()));
                    }
                }
            } else if conjunctions.contains_key(&module) {
                if pulse && module == rx_up_stream {
                    let rx_up_up_stream = rx_up_up_streams.get_mut(&from).unwrap();
                    if *rx_up_up_stream == -1 {
                        println!("{} loops: {}, ", &from, loops,);
                        *rx_up_up_stream = loops;
                        found += 1;
                    };
                }
                *conjunctions
                    .get_mut(&module)
                    .unwrap()
                    .get_mut(&from)
                    .unwrap() = pulse; // update
                let conjunction_values = conjunctions.get(&module).unwrap().values();
                // println!("conjunction_values: {:?}", conjunction_values);
                let all_true = !conjunctions.get(&module).unwrap().values().all(|v| *v);
                for downstream in down_streams.get(&module).unwrap() {
                    let display = if all_true { "high" } else { "low" };
                    // println!("{} -{}-> {}", module, display, downstream);
                    to_process.push_back((downstream.clone(), all_true, module.clone()));
                }
            }
        }
    }
    println!("loops: {}", loops);
    let values = &rx_up_up_streams
        .values()
        .map(|v| *v as i128)
        .collect::<Vec<i128>>();
    println!("values: {:?}", values);
    let total = lcm(values);
    println!("total: {}", total);
}

pub fn lcm(numbers: &[i128]) -> i128 {
    if numbers.len() == 1 {
        return numbers[0];
    }
    let a = numbers[0];
    let b = lcm(&numbers[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: i128, b: i128) -> i128 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

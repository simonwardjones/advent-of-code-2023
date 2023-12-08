use std::collections::HashMap;

const RAW_DATA: &str = include_str!("../../input/day_08.txt");

fn main() {
    // part_one();
    part_two();
}

fn load_data() -> (
    &'static str,
    Vec<(String, (String, String))>,
    HashMap<String, (String, String)>,
) {
    let (instructions, nodes) = RAW_DATA.split_once("\n\n").unwrap();
    let nodes: Vec<(String, (String, String))> = nodes
        .lines()
        .map(|node| {
            let (from, to_pair) = node.split_once(" = ").expect("Invalid node");
            let to_pair = to_pair
                .strip_prefix("(")
                .expect("must have (")
                .strip_suffix(")")
                .expect("must have )")
                .split_once(", ")
                .unwrap();
            let to_pair = (to_pair.0.to_string(), to_pair.1.to_string());
            (from.to_string(), to_pair)
        })
        .collect();
    let node_map: HashMap<String, (String, String)> = nodes.iter().cloned().collect();
    (instructions, nodes, node_map)
}

#[allow(dead_code, unused_variables)]
fn part_one() {
    println!("Part 1");
    let (instructions, nodes, node_map) = load_data();
    // println!("instructions, {instructions}, nodes: {nodes:?}, node_map: {node_map:?}");
    let mut current_node = "AAA".to_string();
    let mut steps = 0;
    let mut directions_iter = instructions.chars().cycle();
    let mut direction: char;
    let mut i = 0;
    while current_node != "ZZZ" {
        i = i + 1;
        direction = directions_iter.next().expect("no directions");
        let next_nodes = node_map.get(&current_node).expect("no next node").clone();
        steps += 1;
        let next_node = if direction == 'L' {
            next_nodes.0
        } else {
            next_nodes.1
        };
        // println!("i: {i}, current_node: {current_node},direction: {direction}, next_node: {next_node:?}");
        current_node = next_node;
    }
    println!("steps: {steps}");
}

#[allow(dead_code, unused_variables)]
fn part_two() {
    println!("Part 2");
    let (instructions, nodes, node_map) = load_data();
    let mut current_nodes = nodes
        .iter()
        .filter(|node| node.0.ends_with("A"))
        .map(|node| node.0.clone())
        .collect::<Vec<_>>();
    let mut directions_iter = instructions.chars().cycle();
    let mut i = 0;
    let mut steps_to_z = current_nodes.iter().map(|_| 0).collect::<Vec<_>>();
    while !current_nodes
        .iter()
        .map(|node| node.ends_with("Z"))
        .all(|x| x)
    // && i < 1000
    {
        let direction = directions_iter.next().expect("no directions");
        i += 1;
        let next_nodes = current_nodes
            .iter()
            .enumerate()
            .map(|(j, node)| {
                let next_nodes = node_map.get(node).expect("no next node").clone();
                let next_node;
                if direction == 'L' {
                    next_node = next_nodes.0;
                } else {
                    next_node = next_nodes.1;
                }
                if next_node.ends_with("Z") {
                    if steps_to_z[j] == 0 {
                        steps_to_z[j] = i;
                    }
                }
                next_node
            })
            .collect::<Vec<_>>();
        if steps_to_z.iter().all(|x| *x > 0) {
            let lowest_common_multiple = lcm(&steps_to_z);
            println!("steps_to_z: {steps_to_z:?} lowest_common_multiple: {lowest_common_multiple}");
            return;
        }

        current_nodes = next_nodes;
    }
    println!("steps: {i:?}");
}

pub fn lcm(numbers: &[usize]) -> usize {
    if numbers.len() == 1 {
        return numbers[0];
    }
    let a = numbers[0];
    let b = lcm(&numbers[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

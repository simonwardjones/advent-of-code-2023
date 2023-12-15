const RAW_DATA: &str = include_str!("../../input/day_15.txt");

fn main() {
    part_one();
    part_two();
}

fn load_data() -> &'static [u8] {
    let data: &[u8] = RAW_DATA.trim().as_bytes();
    data
}

fn hash(x: &'static str) -> i32 {
    x.as_bytes()
        .to_owned()
        .iter()
        .fold(0, |acc, x| (((acc + *x as i32) * 17) % 256))
}

fn print_boxes(boxes: &Vec<Vec<(&str, i32)>>) {
    boxes.iter().enumerate().for_each(|(i, box_)| {
        if box_.len() > 0 {
            println!("Box {}: {:?}", i, box_);
        }
    })
}

#[allow(dead_code, unused_variables)]
fn part_one() {
    println!("Part 1");
    let mut boxes: Vec<Vec<(&str, i32)>> = Vec::new();
    (0..256).for_each(|_| boxes.push(Vec::new()));
    // println!("{:?}", boxes);
    RAW_DATA.trim().split(',').for_each(|step| {
        match step {
            label if label.contains('=') => {
                let (label, focal_length) = label.split_once('=').expect("Invalid = input");
                let focal_length: i32 = focal_length.parse::<i32>().unwrap();
                let box_ = &mut boxes[hash(label) as usize];
                let pos = box_.iter().map(|(l, _)| *l).position(|x| x == label);
                if let Some(pos) = pos {
                    box_[pos] = (label, focal_length);
                } else {
                    box_.push((label, focal_length));
                }
                // println!("label {label}, with = {focal_length} in box {box_number}",);
            }
            label if label.contains('-') => {
                let label = label.strip_suffix('-').unwrap();
                let box_number = hash(label);
                println!("label {label}, with - in box {box_number}  ");
                let box_ = &mut boxes[hash(label) as usize];
                if let Some(pos) = box_.iter().position(|(x, f)| *x == label) {
                    box_.remove(pos);
                }
                // hash(label)
            }
            _ => panic!("Unknown step {}", step),
        }
        // println!("After {step}");
        // print_boxes(&boxes);
        // println!();
    });
    print_boxes(&boxes);
    let total: i32 = boxes
        .iter()
        .enumerate()
        .map(|(box_id, box_)| {
            box_.iter().enumerate().fold(0, |acc, (i, (l, f))| {
                acc + (box_id as i32 + 1) * (i as i32 + 1) * f
            })
        })
        .sum();
    println!("Total: {:?}", total);
}

#[allow(dead_code, unused_variables)]
fn part_two() {
    println!("Part 2");
    let data = load_data();
}

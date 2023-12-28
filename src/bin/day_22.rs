use std::collections::{HashMap, HashSet};

const RAW_DATA: &str = include_str!("../../input/day_22.txt");

fn main() {
    part_one();
    part_two();
}

type Block = (Vec<i32>, Vec<i32>);

fn load_data() -> Vec<Block> {
    let mut data: Vec<Block> = RAW_DATA
        .lines()
        .map(|line| {
            let (from, to) = line.split_once("~").unwrap();
            let from: Vec<_> = from.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
            let to: Vec<_> = to.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
            (from, to)
        })
        .collect();
    // sort data by third element in first tuple
    data.sort_by(|a, b| a.0[2].cmp(&b.0[2]));
    data
}

fn is_below(a: &Block, b: &Block) -> bool {
    // if a is below b
    let (from, to) = a;
    let (from2, to2) = b;
    let next_z = from2[2] - 1 == to[2];
    let x_intercept = from[0] <= to2[0] && to[0] >= from2[0];
    let y_intercept = from[1] <= to2[1] && to[1] >= from2[1];
    // println!(
    //     "next_z => {:?}, x_intercept => {:?}, y_intercept => {:?}",
    //     next_z, x_intercept, y_intercept
    // );
    next_z && x_intercept && y_intercept
}

#[allow(dead_code, unused_variables)]
fn part_one() {
    println!("Part 1");
    let mut data = load_data();
    // println!("{:?}", data);
    let mut supports: HashSet<(usize, usize)> = HashSet::new();
    // for each block in data move down as much as possible while z > 0
    let n = data.len();
    let mut i = 0;
    while i < n {
        while data[i].0[2] > 0 && !data[..i].iter().any(|x| is_below(x, &data[i])) {
            data[i].0[2] -= 1;
            data[i].1[2] -= 1;
        }
        i += 1;
    }
    // println!("{:?}", data);
    for i in 0..n {
        for j in (i + 1)..n {
            if is_below(&data[i], &data[j]) {
                supports.insert((i, j));
            }
        }
    }
    println!("supports =>{:?}", supports);
    let support_map: HashMap<usize, Vec<usize>> =
        supports.iter().fold(HashMap::new(), |mut acc, (i, j)| {
            acc.entry(*i).or_insert(vec![]).push(*j);
            acc
        });
    //flip supports
    let supported_by = supports.iter().fold(HashMap::new(), |mut acc, (i, j)| {
        acc.entry(*j).or_insert(vec![]).push(*i);
        acc
    });
    // println!("support_map => {:?}", support_map);
    // println!("supported_by => {:?}", supported_by);
    let reducible: Vec<_> = (0..n)
        .filter(|b| {
            if !support_map.contains_key(&b) {
                true
            } else {
                support_map[&b].iter().all(|x| supported_by[x].len() > 1)
            }
        })
        .collect();
    // println!("reducible => {:?}", reducible);
    // println!()
    let deps: Vec<_> = (0..n)
        .map(|i| {
            if !support_map.contains_key(&i) {
                0
            } else {
                let mut seen_so_far = HashSet::new();
                seen_so_far.insert(i);
                let mut children: Vec<usize> = support_map[&i]
                    .iter()
                    .filter(|x| {
                        supported_by[&x]
                            .iter()
                            .filter(|x| !seen_so_far.contains(x))
                            .collect::<Vec<_>>()
                            .len()
                            == 0
                    })
                    .map(|x| *x)
                    .collect();
                println!("children => {:?}", children);
                seen_so_far.extend(children.iter());
                let mut total = children.len();
                while children.len() > 0 {
                    let new = children
                        .iter()
                        .flat_map(|child| {
                            support_map
                                .get(&child)
                                .unwrap_or(&Vec::new())
                                .iter()
                                .filter(|x| {
                                    supported_by[&x]
                                        .iter()
                                        .filter(|x| !seen_so_far.contains(x))
                                        .collect::<Vec<_>>()
                                        .len()
                                        == 0
                                })
                                .map(|x| *x)
                                .collect::<HashSet<usize>>()
                        })
                        .collect::<HashSet<usize>>();

                    seen_so_far.extend(new.iter());
                    println!("new children => {:?}", new);
                    children = new.iter().map(|x| *x).collect();
                    total += children.len();
                }
                total
            }
        })
        .collect();
    // println!("deps => {:?}", deps);
    println!("total => {:?}", deps.iter().sum::<usize>());
}

#[allow(dead_code, unused_variables)]
fn part_two() {
    println!("Part 2");
    let data = load_data();
}

use std::collections::HashSet;

const RAW_DATA: &str = include_str!("../../input/day_16.txt");

fn main() {
    part_one();
    part_two();
}

fn load_data() -> Vec<Vec<char>> {
    let data: Vec<Vec<char>> = RAW_DATA
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    data
}

fn part_one() {
    println!("Part 1");
    let velocity: (i32, i32) = (0, 1);
    let position: (i32, i32) = (0, 0);
    let energy = energy(&velocity, &position);
    println!("Energy = {energy}")
}

#[allow(dead_code, unused_variables)]
fn energy(velocity: &(i32, i32), position: &(i32, i32)) -> i32 {
    let data = load_data();
    let mut to_process: Vec<((i32, i32), (i32, i32))> = Vec::from([(*velocity, *position)]);
    let mut seen: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
    let n = data.len() as i32;
    let m = data[0].len() as i32;
    while to_process.len() > 0 {
        let (velocity, position) = to_process.pop().expect("No points");
        // println!(
        //     "Processing velocity = {:?}, position = {:?}",
        //     velocity, position
        // );
        seen.insert((velocity, position));
        let square = data[position.0 as usize][position.1 as usize];
        let next_points = match (square, velocity, position) {
            ('.', v, p) => vec![(v, (p.0 + v.0, p.1 + v.1))],
            ('\\', (1, 0), p) => vec![((0, 1), (p.0, p.1 + 1))],
            ('\\', (0, 1), p) => vec![((1, 0), (p.0 + 1, p.1))],
            ('\\', (-1, 0), p) => vec![((0, -1), (p.0, p.1 - 1))],
            ('\\', (0, -1), p) => vec![((-1, 0), (p.0 - 1, p.1))],
            ('/', (1, 0), p) => vec![((0, -1), (p.0, p.1 - 1))],
            ('/', (0, 1), p) => vec![((-1, 0), (p.0 - 1, p.1))],
            ('/', (-1, 0), p) => vec![((0, 1), (p.0, p.1 + 1))],
            ('/', (0, -1), p) => vec![((1, 0), (p.0 + 1, p.1))],

            ('-', (a, b), p) if b == 0 => vec![((0, 1), (p.0, p.1 + 1)), ((0, -1), (p.0, p.1 - 1))],
            ('-', v, p) if v.0 == 0 => vec![(v, (p.0 + v.0, p.1 + v.1))],
            ('|', v, p) if v.1 == 0 => vec![(v, (p.0 + v.0, p.1 + v.1))],
            ('|', v, p) if v.0 == 0 => vec![((1, 0), (p.0 + 1, p.1)), ((-1, 0), (p.0 - 1, p.1))],

            _ => panic!("Unrecognised pos/vel"),
        };
        for v_p in next_points {
            match v_p {
                (_, (i, j)) if (i < 0 || i >= n || j < 0 || j >= m) => {
                    // println!("Off grid ({i}, {j})");
                    continue;
                }
                s if seen.contains(&s) => {
                    // println!("seen {:?}", s.1);
                    continue;
                }
                _ => to_process.push(v_p),
            }
        }
    }
    // println!("{data:?}");
    // println!("{:?}", seen.iter().map(|(v, p)| *p).collect::<HashSet<(i32, i32)>>().len());
    seen.iter()
        .map(|(v, p)| *p)
        .collect::<HashSet<(i32, i32)>>()
        .len() as i32
}

#[allow(dead_code, unused_variables)]
fn part_two() {
    println!("Part 2");
    let data = load_data();
    let velocity: (i32, i32) = (0, 1);
    let n = data.len() as i32;
    let m = data[0].len() as i32;
    let left: Vec<((i32, i32), (i32, i32))> = (0..n).map(|i| ((0, 1), (i, 0))).collect();
    let right: Vec<((i32, i32), (i32, i32))> = (0..n).map(|i| ((0, -1), (i, m - 1))).collect();
    let top: Vec<((i32, i32), (i32, i32))> = (0..m).map(|i| ((1, 0), (0, i))).collect();
    let bottom: Vec<((i32, i32), (i32, i32))> = (0..m).map(|i| ((-1, 0), (n - 1, i))).collect();
    let energies : Vec<_>= left
        .iter()
        .chain(top.iter())
        .chain(right.iter())
        .chain(bottom.iter())
        .map(|(v, p)| energy(v, p))
        .collect();
    println!("energies = {energies:?}");
    println!("energies = {:?}", energies.iter().reduce(|a, b| a.max(b)).expect("No max"));
}

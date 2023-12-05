use std::collections::HashSet;

const RAW_DATA: &str = include_str!("../../input/day_04.txt");

fn main() {
    part_one();
    part_two();
}

fn part_one() -> i32 {
    println!("Part 1");
    let total_win_value = RAW_DATA
        .lines()
        .filter_map(|line| {
            let (winners, my_numbers) = line.split(":").nth(1).unwrap().split_once(" | ").unwrap();
            let winners: Vec<i32> = winners
                .split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect();
            let my_numbers: Vec<i32> = my_numbers
                .split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect();
            let winners_set: HashSet<i32> = winners.into_iter().collect();
            let my_numbers_set: HashSet<i32> = my_numbers.into_iter().collect();
            let winners_count = winners_set.intersection(&my_numbers_set).count();
            if winners_count > 0 {
                Some((2 as i32).pow((winners_count - 1) as u32))
            } else {
                None
            }
        })
        .sum();
    println!("total_win_value: {}", total_win_value);
    total_win_value
}

#[allow(dead_code, unused_variables)]
fn part_two() {
    println!("Part 2");
    let cards: Vec<(HashSet<i32>, HashSet<i32>)> = RAW_DATA
        .lines()
        .map(|line| {
            let (winners, my_numbers) = line.split(":").nth(1).unwrap().split_once(" | ").unwrap();
            let winners: HashSet<i32> = winners
                .split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect();
            let my_numbers: HashSet<i32> = my_numbers
                .split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect();
            (winners, my_numbers)
        })
        .collect();
    println!("cards: {:?}", cards);

    let mut cache = vec![None; cards.len()];
    let total_win_value = cards
        .iter()
        .enumerate()
        .map(|(id, _)| card_win(id, &cards, &mut cache))
        .sum::<i32>();
    println!("cache: {:?}", cache);
    println!("total_win_value: {}", total_win_value);
}

fn card_win(
    id: usize,
    cards: &Vec<(HashSet<i32>, HashSet<i32>)>,
    cache: &mut Vec<Option<i32>>,
) -> i32 {
    if let Some(value) = cache[id as usize] {
        return value;
    }
    let (winners, my_numbers) = &cards[id];
    let winners_count = winners.intersection(&my_numbers).count();
    if winners_count == 0 {
        cache[id as usize] = Some(1);
        return 1;
    }
    println!(
        "id: {}, winners_count: {}, sum {}",
        id,
        winners_count,
        winners_count + id + 1
    );
    let card_win = ((id + 1)..(winners_count + id + 1))
        .map(|i| card_win(i, cards, cache))
        .sum::<i32>()
        + 1;
    cache[id as usize] = Some(card_win);
    card_win
}

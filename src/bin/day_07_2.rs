use std::{cmp::Ordering, collections::HashMap};

const RAW_DATA: &str = include_str!("../../input/day_07.txt");

fn main() {
    part_two();
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Hash, Eq, Copy, Clone)]
pub enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_str(s: char) -> Option<Card> {
        match s {
            'J' => Some(Card::Joker),
            '2' => Some(Card::Two),
            '3' => Some(Card::Three),
            '4' => Some(Card::Four),
            '5' => Some(Card::Five),
            '6' => Some(Card::Six),
            '7' => Some(Card::Seven),
            '8' => Some(Card::Eight),
            '9' => Some(Card::Nine),
            'T' => Some(Card::Ten),
            'Q' => Some(Card::Queen),
            'K' => Some(Card::King),
            'A' => Some(Card::Ace),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Copy, Clone)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    cards: Vec<Card>,
    card_counts: HashMap<Card, i32>,
    hand_type: HandType,
}

impl Hand {
    // from an iterator of cards, create a hand
    fn from_cards_iter<I>(cards: I) -> Hand
    where
        I: Iterator<Item = Card> + Clone,
    {
        let mut card_counts = HashMap::new();
        for card in cards.clone().into_iter() {
            let count = card_counts.entry(card).or_insert(0);
            *count += 1;
        }
        let cards_vec = cards.collect::<Vec<_>>();
        if let Some((_joker, joker_count)) = card_counts.remove_entry(&Card::Joker) {
            let highest = card_counts.iter().max_by(|a, b| {
                if a.1.cmp(&b.1) == Ordering::Equal {
                    a.0.cmp(&b.0)
                } else {
                    a.1.cmp(&b.1)
                }
            });
            if let Some(highest) = highest {
                card_counts
                    .entry(*highest.0)
                    .and_modify(|e| *e += joker_count);
            } else {
                // all cards are jokers
                card_counts.insert(Card::Ace, 5);
            }
        }
        let hand_type = Hand::get_hand_type(&card_counts);
        Hand {
            cards: cards_vec,
            card_counts,
            hand_type,
        }
    }

    fn get_hand_type(card_counts: &HashMap<Card, i32>) -> HandType {
        let max_len = (card_counts.values().max().unwrap(), card_counts.len());
        match max_len {
            (1, 5) => HandType::HighCard,
            (2, 4) => HandType::OnePair,
            (2, 3) => HandType::TwoPairs,
            (3, 3) => HandType::ThreeOfAKind,
            (3, 2) => HandType::FullHouse,
            (4, 2) => HandType::FourOfAKind,
            (5, 1) => HandType::FiveOfAKind,
            _ => {
                println!("card_counts = {:?}", card_counts);
                panic!("Unknown hand type")
            }
        }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp<'a>(&'a self, other: &'a Self) -> Option<Ordering> {
        if self.hand_type == other.hand_type {
            for (card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                if *card != *other_card {
                    return card.partial_cmp(other_card);
                }
            }
            return Some(Ordering::Equal);
        }
        return self.hand_type.partial_cmp(&other.hand_type);
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn load_data() -> Vec<(Hand, i32)> {
    RAW_DATA
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect::<Vec<Vec<_>>>()
        .iter()
        .map(|line| {
            (
                Hand::from_cards_iter(line[0].chars().filter_map(|c| Card::from_str(c))),
                line[1].parse::<i32>().unwrap(),
            )
        })
        .collect()
}

#[allow(dead_code, unused_variables)]
fn part_two() {
    println!("Part 1");
    let mut hands_bids = load_data();
    hands_bids.sort();
    let result: i32 = hands_bids
        .iter()
        .enumerate()
        .map(|(i, (hand, bid))| (i + 1) as i32 * bid)
        .sum();
    // println!("hands_bids= {:?}", hands_bids);
    println!("results= {:?}", result);
}

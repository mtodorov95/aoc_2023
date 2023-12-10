use std::{cmp::Ordering, collections::HashMap, default, str::FromStr};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Card {
    pub label: char,
}

impl Card {
    const LABELS: [char; 13] = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];

    pub fn strength(&self) -> usize {
        Card::LABELS.len()
            - Card::LABELS
                .iter()
                .position(|&curr| curr == self.label)
                .unwrap_or_default()
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.strength().cmp(&other.strength())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        Card { label: value }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
enum HandType {
    #[default]
    Five = 7,
    Four = 6,
    FullHouse = 5,
    Three = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl HandType {
    pub const fn strength(&self) -> usize {
        *self as usize
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.strength().cmp(&other.strength())
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Hand {
    cards: [Card; 5],
    bid: usize,
}

impl FromStr for Hand {
    type Err = &'static str;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut input = string.split_ascii_whitespace();
        let result = Hand {
            cards: array_init::from_iter(input.next().unwrap_or_default().chars().map(Card::from))
                .unwrap_or_default(),
            bid: input
                .next()
                .unwrap_or_default()
                .trim()
                .parse()
                .unwrap_or_default(),
        };
        Ok(result)
    }
}

impl Hand {
    pub fn hand_type(&self) -> HandType {
        let mut counts: HashMap<char, usize> = HashMap::new();

        for card in &self.cards {
            if let Some(label_count) = counts.get_mut(&card.label) {
                *label_count += 1;
            } else {
                counts.insert(card.label, 1);
            }
        }
        let keys = counts.keys();
        if keys.len() == 1 {
            return HandType::Five;
        }
        if keys.len() == 2 && counts.iter().any(|(_, &val)| val == 4) {
            return HandType::Four;
        }
        if keys.len() == 2 && counts.iter().any(|(_, &val)| val == 3) {
            return HandType::FullHouse;
        }
        if keys.len() == 3 && counts.iter().any(|(_, &val)| val == 3) {
            return HandType::Three;
        }
        if keys.len() == 3 && counts.iter().any(|(_, &val)| val == 2) {
            return HandType::TwoPair;
        }
        if keys.len() == 5 {
            return HandType::HighCard;
        }
        HandType::OnePair
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type() != other.hand_type() {
            return self
                .hand_type()
                .strength()
                .cmp(&other.hand_type().strength());
        }

        for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
            if self_card != other_card {
                return self_card.cmp(other_card);
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let file = include_str!("../input");
    println!("Part 1: {}", part_one(file));
    println!("Part 2: {}", part_two(file));
}

fn part_one(file: &str) -> usize {
    let mut hands: Vec<Hand> = file
        .lines()
        .map(|line| Hand::from_str(line).unwrap_or_default())
        .collect();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| {
            let rank = idx + 1;
            hand.bid * rank
        })
        .sum()
}

fn part_two(file: &str) -> usize {
    let mut hands: Vec<Hand> = file
        .lines()
        .map(|line| Hand::from_str(line).unwrap_or_default())
        .collect();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| {
            let rank = idx + 1;
            hand.bid * rank
        })
        .sum()
}

#[cfg(test)]
mod day_7_tests {
    use crate::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_one(include_str!("../example")), 6440);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_two(include_str!("../example")), 5905);
    }
}

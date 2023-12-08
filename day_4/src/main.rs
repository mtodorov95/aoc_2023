use std::collections::BTreeMap;

fn main() {
    let file = include_str!("../input").to_owned();
    println!("Part 1: {}", part_one(String::from(&file)));
    println!("Part 2: {}", part_two(file));
}

#[derive(Debug)]
struct Card {
    id: u32,
    own: Vec<u32>,
    winning: Vec<u32>,
}

impl Card {
    fn get_points(&self) -> u32 {
        let hits = self.get_num_of_hits();
        match hits {
            0 => 0,
            _ => 1 * (2 as u32).pow(hits - 1),
        }
    }

    fn get_num_of_hits(&self) -> u32 {
        let mut hits = 0;
        for own_num in self.own.iter() {
            if self.winning.contains(own_num) {
                hits += 1;
            }
        }
        hits
    }
}

pub fn part_one(file: String) -> u32 {
    let cards: Vec<Card> = file
        .lines()
        .map(|line| {
            let id = line
                .split(":")
                .next()
                .unwrap()
                .strip_prefix("Card ")
                .unwrap()
                .trim()
                .parse::<u32>()
                .expect("Failed to parse u32");
            let winning = line
                .split(":")
                .last()
                .unwrap()
                .split("|")
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|n| n.trim().parse::<u32>().expect("Failed to parse u32"))
                .collect();
            let own = line
                .split(":")
                .last()
                .unwrap()
                .split("|")
                .last()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|n| n.trim().parse::<u32>().expect("Failed to parse u32"))
                .collect();

            Card { id, own, winning }
        })
        .collect();
    cards.iter().map(|c| c.get_points()).sum::<u32>()
}

pub fn part_two(file: String) -> u32 {
    let mut map: BTreeMap<u32, u32> = BTreeMap::new();
    let cards: Vec<Card> = file
        .lines()
        .map(|line| {
            let id = line
                .split(":")
                .next()
                .unwrap()
                .strip_prefix("Card ")
                .unwrap()
                .trim()
                .parse::<u32>()
                .expect("Failed to parse u32");
            let winning = line
                .split(":")
                .last()
                .unwrap()
                .split("|")
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|n| n.trim().parse::<u32>().expect("Failed to parse u32"))
                .collect();
            let own = line
                .split(":")
                .last()
                .unwrap()
                .split("|")
                .last()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|n| n.trim().parse::<u32>().expect("Failed to parse u32"))
                .collect();

            Card { id, own, winning }
        })
        .collect();

    for card in cards.iter() {
        if let Some(v) = map.get(&card.id) {
            map.insert(card.id, v + 1);
        } else {
            map.insert(card.id, 1);
        }

        let t = map.get(&card.id).unwrap();
        for _ in 0..*t {
            handle_card(card, &mut map);
        }
    }
    map.values().sum()
}

fn handle_card(card: &Card, map: &mut BTreeMap<u32, u32>) {
    let hits = card.get_num_of_hits();
    if hits == 0 {
        return;
    }

    for id in card.id + 1..=card.id + hits {
        if let Some(v) = map.get(&id) {
            map.insert(id, v + 1);
        } else {
            map.insert(id, 1);
        }
    }
}

#[cfg(test)]
mod day_2_tests {
    use crate::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_one(include_str!("../example").to_owned()), 13);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_two(include_str!("../example").to_owned()), 30);
    }
}

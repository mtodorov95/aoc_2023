use std::collections::HashMap;

const PREFIX: &str = "Game ";

fn main() {
    let file = include_str!("../input").to_owned();
    println!("Part 1: {}", part_one(String::from(&file)));
    println!("Part 2: {}", part_two(file));
}

pub fn part_one(file: String) -> u32 {
    let limits: HashMap<&str, u32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let possible_games = file
        .lines()
        .map(|line| {
            let id: u32 = line
                .split(":")
                .next()
                .unwrap_or("")
                .replace(PREFIX, "")
                .parse()
                .unwrap_or(0);
            let cubes: Vec<&str> = line.split(":").last().unwrap_or("").split(";").collect();
            for turn in cubes.iter() {
                let turn: Vec<bool> = turn
                    .split(",")
                    .map(|set| {
                        let set = set.trim();
                        let num: u32 = set
                            .split(" ")
                            .next()
                            .unwrap()
                            .parse()
                            .expect("Failed to parse u32");
                        let color = set.split(" ").last().unwrap_or("");

                        if let Some(limit) = limits.get(color) {
                            if limit < &num {
                                return false;
                            }
                        }
                        true
                    })
                    .collect();
                if turn.iter().any(|e| e == &false) {
                    return 0;
                }
            }
            id
        })
        .sum();

    possible_games
}

pub fn part_two(file: String) -> u32 {
    let power = file
        .lines()
        .map(|line| {
            let cubes: Vec<&str> = line.split(":").last().unwrap_or("").split(";").collect();
            let mut minimum: HashMap<&str, u32> =
                HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

            for turn in cubes.iter() {
                turn.split(",").for_each(|set| {
                    let set = set.trim();
                    let num: u32 = set
                        .split(" ")
                        .next()
                        .unwrap()
                        .parse()
                        .expect("Failed to parse u32");
                    let color = set.split(" ").last().unwrap_or("");

                    if let Some(min) = minimum.get(color) {
                        if &num > min {
                            minimum.insert(color, num);
                        }
                    }
                });
            }
            let tot = minimum.iter().fold(1, |acc, (_, num)| acc * num);
            return tot;
        })
        .sum();

    power
}

#[cfg(test)]
mod day_2_tests {
    use crate::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_one(include_str!("../example").to_owned()), 8);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_two(include_str!("../example").to_owned()), 2286);
    }
}

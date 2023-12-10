fn main() {
    let file = include_str!("../input");
    println!("Part 1: {}", part_one(file));
    println!("Part 2: {}", part_two(file));
}

#[derive(Debug)]
pub struct Race {
    time_limit: usize,
    record: usize,
}

impl Race {
    pub fn num_ways_to_beat(&self) -> usize {
        let mut total = 0;
        for held in 0..=self.time_limit {
            let distance = held * (self.time_limit - held);
            if distance > self.record {
                total += 1;
            }
        }
        total
    }
}

fn part_one(file: &str) -> usize {
    let lines: Vec<&str> = file
        .lines()
        .map(|line| {
            let res = line.split(":").last().unwrap_or_default().trim();
            res
        })
        .collect();
    let times: Vec<usize> = lines
        .first()
        .unwrap()
        .split_whitespace()
        .map(|t| t.trim().parse::<usize>().unwrap_or_default())
        .collect();
    let distances: Vec<usize> = lines
        .last()
        .unwrap()
        .split_whitespace()
        .map(|t| t.trim().parse::<usize>().unwrap_or_default())
        .collect();
    let races: Vec<Race> = times
        .iter()
        .enumerate()
        .map(|(i, time)| Race {
            time_limit: *time,
            record: distances[i],
        })
        .collect();
    let res = races
        .iter()
        .map(|r| r.num_ways_to_beat())
        .fold(1, |acc, curr| acc * curr);
    res
}

fn part_two(file: &str) -> usize {
    let lines: Vec<usize> = file
        .lines()
        .map(|line| {
            let res: String = line
                .split(":")
                .last()
                .unwrap_or_default()
                .trim()
                .split_whitespace()
                .fold(String::new(), |acc, curr| String::from(acc + curr));
            res
        })
        .map(|num| num.trim().parse::<usize>().unwrap_or_default())
        .collect();
    let race = Race {
        time_limit: lines[0],
        record: lines[1],
    };
    race.num_ways_to_beat()
}

#[cfg(test)]
mod day_6_tests {
    use crate::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_one(include_str!("../example")), 288);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_two(include_str!("../example")), 71503);
    }
}

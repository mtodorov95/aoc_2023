// Original arrangements solution thanks to: https://github.com/andypymont

use std::{collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Spring {
    Damaged,
    Working,
    Unknown,
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Working,
            '#' => Self::Damaged,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Default)]
struct Row {
    springs: Vec<Spring>,
    damaged_groups: Vec<usize>,
}

impl FromStr for Row {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split_whitespace();
        let springs: Vec<Spring> = line
            .next()
            .unwrap_or_default()
            .chars()
            .map(|c| Spring::from(c))
            .collect();
        let damaged_groups: Vec<usize> = line
            .next()
            .unwrap_or_default()
            .trim()
            .split(",")
            .map(|n| n.parse::<usize>().unwrap_or_default())
            .collect();
        Ok(Row {
            springs,
            damaged_groups,
        })
    }
}

impl Row {
    fn arrangements(&self) -> usize {
        let mut cache = HashMap::new();
        self.arrangements_for_section(&mut cache, 0, 0)
    }

    fn arrangements_for_section(
        &self,
        cache: &mut HashMap<(usize, usize), usize>,
        spring_ix: usize,
        group_ix: usize,
    ) -> usize {
        if let Some(value) = cache.get(&(spring_ix, group_ix)) {
            return *value;
        }

        let consume_group = self.damaged_groups.get(group_ix).map_or(0, |group_len| {
            if (spring_ix + group_len) > self.springs.len() {
                return 0;
            }

            if (0..*group_len)
                .any(|pos| self.springs.get(spring_ix + pos) == Some(&Spring::Working))
            {
                return 0;
            }

            if self.springs.get(spring_ix + group_len) == Some(&Spring::Damaged) {
                return 0;
            }

            self.arrangements_for_section(cache, spring_ix + group_len + 1, group_ix + 1)
        });

        let skip = match self.springs.get(spring_ix) {
            None => usize::from(group_ix >= self.damaged_groups.len()),
            Some(Spring::Damaged) => 0,
            Some(_) => self.arrangements_for_section(cache, spring_ix + 1, group_ix),
        };

        let arr = consume_group + skip;
        cache.insert((spring_ix, group_ix), arr);
        arr
    }

    fn unfold(&self) -> Self {
        let mut springs = vec![];
        let mut damaged_groups = vec![];

        for i in 0..5 {
            springs.extend(&self.springs);
            if i != 4 {
                springs.push(Spring::Unknown);
            }
            damaged_groups.extend(&self.damaged_groups);
        }

        Self {springs, damaged_groups}
    }
}

#[derive(Debug, Default)]
struct Field {
    rows: Vec<Row>,
}

impl FromStr for Field {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s
            .lines()
            .map(|line| Row::from_str(line).unwrap_or_default())
            .collect();
        Ok(Field { rows })
    }
}
fn main() {
    let file = include_str!("../input");
    println!("Part 1: {}", part_one(file));
    println!("Part 2: {}", part_two(file));
}

fn part_one(file: &str) -> usize {
    let field = Field::from_str(file).unwrap_or_default();
    field.rows.iter().map(|r| r.arrangements()).sum()
}

fn part_two(file: &str) -> usize {
    let field = Field::from_str(file).unwrap_or_default();
    field.rows.iter().map(|r| r.unfold().arrangements()).sum()
}

#[cfg(test)]
mod day_12_tests {
    use crate::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_one(include_str!("../example")), 21);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_two(include_str!("../example")), 525152);
    }
}

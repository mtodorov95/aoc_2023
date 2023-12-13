use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Galaxy {
    x: usize,
    y: usize,
}

#[derive(Debug, Default)]
struct Map {
    layout: Vec<String>,
}

impl FromStr for Map {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let layout: Vec<String> = s.lines().map(|line| String::from(line)).collect();
        let res = Map { layout };
        Ok(res)
    }
}

impl Map {
    fn expand(&mut self, degree: usize) -> Vec<(Galaxy, Galaxy)> {
        let mut empty_rows: Vec<usize> = vec![];
        for (i, row) in self.layout.iter().enumerate() {
            if row.chars().all(|c| c == '.') {
                empty_rows.push(i);
            }
        }

        let mut empty_cols = vec![];
        let mut empty = true;

        for col in 0..self.layout[0].len() {
            for row in self.layout.iter() {
                if row.chars().nth(col).unwrap() != '.' {
                    empty = false;
                }
            }
            if empty {
                empty_cols.push(col);
            }
            empty = true;
        }

        let mut galaxies = self.get_galxies();
        for galaxy in galaxies.iter_mut() {
            let mut x = 0;
            let mut y = 0;
            for row in empty_rows.iter() {
                if galaxy.y > *row {
                    y += degree;
                }
            }
            for col in empty_cols.iter() {
                if galaxy.x > *col {
                    x += degree;
                }
            }
            galaxy.x +=x;
            galaxy.y +=y;
        }
        self.get_pairs_from(galaxies)
    }
    fn get_pairs_from(&self, galaxies: Vec<Galaxy>) -> Vec<(Galaxy, Galaxy)> {
        let mut pairs: Vec<(Galaxy, Galaxy)> = vec![];
        for (idx, galaxy) in galaxies.iter().enumerate() {
            for i in idx + 1..galaxies.len() {
                pairs.push((*galaxy, galaxies[i]));
            }
        }
        pairs
    }

    fn get_galxies(&self) -> Vec<Galaxy> {
        let galaxies: Vec<Galaxy> = self
            .layout
            .iter()
            .enumerate()
            .flat_map(move |(row, line)| {
                line.chars().enumerate().filter_map(move |(col, c)| {
                    if c == '#' {
                        return Some(Galaxy { x: col, y: row });
                    }
                    None
                })
            })
            .collect();
        galaxies
    }
}

fn main() {
    let file = include_str!("../input");
    println!("Part 1: {}", part_one(file));
    println!("Part 2: {}", part_two(file));
}

fn part_one(file: &str) -> usize {
    let mut map = Map::from_str(file).unwrap_or_default();
    let mut sum = 0;

    let pairs = map.expand(1);
    pairs.iter().for_each(|(g1, g2)| {
        let dx = g2.x.abs_diff(g1.x);
        let dy = g2.y.abs_diff(g1.y);
        sum += dx + dy;
    });

    sum
}

fn part_two(file: &str) -> usize {
    let mut map = Map::from_str(file).unwrap_or_default();
    let mut sum = 0;

    let pairs = map.expand(999999);
    pairs.iter().for_each(|(g1, g2)| {
        let dx = g2.x.abs_diff(g1.x);
        let dy = g2.y.abs_diff(g1.y);
        sum += dx + dy;
    });

    sum
}

#[cfg(test)]
mod day_11_tests {
    use crate::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_one(include_str!("../example")), 374);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_two(include_str!("../example")), 2);
    }
}

use std::{ops::Range, str::FromStr};
use rayon::prelude::*;

#[derive(Default, Debug, PartialEq, Clone)]
pub struct RangeConverter {
    pub source_range: Range<usize>,
    pub destination_range: Range<usize>,
}
impl FromStr for RangeConverter {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .trim()
            .split_ascii_whitespace()
            .map(|string| {
                let res: usize = string.trim().parse().unwrap_or_default();
                res
            })
            .collect::<Vec<usize>>();
        let dest_range_start = *numbers.first().unwrap_or(&0);
        let src_range_start = *numbers.get(1).unwrap_or(&0);
        let range_size = *numbers.last().unwrap_or(&0);
        let result = RangeConverter {
            source_range: src_range_start..(src_range_start + range_size),
            destination_range: dest_range_start..(dest_range_start + range_size),
        };
        Ok(result)
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct CategoryConverter {
    pub name: String,
    pub range_converters: Vec<RangeConverter>,
}

impl FromStr for CategoryConverter {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.trim().lines();
        let name = lines.next().unwrap_or_default();
        let mut range_converters = vec![];
        for line in lines {
            range_converters.push(RangeConverter::from_str(line).unwrap_or_default());
        }

        let result = CategoryConverter {
            name: String::from(name),
            range_converters,
        };
        Ok(result)
    }
}

impl CategoryConverter {
    pub fn convert(&self, number: usize) -> usize {
        self.range_converters
            .iter()
            .find_map(|converter| {
                if converter.source_range.contains(&number) {
                    Some(
                        converter.destination_range.start + (number - converter.source_range.start),
                    )
                } else {
                    None
                }
            })
            .unwrap_or(number)
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Almanac {
    pub seeds: Vec<usize>,
    pub category_converters: Vec<CategoryConverter>,
}

impl FromStr for Almanac {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut categories = s.trim().split("\n\n");
        let seeds = categories
            .next()
            .unwrap_or_default()
            .strip_prefix("seeds: ")
            .unwrap_or_default()
            .split_ascii_whitespace()
            .map(|seed| {
                let res: usize = seed.trim().parse().unwrap_or_default();
                res
            })
            .collect::<Vec<usize>>();
        let category_converters = categories
            .map(|category| CategoryConverter::from_str(category).unwrap_or_default())
            .collect::<Vec<CategoryConverter>>();
        let res = Almanac {
            seeds,
            category_converters,
        };
        Ok(res)
    }
}

impl Almanac {
    pub fn min_location(&self) -> usize {
        self.seeds
            .par_iter()
            .map(|&seed| {
                self.category_converters
                    .iter()
                    .fold(seed, |acc, converter| converter.convert(acc))
            })
            .min()
            .unwrap_or_default()
    }

    pub fn set_seeds_as_ranges(&mut self) {
        self.seeds = self
            .seeds
            .par_chunks(2)
            .flat_map(|chunk| {
                if let [start, size] = chunk {
                    let start = *start;
                    let size = *size;
                    let range = start..(start + size);
                    range.into_iter()
                } else {
                    let empty = 0..0;
                    empty.into_iter()
                }
            })
            .collect();
    }
}

fn main() {
    let file = include_str!("../input");
    println!("Part 1: {}", part_one(file));
    println!("Part 2: {}", part_two(file));
}

pub fn part_one(file: &str) -> usize {
    let almanac = Almanac::from_str(file).unwrap_or_default();
    almanac.min_location()
}

pub fn part_two(file: &str) -> usize {
    let mut almanac = Almanac::from_str(file).unwrap_or_default();
    almanac.set_seeds_as_ranges();
    almanac.min_location()
}

#[cfg(test)]
mod day_2_tests {
    use crate::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_one(include_str!("../example")), 35);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_two(include_str!("../example")), 46);
    }
}

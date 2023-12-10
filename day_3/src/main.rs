fn main() {
    let file = include_str!("../input").to_owned();
    println!("Part 1: {}", part_one(String::from(&file)));
    println!("Part 2: {}", part_two(file));
}

#[derive(Debug, Clone)]
struct Number {
    start: usize,
    length: usize,
    seen: bool,
    val: u32,
}

impl Number {
    fn new() -> Self {
        Number {
            start: 0,
            length: 0,
            seen: false,
            val: 0,
        }
    }

    fn is_part(&self, line_length: usize, symbols: &Vec<usize>) -> bool {
        for symbol in symbols.iter() {
            let line_length = line_length as isize;
            let start = self.start as isize;
            let length = self.length as isize;
            let symbol = *symbol as isize;
            let prev = (start - line_length - 1)..(start + length - line_length + 1);
            let same = (start - 1)..(start + length + 1);
            let next = (start + line_length - 1)..(start + length + line_length + 1);
            if prev.contains(&symbol) || same.contains(&symbol) || next.contains(&symbol) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
struct Symbol {
    position: usize,
    ratio_1: u32,
    ratio_2: u32,
    is_star: bool,
}

impl Symbol {
    fn new() -> Self {
        Symbol {
            position: 0,
            ratio_1: 0,
            ratio_2: 0,
            is_star: false,
        }
    }

    fn get_ratio(&mut self, parts: &Vec<Number>, line_length: usize) -> u32 {
        if self.is_star {
            let neighbors: Vec<&Number> = parts
                .iter()
                .filter(|part| part.is_part(line_length, &vec![self.position]))
                .collect();
            if neighbors.len() == 2 {
                self.ratio_1 = neighbors.first().unwrap().val;
                self.ratio_2 = neighbors.last().unwrap().val;
            }
        }

        self.ratio_1 * self.ratio_2
    }
}

pub fn part_one(file: String) -> u32 {
    let length = file.lines().next().unwrap_or("").len();

    let lines: String = file.lines().map(str::trim).collect();
    let mut symbols: Vec<usize> = vec![];
    let mut positions: Vec<Number> = vec![];
    let mut num = Number::new();
    lines.chars().enumerate().for_each(|(i, c)| {
        if c.is_ascii_digit() {
            if !num.seen {
                num.start = i;
                num.seen = true;
            }
            return;
        }

        if num.seen {
            num.length = i - num.start;
            num.val = lines[num.start..num.start + num.length]
                .parse()
                .expect("Failed to parse u32");
            positions.push(num.clone());
            num.seen = false;
        }
        if c.is_ascii_punctuation() && c != '.' {
            symbols.push(i);
        }
    });

    positions
        .iter()
        .filter(|p| p.is_part(length, &symbols))
        .map(|p| p.val)
        .sum::<u32>()
}

pub fn part_two(file: String) -> u32 {
    let length = file.lines().next().unwrap_or("").len();

    let lines: String = file.lines().map(str::trim).collect();
    let mut symbols: Vec<Symbol> = vec![];
    let mut positions: Vec<Number> = vec![];
    let mut num = Number::new();
    lines.chars().enumerate().for_each(|(i, c)| {
        if c.is_ascii_digit() {
            if !num.seen {
                num.start = i;
                num.seen = true;
            }
            return;
        }

        if num.seen {
            num.length = i - num.start;
            num.val = lines[num.start..num.start + num.length]
                .parse()
                .expect("Failed to parse u32");
            positions.push(num.clone());
            num.seen = false;
        }
        if c.is_ascii_punctuation() && c != '.' {
            let mut s = Symbol::new();
            s.position = i;
            if c == '*' {
                s.is_star = true;
            }
            symbols.push(s);
        }
    });

    symbols
        .iter_mut()
        .map(|s| s.get_ratio(&positions, length))
        .sum::<u32>()
}

#[cfg(test)]
mod day_3_tests {
    use crate::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_one(include_str!("../example").to_owned()), 4361);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_two(include_str!("../example").to_owned()), 467835);
    }
}

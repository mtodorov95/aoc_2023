use std::collections::VecDeque;

fn main() {
    let file = include_str!("../input");
    println!("Part 1: {}", part_one(file));
    println!("Part 2: {}", part_two(file));
}

fn part_one(file: &str) -> isize {
    let sequences: Vec<Vec<isize>> = file
        .lines()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|num| num.parse::<isize>().unwrap_or_default())
                .collect()
        })
        .collect();
    let mut sum = 0;
    for seq in sequences.iter() {
        let mut diffs: Vec<Vec<isize>> = vec![seq.clone()];
        let mut current = seq.clone();
        while !current.iter().all(|e| *e == 0) {
            let temp: Vec<isize> = current
                .iter()
                .enumerate()
                .filter_map(|(idx, num)| {
                    if let Some(v) = current.get(idx + 1) {
                        return Some(v - num);
                    }
                    None
                })
                .collect();
            diffs.push(temp.clone());
            current = temp;
        }
        let mut last = diffs.pop().unwrap();
        last.push(0);
        while let Some(mut t) = diffs.pop() {
            let x = t.last().unwrap();
            t.push(x + last.last().unwrap());
            last = t;
        }
        sum += last.last().unwrap();
    }
    sum
}

fn part_two(file: &str) -> isize {
    let sequences: Vec<Vec<isize>> = file
        .lines()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|num| num.parse::<isize>().unwrap_or_default())
                .collect()
        })
        .collect();
    let mut sum = 0;
    for seq in sequences.iter() {
        let mut diffs: Vec<Vec<isize>> = vec![seq.clone()];
        let mut current = seq.clone();
        while !current.iter().all(|e| *e == 0) {
            let temp: Vec<isize> = current
                .iter()
                .enumerate()
                .filter_map(|(idx, num)| {
                    if let Some(v) = current.get(idx + 1) {
                        return Some(v - num);
                    }
                    None
                })
                .collect();
            diffs.push(temp.clone());
            current = temp;
        }

        let last = diffs.pop().unwrap();
        let mut last = VecDeque::from(last);
        last.push_front(0);
        while let Some(t) = diffs.pop() {
            let x = t.first().unwrap();
            let mut t = VecDeque::from(t.clone());
            t.push_front(x - last.front().unwrap());
            last = t;
        }
        sum += last.front().unwrap();
    }
    sum
}

#[cfg(test)]
mod day_9_tests {
    use crate::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_one(include_str!("../example")), 114);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_two(include_str!("../example")), 2);
    }
}

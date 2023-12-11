use std::{collections::VecDeque, str::FromStr};

#[derive(Default, Debug)]
struct Node {
    label: String,
    next_options: [String; 2],
}

impl FromStr for Node {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split(" = ");
        let label: String = line.next().unwrap_or_default().to_string();
        let node_options: Vec<&str> = line
            .next()
            .unwrap_or_default()
            .strip_prefix("(")
            .unwrap_or_default()
            .strip_suffix(")")
            .unwrap_or_default()
            .split(", ")
            .map(|node| node)
            .collect();
        let res = Node {
            label,
            next_options: [node_options[0].to_string(), node_options[1].to_string()],
        };
        Ok(res)
    }
}

#[derive(Default, Debug)]
struct Map {
    order: VecDeque<usize>,
    nodes: Vec<Node>,
}

impl FromStr for Map {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.trim().split("\n\n");
        let order = lines
            .next()
            .unwrap_or_default()
            .chars()
            .map(|c| {
                if c == 'R' {
                    return 1;
                }
                return 0;
            })
            .collect();
        let nodes: Vec<Node> = lines
            .next()
            .unwrap_or_default()
            .lines()
            .map(|line| Node::from_str(line).unwrap_or_default())
            .collect();
        let res = Map { order, nodes };
        Ok(res)
    }
}

impl Map {
    fn walk_to_end(&mut self) -> usize {
        let mut steps = 0;
        let mut current = "AAA";

        while current != "ZZZ" {
            let node: &Node = self
                .nodes
                .iter()
                .find(|node| node.label == current)
                .unwrap();
            current = &node.label;
            steps += 1;

            if let Some(instruction) = self.order.pop_front() {
                self.order.push_back(instruction);
                current = &node.next_options[instruction];
            };
        }
        steps
    }
}

fn main() {
    let file = include_str!("../input");
    println!("Part 1: {}", part_one(file));
    println!("Part 2: {}", part_two(file));
}

fn part_one(file: &str) -> usize {
    let mut map = Map::from_str(file).unwrap_or_default();
    map.walk_to_end()
}

fn part_two(file: &str) -> usize {
    let mut map = Map::from_str(file).unwrap_or_default();
    map.walk_to_end()
}

#[cfg(test)]
mod day_8_tests {
    use crate::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_one(include_str!("../example")), 2);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_two(include_str!("../example")), 5905);
    }
}

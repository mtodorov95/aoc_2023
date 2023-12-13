use std::str::FromStr;

#[derive(Debug, Default, Clone, Copy)]
struct Pipe {
    shape: char,
    pos: (usize, usize),
}

impl From<(char, (usize, usize))> for Pipe {
    fn from(value: (char, (usize, usize))) -> Self {
        Pipe {
            shape: value.0,
            pos: (value.1 .0, value.1 .1),
        }
    }
}

impl Pipe {
    fn next(&self, prev: &Pipe) -> (isize, isize) {
        match self.shape {
            '|' => match prev.pos.1 as isize - self.pos.1 as isize {
                -1 => (self.pos.0 as isize, self.pos.1 as isize + 1),
                1 => (self.pos.0 as isize, self.pos.1 as isize - 1),
                _ => (0, 0),
            },
            '-' => match prev.pos.0 as isize - self.pos.0 as isize {
                -1 => (self.pos.0 as isize + 1, self.pos.1 as isize),
                1 => (self.pos.0 as isize - 1, self.pos.1 as isize),
                _ => (0, 0),
            },
            '7' => match prev.pos.0 == self.pos.0 {
                true => (self.pos.0 as isize - 1, self.pos.1 as isize),
                false => (self.pos.0 as isize, self.pos.1 as isize + 1),
            },
            'L' => match prev.pos.0 == self.pos.0 {
                true => (self.pos.0 as isize + 1, self.pos.1 as isize),
                false => (self.pos.0 as isize, self.pos.1 as isize - 1),
            },
            'F' => match prev.pos.0 == self.pos.0 {
                true => (self.pos.0 as isize + 1, self.pos.1 as isize),
                false => (self.pos.0 as isize, self.pos.1 as isize + 1),
            },
            'J' => match prev.pos.0 == self.pos.0 {
                true => (self.pos.0 as isize - 1, self.pos.1 as isize),
                false => (self.pos.0 as isize, self.pos.1 as isize - 1),
            },
            _ => (0, 0),
        }
    }
}

#[derive(Debug, Default)]
struct Maze {
    layout: Vec<Vec<char>>,
    pipes: Vec<Pipe>,
    start: (isize, isize),
}

impl FromStr for Maze {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let layout: Vec<Vec<char>> = s
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();
        let pipes: Vec<Pipe> = layout
            .iter()
            .enumerate()
            .flat_map(move |(col, chars)| {
                chars.iter().enumerate().filter_map(move |(row, c)| {
                    if *c != '.' {
                        return Some(Pipe::from((*c, (row, col))));
                    }
                    None
                })
            })
            .collect();
        let start: (usize, usize) = pipes.iter().find(|pipe| pipe.shape == 'S').unwrap().pos;
        let start: (isize, isize) = (start.0 as isize, start.1 as isize);
        let res = Maze {
            layout,
            pipes,
            start,
        };
        Ok(res)
    }
}

impl Maze {
    fn walk(&self) -> Vec<Pipe> {
        let neighbors: Vec<Pipe> = self.get_neighbors(self.start);
        let mut neighbors = neighbors.iter();
        let mut current_pipe = neighbors.skip(2).next().unwrap();
        let mut prev = self
            .pipes
            .iter()
            .find(|pipe| pipe.pos.0 as isize == self.start.0 && pipe.pos.1 as isize == self.start.1)
            .unwrap();
        let mut path:Vec<Pipe> = vec![];

        let mut steps = 1;
        path.push(current_pipe.clone());
        while current_pipe.shape != 'S' {
            let next = current_pipe.next(prev);
            prev = current_pipe;
            if let Some(p) = self
                .pipes
                .iter()
                .find(|pipe| pipe.pos.0 as isize == next.0 && pipe.pos.1 as isize == next.1)
            {
                current_pipe = p;
                path.push(current_pipe.clone());
                steps += 1;
            }
        }
        path
    }

    fn get_neighbors(&self, at: (isize, isize)) -> Vec<Pipe> {
        let mut neighbors: Vec<Pipe> = vec![];
        for pipe in self.pipes.iter() {
            if ((pipe.pos.0 as isize == at.0 - 1 || pipe.pos.0 as isize == at.0 + 1)
                && pipe.pos.1 as isize == at.1)
                || ((pipe.pos.1 as isize == at.1 - 1 || pipe.pos.1 as isize == at.1 + 1)
                    && pipe.pos.0 as isize == at.0)
            {
                neighbors.push(pipe.clone());
            }
        }
        neighbors
    }
}
fn polygon_area(vertices: &Vec<Pipe>) -> usize {
    let number_of_vertices = vertices.len();
    let mut sum1 = 0;
    let mut sum2 = 0;

    for i in 0..number_of_vertices - 1 {
        sum1 = sum1 + vertices[i].pos.0 * vertices[i + 1].pos.1;
        sum2 = sum2 + vertices[i].pos.1 * vertices[i + 1].pos.0;
    }

    sum1 = sum1 + vertices[number_of_vertices - 1].pos.0 * vertices[0].pos.1;

    sum2 = sum2 + vertices[0].pos.0 * vertices[number_of_vertices - 1].pos.1;

    return (sum1 - sum2) / 2;
}

fn main() {
    let file = include_str!("../input");
    println!("Part 1: {}", part_one(file));
    println!("Part 2: {}", part_two(file));
}

fn part_one(file: &str) -> usize {
    let maze = Maze::from_str(file).unwrap_or_default();
    let path = maze.walk();
    path.len()/2
}

fn part_two(file: &str) -> usize {
    let maze = Maze::from_str(file).unwrap_or_default();
    let path = maze.walk();
    let area = polygon_area(&path);
    area - (path.len() / 2) + 1
}

#[cfg(test)]
mod day_10_tests {
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

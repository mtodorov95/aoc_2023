fn main() {
    let file = include_str!("../input").to_owned();
    println!("Part 1: {}", part_one(String::from(&file)));
    println!("Part 2: {}", part_two(file));
}

pub fn part_one(file: String) -> u32 {
    let res: u32 = file
        .lines()
        .map(|line| {
            let nums: Vec<char> = line.chars().filter(|&char| char.is_ascii_digit()).collect();
            let first = nums.first().unwrap().to_owned();
            let second = nums.last().unwrap().to_owned();
            let number = format!("{}{}", first, second).parse::<u32>().unwrap_or(0);
            number
        })
        .sum();
    res
}

pub fn part_two(file: String) -> u32 {
    let spelled = vec![
        "oneight",
        "twone",
        "threeight",
        "fiveight",
        "sevenine",
        "eightwo",
        "eighthree",
        "nineight",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ];
    let values = vec![
        "18", "21", "38", "58", "79", "82", "83", "98", "1", "2", "3", "4", "5", "6", "7", "8", "9",
    ];

    let res: u32 = file
        .lines()
        .map(|line| {
            let mut line = line.to_owned();
            for (i, num) in spelled.iter().enumerate() {
                line = line.replace(num, values[i]);
            }
            let nums: Vec<char> = line.chars().filter(|&char| char.is_ascii_digit()).collect();
            let first = nums.first().unwrap().to_owned();
            let second = nums.last().unwrap().to_owned();
            let number = format!("{}{}", first, second).parse::<u32>().unwrap_or(0);
            number
        })
        .sum();
    res
}

#[cfg(test)]
mod day_1_tests {
    use crate::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_one(include_str!("../example").to_owned()), 142);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_two(include_str!("../example-2").to_owned()), 281);
    }
}

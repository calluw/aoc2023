use aoc2023::{get_day_input, print_elapsed_time};
use phf::phf_map;
use std::iter::FromIterator;

const DIGIT_MAP: phf::Map<&'static str, char> = phf_map! {
    "one" => '1',
    "two" => '2',
    "three" => '3',
    "four" => '4',
    "five" => '5',
    "six" => '6',
    "seven" => '7',
    "eight" => '8',
    "nine" => '9',
};

fn is_digit(ch: &char) -> bool {
    ('0'..='9').contains(ch)
}

fn part_one(input: &Vec<String>) -> u32 {
    input
        .iter()
        .map(|line| {
            let first = line.chars().filter(is_digit).next().unwrap();
            let last = line.chars().rev().filter(is_digit).next().unwrap();
            String::from_iter([first, last]).parse::<u32>().unwrap()
        })
        .sum()
}

fn get_digit(input: &String, rev: bool) -> char {
    let input: String = if rev {
        input.chars().rev().collect()
    } else {
        input.to_string()
    };

    let mut slice = input.as_str();

    loop {
        if ('0'..'9').contains(&slice.chars().nth(0).unwrap()) {
            return slice.chars().nth(0).unwrap();
        }
        for (key, digit) in DIGIT_MAP.entries() {
            let rev_key = key.chars().rev().collect::<String>();
            if slice.starts_with(if !rev { key } else { rev_key.as_str() }) {
                return *digit;
            }
        }
        slice = &slice[1..];
    }
}

fn part_two(input: &Vec<String>) -> u32 {
    input
        .iter()
        .map(|line| {
            let first = get_digit(line, false);
            let last = get_digit(line, true);
            String::from_iter([first, last]).parse::<u32>().unwrap()
        })
        .sum()
}

fn parse_input(input_str: &str) -> Vec<String> {
    input_str.lines().map(|line| line.to_owned()).collect()
}

fn main() {
    let input_str = get_day_input("01");
    let input = parse_input(&input_str);
    println!("Day 01:");
    println!("=========");
    println!("Part one: {}", print_elapsed_time(|| part_one(&input)));
    println!("Part two: {}", print_elapsed_time(|| part_two(&input)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example_01() {
        let input_str: String = String::from(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );

        let input = parse_input(&input_str);

        assert_eq!(part_one(&input), 142);
    }

    #[test]
    fn test_next_example_01() {
        let input_str: String = String::from(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );

        let input = parse_input(&input_str);

        assert_eq!(part_two(&input), 281);
    }
}

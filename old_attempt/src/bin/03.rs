use std::collections::{HashMap, HashSet};

use aoc2023::{get_day_input, print_elapsed_time};

type InputType = String;

fn part_one(input: &[InputType]) -> u32 {
    let symbol_coords: Vec<(usize, usize)> = input
        .iter()
        .enumerate()
        .map(|(i, s)| {
            s.chars()
                .enumerate()
                .filter(|(_, ch)| ch.is_ascii_punctuation() && *ch != '.')
                .map(move |(j, _)| (i, j))
        })
        .flatten()
        .collect();
    println!("Symbol coords: {:?}", symbol_coords);
    let symbol_adjacent_coords: Vec<(usize, usize)> = symbol_coords
        .iter()
        .map(|(i, j)| {
            let mut adjacent_coords: HashSet<(usize, usize)> = HashSet::new();
            // i
            adjacent_coords.insert((*i, *j));
            adjacent_coords.insert((*i, j + 1));
            adjacent_coords.insert((*i, j.checked_sub(1).unwrap_or(0)));
            // i + 1
            adjacent_coords.insert((i + 1, *j));
            adjacent_coords.insert((i + 1, j + 1));
            adjacent_coords.insert((i + 1, j.checked_sub(1).unwrap_or(0)));
            // i - 1
            adjacent_coords.insert((i.checked_sub(1).unwrap_or(0), *j));
            adjacent_coords.insert((i.checked_sub(1).unwrap_or(0), *j + 1));
            adjacent_coords.insert((i.checked_sub(1).unwrap_or(0), j.checked_sub(1).unwrap_or(0)));
            adjacent_coords
        })
        .flatten()
        .collect();
    println!("Symbol adjacent coords {:?}", symbol_adjacent_coords);
    0
}

fn part_two(_input: &[InputType]) -> u32 {
    todo!()
}

fn parse_input(input: &str) -> Vec<InputType> {
    input.lines().map(|l| l.to_owned()).collect()
}

fn main() {
    let input_str = get_day_input("03");
    let input = parse_input(&input_str);
    println!("Day 03:");
    println!("=========");
    println!("Part one: {}", print_elapsed_time(|| part_one(&input)));
    println!("Part two: {}", print_elapsed_time(|| part_two(&input)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let input_str: String = String::from(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );

        let input = parse_input(&input_str);

        assert_eq!(part_one(&input), 4361);
    }
}

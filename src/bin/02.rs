use std::{collections::BTreeMap, str::FromStr};

use aoc2023::{get_day_input, print_elapsed_time};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending},
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
struct Cubes {
    num: u32,
    colour: String,
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Vec<Cubes>>,
}

fn part_one(input: &[Game]) -> u32 {
    let map: BTreeMap<&'static str, u32> =
        BTreeMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut possible_games = Vec::new();
    for game in input {
        let mut possible = true;
        for round in &game.rounds {
            for cubes in round {
                let colour_max: u32 = *map.get(&cubes.colour as &str).expect("Got unknown colour");
                if cubes.num > colour_max {
                    possible = false;
                }
            }
        }
        if possible {
            possible_games.push(game.id);
        }
    }
    possible_games.iter().sum()
}

fn part_two(input: &[Game]) -> u32 {
    let mut powers = Vec::new();
    for game in input {
        let mut map = BTreeMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        for round in &game.rounds {
            for cubes in round {
                let colour_max: u32 = *map.get(&cubes.colour as &str).expect("Got unknown colour");
                if cubes.num > colour_max {
                    map.entry(&cubes.colour).and_modify(|e| *e = cubes.num);
                }
            }
        }
        powers.push(map.values().product());
    }
    powers.iter().sum()
}

fn parse_cube(input: &str) -> IResult<&str, Cubes> {
    let (input, (num, colour)) = separated_pair(
        map_res(digit1, str::parse),
        tag(" "),
        map_res(alpha1, String::from_str),
    )(input)?;
    Ok((input, Cubes { num, colour }))
}

fn parse_cubes(input: &str) -> IResult<&str, Vec<Cubes>> {
    Ok(separated_list1(tag(", "), parse_cube)(input)?)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, id) = preceded(tag("Game "), map_res(digit1, str::parse))(input)?;
    let (input, rounds) = preceded(tag(": "), separated_list1(tag("; "), parse_cubes))(input)?;
    Ok((input, Game { id, rounds }))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    Ok(separated_list1(line_ending, parse_game)(input)?)
}

fn parse_input(input: &str) -> Vec<Game> {
    let (_, games) = parse_games(input).expect("Input should parse correctly!");
    games
}

fn main() {
    let input_str = get_day_input("02");
    let input = parse_input(&input_str);
    println!("Day 02:");
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
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );

        let input = parse_input(&input_str);

        assert_eq!(part_one(&input), 8);
        assert_eq!(part_two(&input), 2286);
    }
}

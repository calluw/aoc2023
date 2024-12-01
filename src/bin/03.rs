use nom::{
    branch::alt,
    character::complete::{char, digit1, none_of},
    combinator::{map, map_res},
    multi::many1,
    IResult,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    str::FromStr,
};

advent_of_code::solution!(3);

type Coord = (isize, isize);

fn coord_neighbours(coord: Coord) -> BTreeSet<Coord> {
    BTreeSet::from([
        // Same line
        (coord.0, coord.1 + 1),
        (coord.0, coord.1 - 1),
        // Above
        (coord.0 - 1, coord.1),
        (coord.0 - 1, coord.1 + 1),
        (coord.0 - 1, coord.1 - 1),
        // Below
        (coord.0 + 1, coord.1),
        (coord.0 + 1, coord.1 + 1),
        (coord.0 + 1, coord.1 - 1),
    ])
}

#[derive(Debug)]
enum SchematicEntry {
    PartNumber(u32),
    Symbol(char),
    Period,
}

fn parse_part_number(input: &str) -> IResult<&str, (SchematicEntry, usize)> {
    let initial_len = input.len();

    let (input, num) = map_res(digit1, u32::from_str)(input)?;

    let chars_parsed = initial_len - input.len();
    Ok((input, (SchematicEntry::PartNumber(num), chars_parsed)))
}

fn parse_symbol(input: &str) -> IResult<&str, (SchematicEntry, usize)> {
    // For now assume a symbol is anything non-numeric except periods
    // This always parses exactly one character
    map(none_of(".1234567890"), |ch| (SchematicEntry::Symbol(ch), 1))(input)
}

fn parse_period(input: &str) -> IResult<&str, (SchematicEntry, usize)> {
    // This always parses exactly one character
    map(char('.'), |_| (SchematicEntry::Period, 1))(input)
}

fn parse_schematic_entry(input: &str) -> IResult<&str, (SchematicEntry, usize)> {
    alt((parse_part_number, parse_symbol, parse_period))(input)
}

fn parse_schematic_line(input: &str) -> IResult<&str, Vec<(SchematicEntry, usize)>> {
    many1(parse_schematic_entry)(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let schematic_lines = input
        .lines()
        .map(parse_schematic_line)
        .map(|r| r.expect("parsing puzzle input succeeds"))
        .map(|(_, schematic_line)| schematic_line);

    let mut number_coords = BTreeMap::<Coord, (u32, usize)>::new();
    let mut symbol_coords = BTreeSet::<Coord>::new();

    for (i, line) in schematic_lines.enumerate() {
        let i = i as isize;
        let mut j: isize = 0;
        for (entry, width) in line {
            match entry {
                SchematicEntry::PartNumber(num) => {
                    number_coords.insert((i, j), (num, width));
                }
                SchematicEntry::Symbol(_) => {
                    symbol_coords.insert((i, j));
                }
                SchematicEntry::Period => {}
            };
            j += width as isize;
        }
    }

    for (mut coord, (num, width)) in number_coords {}

    todo!()
}

pub fn part_two(input: &str) -> Option<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

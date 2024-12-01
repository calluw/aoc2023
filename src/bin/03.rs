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

fn get_coord_neighbours(coord: Coord) -> BTreeSet<Coord> {
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

fn get_schematic(input: &str) -> Vec<Vec<(SchematicEntry, usize)>> {
    input
        .lines()
        .map(parse_schematic_line)
        .map(|r| r.expect("parsing puzzle input succeeds"))
        .map(|(_, schematic_line)| schematic_line)
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let schematic = get_schematic(input);

    let mut number_coords = BTreeMap::<Coord, (u32, usize)>::new();
    let mut symbol_coords = BTreeSet::<Coord>::new();

    for (i, line) in schematic.into_iter().enumerate() {
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

    let mut total_touching = 0;

    for (coord, (num, width)) in number_coords {
        let mut neighbour_coords = BTreeSet::new();
        for offset in 0..width as isize {
            let relevant_coord = (coord.0, coord.1 + offset);
            neighbour_coords.extend(get_coord_neighbours(relevant_coord));
        }

        let touches_symbol = neighbour_coords.iter().any(|c| symbol_coords.contains(c));

        if touches_symbol {
            total_touching += num;
        }
    }

    Some(total_touching)
}

pub fn part_two(input: &str) -> Option<u32> {
    let schematic = get_schematic(input);

    let mut number_coords = BTreeMap::<Coord, (u32, usize)>::new();
    let mut asterisk_coords = BTreeMap::<Coord, Vec<u32>>::new();

    for (i, line) in schematic.into_iter().enumerate() {
        let i = i as isize;
        let mut j: isize = 0;
        for (entry, width) in line {
            match entry {
                SchematicEntry::PartNumber(num) => {
                    number_coords.insert((i, j), (num, width));
                }
                SchematicEntry::Symbol(ch) => {
                    if ch == '*' {
                        asterisk_coords.insert((i, j), Vec::new());
                    }
                }
                SchematicEntry::Period => {}
            };
            j += width as isize;
        }
    }

    for (coord, (num, width)) in number_coords {
        let mut neighbour_coords = BTreeSet::new();
        for offset in 0..width as isize {
            let relevant_coord = (coord.0, coord.1 + offset);
            neighbour_coords.extend(get_coord_neighbours(relevant_coord));
        }

        let mut asterisks_touched = BTreeSet::new();

        for neighbour_coord in neighbour_coords {
            if asterisk_coords.contains_key(&neighbour_coord) {
                asterisks_touched.insert(neighbour_coord);
            }
        }

        for asterisk_coord in asterisks_touched {
            asterisk_coords.entry(asterisk_coord).and_modify(|nums| nums.push(num));
        }
    }

    let mut gear_ratio_sum = 0;
    for (_, nums) in asterisk_coords {
        if nums.len() != 2 {
            continue;
        }

        gear_ratio_sum += nums.iter().product::<u32>();
    }

    Some(gear_ratio_sum)
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
        assert_eq!(result, Some(467835));
    }
}

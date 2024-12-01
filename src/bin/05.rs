use nom::{
    bytes::complete::tag,
    character::{complete::digit1, complete::multispace1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};
use std::str::FromStr;

advent_of_code::solution!(5);

struct Range {
    dest_start: usize,
    source_start: usize,
    source_end: usize,
    offset: isize,
}

impl Range {
    fn source_contains(&self, x: usize) -> bool {
        x >= self.source_start && x < self.source_end
    }

    /// This will only work if you already checked that the source value is in the range
    fn map_unchecked(&self, x: usize) -> usize {
        (x as isize + self.offset) as usize
    }

    /// This returns true if the value was actually mapped, else returned unmapped
    fn check_map(&self, x: usize) -> (bool, usize) {
        if self.source_contains(x) {
            (true, self.map_unchecked(x))
        } else {
            (false, x)
        }
    }
}

fn parse_range(input: &str) -> IResult<&str, Range> {
    let (input, (dest, source, length)) = tuple((
        map_res(digit1, usize::from_str),
        preceded(multispace1, map_res(digit1, usize::from_str)),
        preceded(multispace1, map_res(digit1, usize::from_str)),
    ))(input)?;
    Ok((
        input,
        Range {
            dest_start: dest,
            source_start: source,
            source_end: source + length,
            offset: dest as isize - source as isize,
        },
    ))
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<usize>> {
    preceded(
        tag("seeds: "),
        separated_list1(multispace1, map_res(digit1, usize::from_str)),
    )(input)
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<Vec<Range>>) {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let seed_section = &sections[0];
    let map_sections = &sections[1..];

    let seeds = parse_seeds(seed_section).expect("seeds input parses").1;
    let maps = map_sections
        .iter()
        .map(|s| {
            s.lines()
                .skip(1)
                .map(parse_range)
                .map(|r| r.expect("puzzle input parses").1)
                .collect::<Vec<Range>>()
        })
        .collect::<Vec<Vec<Range>>>();

    (seeds, maps)
}

fn get_min_location(seeds: Vec<usize>, maps: Vec<Vec<Range>>) -> Option<u32> {
    let mut mapped_seeds = Vec::new();
    for mut seed in seeds {
        for map in &maps {
            for range in map {
                let (mapped, new_seed) = range.check_map(seed);
                seed = new_seed;
                if mapped {
                    break;
                }
            }
        }
        mapped_seeds.push(seed);
    }

    mapped_seeds.into_iter().min().map(|x| x as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (seeds, maps) = parse_input(input);
    get_min_location(seeds, maps)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (seeds, maps) = parse_input(input);

    // Expand out the seeds
    let mut new_seeds = Vec::new();
    for pair in seeds.chunks_exact(2) {
        let start = pair[0];
    }

    get_min_location(seeds, maps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

use std::collections::BTreeSet;
use std::str::FromStr;

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0, multispace1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, pair, terminated},
    IResult,
};

advent_of_code::solution!(4);

#[derive(Debug, Clone)]
struct Card {
    num: usize,
    winning: Vec<u32>,
    present: Vec<u32>,
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, num) = delimited(
        pair(tag("Card "), multispace0),
        map_res(digit1, usize::from_str),
        pair(char(':'), multispace0),
    )(input)?;
    let (input, winning) = terminated(
        separated_list1(multispace1, map_res(digit1, u32::from_str)),
        delimited(multispace0, char('|'), multispace0),
    )(input)?;
    let (input, present) = separated_list1(multispace1, map_res(digit1, u32::from_str))(input)?;

    Ok((
        input,
        Card {
            num,
            winning,
            present,
        },
    ))
}

pub fn part_one(input: &str) -> Option<u32> {
    let cards: Vec<Card> = input
        .lines()
        .map(parse_card)
        .map(|r| r.expect("puzzle input to parse").1)
        .collect();

    let mut total_points = 0;

    for card in cards {
        let winning_set = BTreeSet::from_iter(card.winning);
        let num_winning = card
            .present
            .iter()
            .filter(|n| winning_set.contains(n))
            .count();

        if num_winning > 0 {
            total_points += 2u32.pow(num_winning as u32 - 1);
        }
    }

    Some(total_points)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards: Vec<Card> = input
        .lines()
        .map(parse_card)
        .map(|r| r.expect("puzzle input to parse").1)
        .collect();

    let mut current_cards: Vec<usize> = cards.iter().map(|c| c.num).collect();

    let mut card_total = 0;

    while !current_cards.is_empty() {
        let mut new_cards: Vec<usize> = Vec::new();
        for card_num in current_cards {
            let card = &cards[card_num - 1];
            let winning_set = BTreeSet::from_iter(&card.winning);
            let num_winning = card
                .present
                .iter()
                .filter(|n| winning_set.contains(n))
                .count();

            for offset in 0..num_winning {
                new_cards.push(card_num + offset + 1);
            }

            card_total += 1;
        }
        current_cards = new_cards;
    }

    Some(card_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}

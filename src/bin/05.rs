use std::collections::HashMap;

use gxhash::HashSet;
use itertools::Itertools;
use nom::{
    character::complete::{char, newline, u32},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};
use pathfinding::num_traits::ops::inv;

advent_of_code::solution!(5);

type Rules = HashMap<u32, HashSet<u32>>;
type Updates = Vec<Vec<u32>>;

fn rule(input: &str) -> IResult<&str, (u32, u32)> {
    map(
        terminated(separated_pair(u32, char('|'), u32), newline),
        |(a, b)| (b, a),
    )(input)
}

fn rules(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    terminated(many1(rule), newline)(input)
}

fn updates(input: &str) -> IResult<&str, Updates> {
    many1(terminated(separated_list1(char(','), u32), newline))(input)
}

fn parse_input(input: &str) -> IResult<&str, (Rules, Updates)> {
    let (input, rules) = rules(input)?;
    let (input, updates) = updates(input)?;

    let rules = rules.into_iter().into_grouping_map().collect();
    Ok((input, (rules, updates)))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_input, (rules, updates)) = parse_input(input).unwrap();

    Some(
        updates
            .iter()
            .filter_map(|update| {
                let mid = update.len() / 2;
                update
                    .iter()
                    .tuple_windows()
                    .all(|(a, b)| match rules.get(b) {
                        Some(prev_pages) => prev_pages.contains(a),
                        None => false,
                    })
                    .then_some(update[mid])
            })
            .sum(),
    )
}

fn fix_invalid(invalid: Vec<u32>, rules: &Rules) -> Vec<u32> {
    
    let bad: Vec<_> = invalid.iter()
    .tuple_windows()
    .filter(|&(a, b)| match rules.get(b) {
        Some(prev_pages) => !prev_pages.contains(a),
        None => true,
    }).collect();

    invalid.

    invalid
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_input, (rules, updates)) = parse_input(input).unwrap();

    let invalid: Vec<_> = updates
        .iter()
        .filter(|update| {
            update
                .iter()
                .tuple_windows()
                .any(|(a, b)| match rules.get(b) {
                    Some(prev_pages) => !prev_pages.contains(a),
                    None => true,
                })
        })
        .collect();

    dbg!(invalid);
    
    Some(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}

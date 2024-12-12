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

pub fn part_two(input: &str) -> Option<u32> {
    let (_input, (rules, mut updates)) = parse_input(input).unwrap();

    let invalid_indices = updates
        .iter()
        .enumerate()
        .flat_map(|(ix, update)| {
            update
                .iter()
                .tuple_windows()
                .enumerate()
                .flat_map(|(i, (a, b))| match rules.get(b) {
                    Some(prev) if !prev.contains(a) => Some((ix, i + 1)),
                    None => Some((ix, i + 1)),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .into_group_map();

    invalid_indices.iter().for_each(|(&update_ix, pages_ix)| {
        let update = &mut updates[update_ix];
        // println!("Starting to fix {update:#?}");
        let pages: Vec<_> = pages_ix
            .iter()
            // .sorted()
            .rev()
            .map(|&page_ix| update.remove(page_ix))
            .collect();
        // println!("Incorrect pages: {pages:#?}");

        for page in pages {
            if let Some((correct_position, _)) =
                update.iter().enumerate().find(|&(_upd_ix, upd_page)| {
                    let Some(prev) = rules.get(upd_page) else {
                        return false;
                    };
                    if prev.contains(&page) {
                        // println!("Found right position for {page} before page {upd_page}");
                        true
                    } else {
                        false
                    }
                })
            {
                update.insert(correct_position, page);
                // dbg!(&update);
            }
        }
    });

    Some(
        invalid_indices
            .keys()
            .map(|&upd_ix| (upd_ix, updates[upd_ix].len() / 2))
            .map(|(upd_ix, mid)| updates[upd_ix][mid])
            .sum(),
    )
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

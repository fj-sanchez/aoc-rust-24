use itertools::Itertools;
use nom::{
    character::complete::{i32, space1},
    multi::separated_list1,
    IResult,
};

advent_of_code::solution!(2);

fn parse_report(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(space1, i32)(input)
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| parse_report(line).unwrap().1)
        .collect()
}

fn is_invalid_report(report: &[i32]) -> bool {
    let diffs = report
        .iter()
        .tuple_windows()
        .map(|(&a, &b)| b - a)
        .collect_vec();

    diffs
        .iter()
        .any(|&x| x.signum() != diffs[0].signum() || x.abs() < 1 || x.abs() > 3)
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = parse_input(input);

    Some(
        reports
            .into_iter()
            .map(|report: std::vec::Vec<i32>| is_invalid_report(&report))
            .map(|invalid_level| !invalid_level as u32)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = parse_input(input);

    Some(
        reports
            .into_iter()
            .filter_map(|report| {
                if is_invalid_report(&report) {
                    let len = report.len();
                    report
                        .into_iter()
                        .combinations(len - 1)
                        .find(|subset| !is_invalid_report(subset))
                        .map(|_| 1)
                } else {
                    Some(1)
                }
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}

advent_of_code::solution!(3);

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, u32},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[derive(Debug, Clone)]
enum Op {
    Mul((u32, u32)),
    Do,
    Dont,
}

fn mul(input: &str) -> IResult<&str, Op> {
    let (input, pair) =
        delimited(tag("mul("), separated_pair(u32, char(','), u32), char(')'))(input)?;

    Ok((input, Op::Mul(pair)))
}

fn op(input: &str) -> IResult<&str, Op> {
    let (input, op) = alt((
        mul,
        value(Op::Do, tag("do()")),
        value(Op::Dont, tag("don't()")),
    ))(input)?;

    Ok((input, op))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Op>> {
    many1(many_till(anychar, op).map(|(_discarded, op)| op))(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_unused, ops) = parse_input(input).unwrap();

    let sum_product = ops
        .iter()
        .filter_map(|op| match op {
            Op::Mul((a, b)) => Some(a * b),
            _ => None,
        })
        .sum();

    Some(sum_product)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_unused, ops) = parse_input(input).unwrap();

    let (_enabled, sum_product) =
        ops.iter()
            .fold((true, 0), |(enabled, acc), op| match (enabled, op) {
                (true, &Op::Mul((a, b))) => (true, acc + a * b),
                (_, Op::Do) => (true, acc),
                (_, Op::Dont) => (false, acc),
                _ => (enabled, acc),
            });

    Some(sum_product)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}

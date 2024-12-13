use nom::{
    character::complete::{anychar, i64, line_ending},
    combinator::{map, opt},
    multi::{many1, many_till},
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

advent_of_code::solution!(13);

#[derive(Debug)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

fn label(input: &str) -> IResult<&str, String, ErrorTree<&str>> {
    let (input, label) = map(many_till(anychar, tag(": ")), |(label, _)| label)(input)?;
    Ok((input, String::from_iter(label)))
}

fn xy(input: &str) -> IResult<&str, (i64, i64), ErrorTree<&str>> {
    separated_pair(
        preceded(
            tuple((tag("X"), anychar)).context("Should be preceeded by X"),
            i64,
        ),
        tag(", "),
        preceded(
            tuple((tag("Y"), anychar)).context("Should be preceeded by Y"),
            i64,
        ),
    )(input)
}

fn line(input: &str) -> IResult<&str, (i64, i64), ErrorTree<&str>> {
    let (input, (_, xy, _)) = tuple((
        label,
        xy,
        line_ending.context("Should end with line_ending"),
    ))(input)?;
    Ok((input, xy))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Machine>, ErrorTree<&str>> {
    let machine = map(
        terminated(
            tuple((line, line, line)).context("Each machine should have 3 lines"),
            opt(line_ending),
        ),
        |(a, b, prize)| Machine { a, b, prize },
    );

    many1(machine)(input)
}

const A_COST: i64 = 3;
const B_COST: i64 = 1;
const PRIZE_OFFSET: i64 = 10000000000000;

fn solve_machine(machine: &Machine, prize_offset: i64) -> Option<(i64, i64)> {
    let (dxa, dya) = machine.a;
    let (dxb, dyb) = machine.b;
    let (px, py) = (
        machine.prize.0 + prize_offset,
        machine.prize.1 + prize_offset,
    );

    let b_num = dxa * py - px * dya;
    let b_den = dyb * dxa - dxb * dya;
    if (b_num % b_den) != 0 {
        return None;
    }
    let b = b_num / b_den;

    let a_num = px - b * dxb;
    let a_den = dxa;
    if (a_num % a_den) != 0 {
        return None;
    }
    let a = a_num / a_den;

    if a >= 0 && b >= 0 {
        Some((a, b))
    } else {
        None
    }
}

fn solve(machines: &[Machine], prize_offset: i64) -> i64 {
    machines
        .iter()
        .filter_map(|m| solve_machine(m, prize_offset))
        .map(|(a, b)| a * A_COST + b * B_COST)
        .sum()
}

pub fn part_one(input: &str) -> Option<i64> {
    let (_, machines) = parse_input(input).unwrap();
    Some(solve(&machines, 0))
}

pub fn part_two(input: &str) -> Option<i64> {
    let (_, machines) = parse_input(input).unwrap();
    Some(solve(&machines, PRIZE_OFFSET))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}

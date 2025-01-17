use std::{thread::sleep, time};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{char, i32, line_ending, space1},
    combinator::map,
    multi::many1,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};
use pathfinding::grid::Grid;

advent_of_code::solution!(14);

#[cfg(test)]
mod constants {
    pub const WIDTH: i32 = 11;
    pub const HEIGHT: i32 = 7;
    pub const PLAY_PART2: bool = true;
}

#[cfg(not(test))]
mod constants {
    pub const WIDTH: i32 = 101;
    pub const HEIGHT: i32 = 103;
    pub const PLAY_PART2: bool = false;
}

use constants::*;

type XY = (i32, i32);

#[derive(Debug, PartialEq, Eq, Hash)]
struct Robot {
    position: XY,
    velocity: XY,
}

fn xy(input: &str) -> IResult<&str, XY> {
    separated_pair(i32, char(','), i32)(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Robot>> {
    let robot = map(
        terminated(
            separated_pair(preceded(tag("p="), xy), space1, preceded(tag("v="), xy)),
            line_ending,
        ),
        |(position, velocity)| Robot { position, velocity },
    );
    let (input, robots) = many1(robot)(input)?;

    Ok((input, robots))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_input, robots) = parse_input(input).unwrap();

    let steps = 100;
    let x_mid = (WIDTH - 1) / 2;
    let y_mid = (HEIGHT - 1) / 2;

    Some(
        robots
            .iter()
            .map(move |r| {
                (
                    ((r.position.0 + r.velocity.0 * steps).rem_euclid(WIDTH)),
                    ((r.position.1 + r.velocity.1 * steps).rem_euclid(HEIGHT)),
                )
            })
            // .inspect(|&c| println!("Coord: {c:?}"))
            .filter(|&(x, y)| x != x_mid && y != y_mid)
            .map(|(x, y)| ((x < ((WIDTH - 1) / 2)), (y < ((HEIGHT - 1) / 2))))
            // .inspect(|(x, y)| println!("Cuadrant: {x},{y}"))
            .sorted()
            .dedup_with_count()
            .map(|(count, _q)| count as u32)
            .product(),
    )
}

pub fn part_two(input: &str) -> Option<bool> {
    let (_input, mut robots) = parse_input(input).unwrap();
    let steps = 1;

    let mut found = false;
    let mut t = 0;
    while !found && t < 100000 {
        robots.iter_mut().for_each(move |r| {
            r.position = (
                ((r.position.0 + r.velocity.0 * steps).rem_euclid(WIDTH)),
                ((r.position.1 + r.velocity.1 * steps).rem_euclid(HEIGHT)),
            );
        });
        t += 1;

        found = robots.iter().map(|r| r.position).all_unique();
    }

    // print only 100 frames around the solution
    if PLAY_PART2 {
        let g = Grid::from_iter(
            robots
                .iter()
                .map(|r| (r.position.0 as usize, r.position.1 as usize)),
        );
        println!("t={t}\n{g:#?}");
        sleep(time::Duration::from_millis(50));
    }

    Some(found)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(true));
    }
}

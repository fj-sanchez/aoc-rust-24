use std::{collections::HashMap, fmt};

use itertools::Itertools;
use pathfinding::{matrix::Matrix, prelude::dijkstra};

advent_of_code::solution!(18);

type Position = (usize, usize);

#[derive(Debug, PartialEq, Eq)]
pub struct PositionResult(usize, usize);

impl fmt::Display for PositionResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.0, self.1)
    }
}

impl From<Position> for PositionResult {
    fn from((x, y): Position) -> Self {
        PositionResult(x, y)
    }
}

#[cfg(test)]
mod constants {
    use super::Position;

    pub const START: Position = (0, 0);
    pub const END: Position = (6, 6);
    pub const START_TIME: i32 = 12;
}

#[cfg(not(test))]
mod constants {
    use super::Position;

    pub const START: Position = (0, 0);
    pub const END: Position = (70, 70);
    pub const START_TIME: i32 = 1024;
}

use constants::*;

fn parse_input(input: &str, (max_x, max_y): Position) -> (i32, Matrix<i32>) {
    let coords: HashMap<Position, i32> = input
        .lines()
        .enumerate()
        .map(|(t, line)| {
            let (x, y) = line
                .split(",")
                .map(|c| c.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            ((x, y), t as i32)
        })
        .collect();

    (
        coords.len() as i32,
        Matrix::from_fn(max_x, max_y, move |c| {
            coords.get(&c).unwrap_or(&-1).to_owned()
        }),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, map) = parse_input(input, (END.0 + 1, END.1 + 1));

    Some(get_shortest_path_at_t(START_TIME, &map).unwrap().1)
}

fn get_shortest_path_at_t(max_t: i32, map: &Matrix<i32>) -> Option<(Vec<(usize, usize)>, u32)> {
    dijkstra(
        &START,
        |&c| {
            map.neighbours(c, false)
                .filter(|&c| {
                    let t = map.get(c).unwrap();
                    !(0..max_t).contains(t)
                })
                .map(|c| (c, 1))
        },
        |&c| c == END,
    )
}

pub fn part_two(input: &str) -> Option<PositionResult> {
    let (max_t, map) = parse_input(input, (END.0 + 1, END.1 + 1));

    let no_exit_time = (0..max_t)
        .rev()
        .find(|&t| get_shortest_path_at_t(t, &map).is_some())
        .unwrap();

    let result = map
        .items()
        .find(|(_, &t)| t == no_exit_time)
        .map(|(c, _)| c)
        .map(PositionResult::from);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(PositionResult(6, 1)));
    }
}

use std::collections::BTreeMap;

use itertools::Itertools;
use pathfinding::{
    matrix::{directions::DIRECTIONS_4, Matrix},
    prelude::dfs,
};

advent_of_code::solution!(20);

#[cfg(test)]
mod constants {
    pub const MIN_SAVE_PART_1: i32 = 12;
    pub const MIN_SAVE_PART_2: i32 = 50;
}

#[cfg(not(test))]
mod constants {
    pub const MIN_SAVE_PART_1: i32 = 100;
    pub const MIN_SAVE_PART_2: i32 = 100;
}

use constants::*;

fn parse_input(input: &str) -> Matrix<char> {
    Matrix::from_rows(input.lines().map(|l| l.chars())).unwrap()
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = parse_input(input);
    let start = map.items().find(|(_, &v)| v == 'S').unwrap().0;
    let racetrack: BTreeMap<(usize, usize), i32> = dfs(
        start,
        |&n| {
            map.neighbours(n, false)
                .filter(|&n| map.get(n).unwrap() != &'#')
        },
        |&n| map.get(n).unwrap() == &'E',
    )
    .unwrap()
    .into_iter()
    .enumerate()
    .map(|(pos, coord)| (coord, pos as i32))
    .collect();

    Some(
        racetrack
            .iter()
            .flat_map(|(&coord, pos)| {
                DIRECTIONS_4
                    .iter()
                    .filter_map(|&dir| {
                        map.move_in_direction(coord, dir)
                            .and_then(|cheat_coord| map.move_in_direction(cheat_coord, dir))
                            .filter(|cheat_coord| {
                                racetrack.get(cheat_coord).map_or(false, |&cheat_pos| {
                                    cheat_pos - (pos + 2) >= MIN_SAVE_PART_1
                                })
                            })
                    })
                    .collect::<Vec<_>>()
            })
            .count(),
    )
}

fn distance(c1: &(usize, usize), c2: &(usize, usize)) -> usize {
    c1.0.abs_diff(c2.0) + c1.1.abs_diff(c2.1)
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = parse_input(input);
    let start = map.items().find(|(_, &v)| v == 'S').unwrap().0;
    let racetrack: BTreeMap<(usize, usize), i32> = dfs(
        start,
        |&n| {
            map.neighbours(n, false)
                .filter(|&n| map.get(n).unwrap() != &'#')
        },
        |&n| map.get(n).unwrap() == &'E',
    )
    .unwrap()
    .into_iter()
    .enumerate()
    .map(|(pos, coord)| (coord, pos as i32))
    .collect();

    let sorted_racetrack = racetrack.iter().sorted_by_key(|(_coord, &pos)| pos);

    Some(
        sorted_racetrack
            .clone()
            .enumerate()
            .flat_map(|(i, (&coord, &pos))| {
                // this could be optimised by checking only elements in the racetrack from current to end
                sorted_racetrack
                    .clone()
                    .skip(i)
                    .filter_map(|(cheat_coord, &cheat_pos)| {
                        let hack_duration = distance(&coord, cheat_coord) as i32;
                        if cheat_pos > pos
                            && hack_duration <= 20
                            && cheat_pos - (pos + hack_duration) >= MIN_SAVE_PART_2
                        {
                            return Some((coord, cheat_coord));
                        }
                        None
                    })
                    .unique()
                    .collect::<Vec<_>>()
            })
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(285));
    }
}

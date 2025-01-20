use gxhash::{HashMap, HashSet, HashSetExt};

use pathfinding::{
    matrix::{directions::DIRECTIONS_4, Matrix},
    prelude::dfs,
};

advent_of_code::solution!(20);

#[cfg(test)]
mod constants {
    pub const HACK_TIME_PART_1: i32 = 2;
    pub const MIN_SAVING_PART_1: i32 = 12;
    pub const HACK_TIME_PART_2: i32 = 20;
    pub const MIN_SAVING_PART_2: i32 = 50;
}

#[cfg(not(test))]
mod constants {
    pub const HACK_TIME_PART_1: i32 = 2;
    pub const MIN_SAVING_PART_1: i32 = 100;
    pub const HACK_TIME_PART_2: i32 = 20;
    pub const MIN_SAVING_PART_2: i32 = 100;
}

use constants::*;

fn parse_input(input: &str) -> Matrix<char> {
    Matrix::from_rows(input.lines().map(|l| l.chars())).unwrap()
}

fn distance(c1: &(usize, usize), c2: &(usize, usize)) -> usize {
    c1.0.abs_diff(c2.0) + c1.1.abs_diff(c2.1)
}

fn count_possible_cheats(map: &Matrix<char>, max_hack_time: i32, min_saving: i32) -> usize {
    let start = map.items().find(|(_, &v)| v == 'S').unwrap().0;
    let racetrack = dfs(
        start,
        |&n| {
            map.neighbours(n, false)
                .filter(|&n| map.get(n).unwrap() != &'#')
        },
        |&n| map.get(n).unwrap() == &'E',
    )
    .unwrap();

    let racetrack_positions: HashMap<&(usize, usize), i32> = racetrack
        .iter()
        .enumerate()
        .map(|(pos, coord)| (coord, pos as i32))
        .collect();

    let num_cheats = racetrack
        .iter()
        .enumerate()
        .map(|(start_pos, start_coord)| {
            let mut cheats = HashSet::with_capacity(10000);
            racetrack
                .iter()
                .skip(start_pos + 1)
                .for_each(|target_coord| {
                    let distance = distance(start_coord, target_coord) as i32;
                    if distance <= max_hack_time
                        && racetrack_positions[target_coord] - (start_pos as i32 + distance)
                            >= min_saving
                    {
                        cheats.insert((start_coord, target_coord));
                    }
                });
            cheats.len()
        })
        .sum();
    num_cheats
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = parse_input(input);
    let start = map.items().find(|(_, &v)| v == 'S').unwrap().0;
    let racetrack: HashMap<(usize, usize), i32> = dfs(
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
                                    cheat_pos - (pos + HACK_TIME_PART_1) >= MIN_SAVING_PART_1
                                })
                            })
                    })
                    .collect::<Vec<_>>()
            })
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = parse_input(input);
    let num_cheats = count_possible_cheats(&map, HACK_TIME_PART_2, MIN_SAVING_PART_2);

    Some(num_cheats)
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

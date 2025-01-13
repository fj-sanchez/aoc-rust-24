use std::fmt::Display;

use itertools::Itertools;
use num::integer::gcd;
use pathfinding::matrix::Matrix;

advent_of_code::solution!(8);

fn parse_input(input: &str) -> Matrix<char> {
    Matrix::from_rows(input.lines().map(|line| line.chars())).unwrap()
}

fn _print_grid<T: Display>(grid: &Matrix<T>) {
    for r in grid.iter() {
        for x in r {
            print!("{x}");
        }
        println!();
    }
    println!();
}

fn get_antinodes<F>(antennas: &Matrix<char>, antinode_calculator: F) -> Vec<(usize, usize)>
where
    F: Fn(Position, Position, &Matrix<char>) -> Vec<Position>,
{
    let antinodes: Vec<_> = antennas
        .items()
        .filter(|&(_, &freq)| freq != '.')
        .into_group_map_by(|(_, &freq)| freq)
        .into_iter()
        .flat_map(|(_freq, antennas_group)| {
            // println!("For freq={_freq}:");
            antennas_group
                .into_iter()
                .tuple_combinations()
                .flat_map(|((a, _), (b, _))| antinode_calculator(a, b, antennas))
                .collect::<Vec<_>>()
        })
        .collect();
    antinodes
}

type Position = (usize, usize);

fn calculate_antinodes(
    (ax, ay): Position,
    (bx, by): Position,
    antennas: &Matrix<char>,
) -> Vec<(usize, usize)> {
    let mut antinodes = Vec::new();

    if let (Some(ab_x), Some(ab_y)) = ((2 * bx).checked_sub(ax), (2 * by).checked_sub(ay)) {
        if antennas.within_bounds((ab_x, ab_y)) {
            antinodes.push((ab_x, ab_y));
        }
    }
    if let (Some(ba_x), Some(ba_y)) = ((2 * ax).checked_sub(bx), (2 * ay).checked_sub(by)) {
        if antennas.within_bounds((ba_x, ba_y)) {
            antinodes.push((ba_x, ba_y));
        }
    }
    antinodes
}

fn calculate_antinodes_2(
    (ax, ay): Position,
    (bx, by): Position,
    antennas: &Matrix<char>,
) -> Vec<(usize, usize)> {
    let (ax, ay) = (ax as i32, ay as i32);
    let (bx, by) = (bx as i32, by as i32);

    let (dx, dy) = ((bx - ax), (by - ay));
    let gcd = gcd(dx.abs(), dy.abs());
    let (dx, dy) = (dx / gcd, dy / gcd);

    // Find starting point by moving backwards until we hit x=0 or y=0
    let mut t = 0;
    while ax + (t - 1) * dx >= 0 && ay + (t - 1) * dy >= 0 {
        t -= 1;
    }
    let (sx, sy) = (ax + t * dx, ay + t * dy);

    (sx..antennas.columns as i32)
        .step_by(dx as usize)
        .enumerate()
        .map(|(i, x)| (x, sy + i as i32 * dy))
        .filter(|&(x, y)| antennas.within_bounds((x as usize, y as usize)))
        .map(|(x, y)| (x as usize, y as usize))
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let antennas = parse_input(input);
    let antinodes = get_antinodes(&antennas, calculate_antinodes);

    Some(antinodes.into_iter().unique().count())
}
pub fn part_two(input: &str) -> Option<usize> {
    let antennas = parse_input(input);
    let antinodes = get_antinodes(&antennas, calculate_antinodes_2);
    Some(antinodes.into_iter().unique().count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}

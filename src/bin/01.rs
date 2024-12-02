use itertools::Itertools;
use nom::{
    character::complete::{space1, u32},
    sequence::separated_pair,
    IResult,
};

advent_of_code::solution!(1);

fn left_right_coordinates(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(u32, space1, u32)(input)
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left_coords = Vec::default();
    let mut right_coords = Vec::default();

    for line in input.lines() {
        let (_, (left_coord, right_coord)) = left_right_coordinates(line).unwrap();
        left_coords.push(left_coord);
        right_coords.push(right_coord);
    }
    (left_coords, right_coords)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (left_coords, right_coords) = parse_input(input);
    Some(
        left_coords
            .iter()
            .sorted()
            .zip(right_coords.iter().sorted())
            .map(|(&left_coord, &right_coord)| left_coord.abs_diff(right_coord))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = parse_input(input);
    let right_counts = right.iter().counts();
    Some(
        left.iter()
            .map(|num| (*right_counts.get(num).unwrap_or(&0) as u32) * num)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}

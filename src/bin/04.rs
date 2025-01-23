advent_of_code::solution!(4);

use itertools::Itertools;
use pathfinding::matrix::{
    directions::{self},
    Matrix,
};

fn parse_input(input: &str) -> Matrix<char> {
    Matrix::from_rows(input.lines().map(|l| l.chars())).unwrap()
}

fn is_word_in_direction(
    letters: &Matrix<char>,
    coordinate: &(usize, usize),
    direction: &(isize, isize),
    word: &str,
) -> bool {
    letters
        .get(*coordinate)
        .is_some_and(|&c| word.starts_with(c))
        && (word.len() == 1
            || letters
                .move_in_direction(*coordinate, *direction)
                .is_some_and(|next_coord| {
                    is_word_in_direction(letters, &next_coord, direction, &word[1..])
                }))
}

pub fn part_one(input: &str) -> Option<u32> {
    let word = "XMAS";
    let letters = parse_input(input);

    let starting_coords: Vec<_> = letters
        .keys()
        .filter(|&(row, column)| letters.get((row, column)) == Some(&'X'))
        .collect();

    let matches: Vec<_> = starting_coords
        .iter()
        .cartesian_product(directions::DIRECTIONS_8)
        .filter(|(coord, dir)| is_word_in_direction(&letters, coord, dir, word))
        .collect();

    Some(matches.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let word = "MAS";
    let letters = parse_input(input);

    let starting_coords: Vec<_> = letters
        .keys()
        .filter(|&(row, column)| letters.get((row, column)) == Some(&'M'))
        .collect();

    let matches: Vec<_> = starting_coords
        .iter()
        .cartesian_product([
            directions::NE,
            directions::SE,
            directions::SW,
            directions::NW,
        ])
        .filter(|&(coord, dir)| is_word_in_direction(&letters, coord, &dir, word))
        .map(|(&coord, dir)| letters.move_in_direction(coord, dir))
        .duplicates()
        .collect();

    Some(matches.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9));
    }
}

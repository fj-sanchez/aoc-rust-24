use itertools::Itertools;
use pathfinding::{
    matrix::{
        directions::{self},
        Matrix,
    },
    prelude::{astar_bag, dijkstra},
};
use std::fmt;

advent_of_code::solution!(16);

type Position = (usize, usize);
type Direction = (isize, isize);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MapElement {
    Start,
    Free,
    Wall,
    End,
}

impl fmt::Display for MapElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MapElement::Start => "S",
                MapElement::Wall => "#",
                MapElement::Free => ".",
                MapElement::End => "E",
            }
        )
    }
}

fn _print_grid(grid: &Matrix<MapElement>) {
    for r in grid.iter() {
        for x in r {
            print!("{x}");
        }
        println!();
    }
    println!();
}

fn parse_input(input: &str) -> Matrix<MapElement> {
    Matrix::from_iter(input.lines().map(|l| {
        l.chars().map(|c| match c {
            'S' => MapElement::Start,
            '.' => MapElement::Free,
            '#' => MapElement::Wall,
            'E' => MapElement::End,
            _ => panic!(),
        })
    }))
}

fn neighbours(
    position: Position,
    direction: Direction,
    map: &Matrix<MapElement>,
) -> Vec<((Position, Direction), u32)> {
    let orthogonal_moves = match direction {
        (y, 0) => [(0, -y), (0, y)],
        (0, x) => [(-x, 0), (x, 0)],
        _ => panic!("Invalid direction"),
    };

    let mut next_nodes = Vec::with_capacity(3);

    for &dir in &orthogonal_moves {
        let next_orthogonal = map.move_in_direction(position, dir).unwrap();
        if map.get(next_orthogonal).unwrap() != &MapElement::Wall {
            next_nodes.push(((position, dir), 1000));
        }
    }

    let next_straight = map.move_in_direction(position, direction).unwrap();
    if map.get(next_straight).unwrap() != &MapElement::Wall {
        next_nodes.push(((next_straight, direction), 1));
    }

    next_nodes
}

fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> u32 {
    (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_input(input);
    // this can be hard-coded as per problem definition
    let start = (map.rows - 2, 1);
    let end = (1, map.columns - 2);

    Some(
        dijkstra(
            &(start, directions::E),
            |&(pos, dir)| neighbours(pos, dir, &map),
            |(pos, _)| *pos == end,
        )
        .unwrap()
        .1,
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = parse_input(input);
    // this can be hard-coded as per problem definition
    let start = (map.rows - 2, 1);
    let end = (1, map.columns - 2);

    astar_bag(
        &(start, directions::E),
        |&(pos, dir)| neighbours(pos, dir, &map),
        |&(pos, _)| manhattan_distance(pos, start),
        |(pos, _)| *pos == end,
    )
    .map(|(solutions, _)| {
        solutions
            .into_iter()
            .flat_map(|sol| sol.into_iter().map(|(c, _)| c))
            .unique()
            .count()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(64));
    }
}

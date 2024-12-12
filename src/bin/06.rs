use core::fmt;
use std::collections::HashSet;

use pathfinding::matrix::{
    directions::{self, DIRECTIONS_4},
    Matrix,
};

advent_of_code::solution!(6);

type Position = (usize, usize);
type Direction = (isize, isize);

#[derive(Debug, Clone, Copy)]
struct Guard {
    position: Position,
    direction: Direction,
}

#[derive(Debug, PartialEq, Eq)]
enum GridKind {
    Block,
    Free,
    Guard(char),
}

impl GridKind {
    fn parse_grid_kind(c: char) -> GridKind {
        match c {
            '#' => Self::Block,
            '.' => Self::Free,
            '^' | '>' | 'v' | '<' => Self::Guard(c),
            _ => panic!("Error parsing grid element: {}", c),
        }
    }
}

impl fmt::Display for GridKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GridKind::Block => "#",
                GridKind::Free => ".",
                GridKind::Guard(_) => "o",
            }
        )
    }
}

impl Guard {
    fn dir_from_char(g: &GridKind) -> Direction {
        match g {
            GridKind::Guard('^') => directions::N,
            GridKind::Guard('>') => directions::E,
            GridKind::Guard('v') => directions::S,
            GridKind::Guard('<') => directions::W,
            _ => panic!("Error parsing direction: {:?}", g),
        }
    }

    fn turn_right(mut self) {
        self.direction = DIRECTIONS_4
            .iter()
            .cycle()
            .skip_while(|&&p| p != self.direction)
            .next()
            .unwrap()
            .to_owned();
    }
}

fn parse_input(input: &str) -> (Matrix<GridKind>, Guard) {
    let mut grid = Matrix::from_rows(
        input
            .lines()
            .map(|l| l.chars().map(GridKind::parse_grid_kind)),
    )
    .unwrap();

    let guard: Guard = grid
        .items_mut()
        .find(|(_, kind)| matches!(kind, GridKind::Guard(_)))
        .map(|(position, kind)| {
            let guard = Guard {
                position,
                direction: Guard::dir_from_char(kind),
            };
            *kind = GridKind::Free;
            guard
        })
        .unwrap();

    (grid, guard)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, mut guard) = parse_input(input);
    let mut visited = HashSet::new();
    
    visited.insert(guard.position);
    while let Some(next_pos) = grid.move_in_direction(guard.position, guard.direction) {
        match grid.get(next_pos).unwrap() {
            GridKind::Block => guard.turn_right(),
            GridKind::Free => guard.position = next_pos,
            _ => panic!(),
        }
        visited.insert(guard.position);
    }

    Some(visited.len() as u32)
}

fn print_grid(grid: Matrix<GridKind>) {
    for r in grid.iter() {
        for x in r {
            print!("{x}");
        }
        println!();
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

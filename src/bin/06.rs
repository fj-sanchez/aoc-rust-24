use core::fmt;

use gxhash::{HashSet, HashSetExt};
use pathfinding::matrix::{directions, Matrix};

advent_of_code::solution!(6);

type Position = (usize, usize);
type Direction = (isize, isize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Guard {
    position: Position,
    direction: Direction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            directions::N => directions::E,
            directions::E => directions::S,
            directions::S => directions::W,
            directions::W => directions::N,
            _ => panic!("Invalid guard direction"),
        };
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

fn _print_grid(grid: Matrix<GridKind>) {
    for r in grid.iter() {
        for x in r {
            print!("{x}");
        }
        println!();
    }
}

// this is only to optimise the runtime of part 2
#[inline]
fn guard_to_u32(guard: Guard) -> u32 {
    (guard.position.0 as u32)
        + ((guard.position.1 as u32) << 8)
        + ((to2bit(guard.direction.0) + (to2bit(guard.direction.1) << 8)) << 16)
}

#[inline]
fn to2bit(x: isize) -> u32 {
    match x {
        _ if x == 0 => 0,
        _ if x > 0 => 1,
        _ => 2,
    }
}

fn has_cycle(guard: &Guard, grid: &Matrix<GridKind>, obstacle_pos: Position) -> bool {
    let mut guard = *guard;

    let mut visited = HashSet::with_capacity(10000);

    visited.insert(guard_to_u32(guard));
    while let Some(next_pos) = grid.move_in_direction(guard.position, guard.direction) {
        let next_grid = if next_pos == obstacle_pos {
            &GridKind::Block
        } else {
            grid.get(next_pos).unwrap()
        };
        match next_grid {
            GridKind::Block => guard.turn_right(),
            GridKind::Free => guard.position = next_pos,
            _ => panic!(),
        }
        if !visited.insert(guard_to_u32(guard)) {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<usize> {
    let (grid, mut guard) = parse_input(input);
    let mut visited = HashSet::with_capacity(10000);

    visited.insert(guard.position);
    while let Some(next_pos) = grid.move_in_direction(guard.position, guard.direction) {
        match grid.get(next_pos).unwrap() {
            GridKind::Block => guard.turn_right(),
            GridKind::Free => guard.position = next_pos,
            _ => panic!(),
        }
        visited.insert(guard.position);
    }

    Some(visited.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (grid, guard) = parse_input(input);
    let mut dummy_guard = guard;
    let mut guard_path = HashSet::with_capacity(10000);

    while let Some(next_pos) = grid.move_in_direction(dummy_guard.position, dummy_guard.direction) {
        match grid.get(next_pos).unwrap() {
            GridKind::Block => dummy_guard.turn_right(),
            GridKind::Free => dummy_guard.position = next_pos,
            _ => panic!(),
        }
        guard_path.insert(dummy_guard.position);
    }

    Some(
        guard_path
            .iter()
            .filter(|&&obs| has_cycle(&guard, &grid, obs))
            .count(),
    )
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
        assert_eq!(result, Some(6));
    }
}

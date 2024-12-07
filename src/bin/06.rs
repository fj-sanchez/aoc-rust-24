advent_of_code::solution!(6);

use pathfinding::matrix::{directions::{self, DIRECTIONS_4}, Matrix};

type Position = (usize, usize);
type Direction = (isize, isize);

#[derive(Debug)]
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
        self.direction = DIRECTIONS_4.iter().cycle().skip_while(|p| p!=self.direction).next().unwrap();
    }
}

fn parse_input(input: &str) -> (Matrix<GridKind>, Guard) {
    let grid = Matrix::from_rows(
        input
            .lines()
            .map(|l| l.chars().map(GridKind::parse_grid_kind)),
    )
    .unwrap();

    let guard = grid
        .items()
        .find(|(_, kind)| matches!(kind, GridKind::Guard(_)))
        .map(|(position, guard_kind)| Guard {
            position,
            direction: Guard::dir_from_char(guard_kind),
        })
        .unwrap();

    (grid, guard)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, guard) = parse_input(input);

    Some(0)
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

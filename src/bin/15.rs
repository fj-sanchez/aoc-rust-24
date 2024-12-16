use std::fmt;

use nom::Map;
use pathfinding::{
    grid::Grid,
    matrix::{directions, Matrix},
};

advent_of_code::solution!(15);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MapElement {
    Wall,
    Free,
    Box,
    Robot,
}

impl fmt::Display for MapElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MapElement::Wall => "#",
                MapElement::Free => ".",
                MapElement::Box => "O",
                MapElement::Robot => "@",
            }
        )
    }
}

type Position = (usize, usize);
type Move = (isize, isize);

#[derive(Debug)]
struct Robot {
    position: Position,
}

fn parse_input(input: &str) -> (Matrix<MapElement>, Vec<Move>) {
    let (map_lines, move_lines): (Vec<&str>, Vec<&str>) =
        input.lines().partition(|l| l.starts_with('#'));

    let map = Matrix::from_rows(map_lines.iter().map(|l| {
        l.chars().map(|c| match c {
            '#' => MapElement::Wall,
            'O' => MapElement::Box,
            '.' => MapElement::Free,
            '@' => MapElement::Robot,
            _ => panic!(),
        })
    }))
    .unwrap();

    dbg!(&move_lines);
    let moves = move_lines
        .iter()
        .filter(|line| !line.is_empty())
        .flat_map(|line| {
            line.chars().map(|move_| match move_ {
                '^' => directions::N,
                '>' => directions::E,
                'v' => directions::S,
                '<' => directions::W,
                _ => panic!(),
            })
        })
        .collect();

    (map, moves)
}

fn try_move(position: Position, move_: Move, map: &mut Matrix<MapElement>) -> Option<Position> {
    // let position_idx = map.idx(position);
    map.move_in_direction(position, move_).and_then(|next_pos| {
        // let next_pos_idx = map.idx(next_pos);
        match map.get(next_pos) {
            Some(MapElement::Wall) => None,
            Some(MapElement::Box) => try_move(next_pos, move_, map).map(|_| {
                map.swap(position, next_pos);
                next_pos
            }),
            Some(MapElement::Free) => {
                map.swap(position, next_pos);
                Some(next_pos)
            }
            _ => panic!(),
        }
    })
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

fn move_repr(&move_: &Move) -> char {
    match move_ {
        directions::N => '^',
        directions::E => '>',
        directions::S => 'v',
        directions::W => '<',
        _ => panic!(),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut map, moves) = parse_input(input);

    let (mut robot_pos, _) = map
        .items()
        .find(|&(_, element)| element == &MapElement::Robot)
        .unwrap();

    // println!("Initial state:");
    // _print_grid(&map);

    moves.iter().for_each(|&move_| {
        // println!("Move: {:?}:", move_repr(&move_));
        if let Some(new_robot_pos) = try_move(robot_pos, move_, &mut map) {
            robot_pos = new_robot_pos;
        }
        // _print_grid(&map);
    });

    Some(
        map.items()
            .filter(|(_, &elem)| elem == MapElement::Box)
            .map(|((y, x), _)| (100 * y + x) as u32)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2028));
        // let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

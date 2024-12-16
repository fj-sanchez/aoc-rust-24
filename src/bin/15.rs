use std::fmt;

use pathfinding::matrix::{directions, Matrix};

advent_of_code::solution!(15);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MapElement {
    Wall,
    Free,
    Box,
    Robot,
    BoxLeft,
    BoxRight,
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
                MapElement::BoxLeft => "[",
                MapElement::BoxRight => "]",
            }
        )
    }
}

type Position = (usize, usize);
type Move = (isize, isize);

fn parse_input(input: &str, is_wide: bool) -> (Matrix<MapElement>, Vec<Move>) {
    let (map_lines, move_lines): (Vec<&str>, Vec<&str>) =
        input.lines().partition(|l| l.starts_with('#'));

    let map = if !is_wide {
        Matrix::from_rows(map_lines.iter().map(|l| {
            l.chars().map(|c| match c {
                '#' => MapElement::Wall,
                'O' => MapElement::Box,
                '.' => MapElement::Free,
                '@' => MapElement::Robot,
                _ => panic!(),
            })
        }))
    } else {
        Matrix::from_rows(map_lines.iter().map(|l| {
            l.chars().flat_map(|c| match c {
                '#' => [MapElement::Wall, MapElement::Wall],
                'O' => [MapElement::BoxLeft, MapElement::BoxRight],
                '.' => [MapElement::Free, MapElement::Free],
                '@' => [MapElement::Robot, MapElement::Free],
                _ => panic!(),
            })
        }))
    }
    .unwrap();

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

fn try_move(
    position: Position,
    move_: Move,
    map: &mut Matrix<MapElement>,
    update: bool,
) -> Option<Position> {
    map.move_in_direction(position, move_)
        .and_then(|next_pos| match map.get(next_pos) {
            Some(MapElement::Wall) => None,
            Some(MapElement::Box | MapElement::BoxLeft | MapElement::BoxRight) => {
                try_move(next_pos, move_, map, update).map(|_| {
                    if update {
                        map.swap(position, next_pos);
                    }
                    next_pos
                })
            }
            Some(MapElement::Free) => {
                if update {
                    map.swap(position, next_pos);
                }
                Some(next_pos)
            }
            _ => panic!(),
        })
}

fn try_move_wide(
    position: Position,
    move_: Move,
    map: &mut Matrix<MapElement>,
    update: bool,
) -> Option<Position> {
    map.move_in_direction(position, move_)
        .and_then(|next_pos| match (map.get(next_pos), move_) {
            (_, directions::E | directions::W) => try_move(position, move_, map, true),
            (Some(MapElement::Wall), _) => None,
            (Some(box_side @ (MapElement::BoxLeft | MapElement::BoxRight)), _) => {
                let side_dir = match box_side {
                    MapElement::BoxLeft => directions::E,
                    MapElement::BoxRight => directions::W,
                    _ => panic!(),
                };
                let next_pos_side = map.move_in_direction(next_pos, side_dir).unwrap();
                let this = try_move_wide(next_pos, move_, map, false);
                let side = try_move_wide(next_pos_side, move_, map, false);

                if this.is_some() && side.is_some() {
                    if update {
                        try_move_wide(next_pos, move_, map, true);
                        try_move_wide(next_pos_side, move_, map, true);
                        map.swap(position, next_pos);
                    }
                    return Some(next_pos);
                }
                None
            }
            (Some(MapElement::Free), _) => {
                if update {
                    map.swap(position, next_pos);
                }
                Some(next_pos)
            }
            _ => panic!("Elem={:?} dir={:?}", map.get(next_pos), move_),
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

fn _move_repr(&move_: &Move) -> char {
    match move_ {
        directions::N => '^',
        directions::E => '>',
        directions::S => 'v',
        directions::W => '<',
        _ => panic!(),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut map, moves) = parse_input(input, false);

    let (mut robot_pos, _) = map
        .items()
        .find(|&(_, element)| element == &MapElement::Robot)
        .unwrap();

    // println!("Initial state:");
    // _print_grid(&map);

    moves.iter().for_each(|&move_| {
        // println!("Move: {:?}:", _move_repr(&move_));
        if let Some(new_robot_pos) = try_move(robot_pos, move_, &mut map, true) {
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
    let (mut map, moves) = parse_input(input, true);

    let (mut robot_pos, _) = map
        .items()
        .find(|&(_, element)| element == &MapElement::Robot)
        .unwrap();

    // println!("Initial state:");
    // _print_grid(&map);

    moves.iter().for_each(|&move_| {
        // println!("Move: {:?}:", _move_repr(&move_));
        if let Some(new_robot_pos) = try_move_wide(robot_pos, move_, &mut map, true) {
            robot_pos = new_robot_pos;
        }
        // _print_grid(&map);
    });

    Some(
        map.items()
            .filter(|(_, &elem)| elem == MapElement::BoxLeft)
            .map(|((y, x), _)| (100 * y + x) as u32)
            .sum(),
    )
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
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}

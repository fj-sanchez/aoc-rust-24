use pathfinding::{
    matrix::Matrix,
    prelude::{count_paths, dfs_reach},
};

advent_of_code::solution!(10);

fn parse_input(input: &str) -> Matrix<u32> {
    Matrix::from_rows(
        input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap())),
    )
    .unwrap()
}

fn adjacent_higher_positions<'map>(
    map: &'map Matrix<u32>,
    coord: (usize, usize),
    val: &'map u32,
) -> Vec<((usize, usize), &'map u32)> {
    map.neighbours(coord, false)
        .map(|c| (c, map.get(c).unwrap()))
        .filter(|&(_next_coord, &next_val)| next_val == (val + 1))
        .collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = parse_input(input);

    Some(
        map.items()
            .filter(|(_coord, &val)| val == 0)
            .flat_map(|(cur_coord, cur_val)| {
                dfs_reach((cur_coord, cur_val), |&(coord, val)| {
                    adjacent_higher_positions(&map, coord, val)
                })
                .filter(|(_coord, &val)| val == 9)
            })
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = parse_input(input);

    Some(
        map.items()
            .filter(|(_coord, &val)| val == 0)
            .map(|(cur_coord, cur_val)| {
                count_paths(
                    (cur_coord, cur_val),
                    |&(coord, val)| adjacent_higher_positions(&map, coord, val),
                    |(_coord, &val)| val == 9,
                )
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}

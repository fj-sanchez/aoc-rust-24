use std::collections::BTreeSet;

use itertools::Itertools;
use pathfinding::{
    grid::Grid,
    matrix::{directions, Matrix},
    utils::move_in_direction,
};

advent_of_code::solution!(12);

fn parse_input(input: &str) -> Matrix<char> {
    Matrix::from_rows(input.lines().map(|l| l.chars())).unwrap()
}

fn count_sides(coord: &(usize, usize), plot: &Grid) -> usize {
    const CORNER_ENDS: [[(isize, isize); 3]; 4] = [
        [directions::E, directions::S, directions::SE],
        [directions::W, directions::S, directions::SW],
        [directions::S, directions::E, directions::SE],
        [directions::N, directions::E, directions::NE],
    ];

    CORNER_ENDS
        .iter()
        .filter(|corner_end| {
            let dimensions = (plot.width, plot.height);
            let edge = move_in_direction(*coord, corner_end[0], dimensions).unwrap_or_default();
            let corner1 = move_in_direction(*coord, corner_end[1], dimensions).unwrap_or_default();
            let corner2 = move_in_direction(*coord, corner_end[2], dimensions).unwrap_or_default();

            !plot.has_vertex(edge) && (!plot.has_vertex(corner1) || plot.has_vertex(corner2))
        })
        .count()
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = parse_input(input);

    let plants: BTreeSet<char> = map
        .values()
        .filter(|&&p| p != '0')
        .unique()
        .cloned()
        .collect();

    let mut result = 0;

    for plant in &plants {
        let mut plant_coords = Grid::from_iter(
            map.items()
                .filter(|&(_, p)| p == plant)
                .map(|(coord, _)| coord),
        );

        while let Some(coord) = plant_coords.iter().next() {
            let garden_plot = Grid::from_iter(plant_coords.bfs_reachable(coord, |_| true));

            result += garden_plot.vertices_len()
                * ((garden_plot.vertices_len() * 4) - 2 * garden_plot.edges().count());

            garden_plot.iter().for_each(|v| {
                plant_coords.remove_vertex(v);
            });
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = parse_input(input);

    let plants: BTreeSet<char> = map
        .values()
        .filter(|&&p| p != '0')
        .unique()
        .cloned()
        .collect();

    let mut result = 0;

    for plant in &plants {
        let mut plant_coords = Grid::from_iter(
            map.items()
                .filter(|&(_, p)| p == plant)
                .map(|(coord, _)| coord),
        );

        while let Some(coord) = plant_coords.iter().next() {
            let garden_plot = Grid::from_iter(plant_coords.bfs_reachable(coord, |_| true));

            let sides: usize = garden_plot
                .iter()
                .map(|v| count_sides(&v, &garden_plot))
                .sum();

            result += garden_plot.vertices_len() * sides;
            garden_plot.iter().for_each(|v| {
                plant_coords.remove_vertex(v);
            });
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(436));
    }
}

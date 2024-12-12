use std::collections::BTreeSet;

use itertools::Itertools;
use pathfinding::{grid::Grid, matrix::Matrix};

advent_of_code::solution!(12);

fn parse_input(input: &str) -> Matrix<char> {
    Matrix::from_rows(input.lines().map(|l| l.chars())).unwrap()
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = parse_input(input);

    let plot_types: BTreeSet<char> = map.values().unique().cloned().collect();
    let mut result = 0;

    for plot_type in &plot_types {
        let plot_coord: BTreeSet<(usize, usize)> = map
            .items()
            .filter(|&(_, v)| v == plot_type)
            .map(|(c, _)| c)
            .collect();
        let mut plot = Grid::from_iter(plot_coord.iter().cloned());

        while let Some(start) = plot.iter().next() {
            let region = plot.bfs_reachable(start, |_| true);
            let g = Grid::from_iter(region.into_iter());
            result += g.vertices_len() * ((g.vertices_len() * 4) - 2 * g.edges().count());
            g.iter().for_each(|v| {
                plot.remove_vertex(v);
            });
        }
    }

    Some(result)
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
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(1206));
    }
}

use std::collections::BTreeSet;

use gxhash::{HashMap, HashMapExt, HashSet};
use itertools::Itertools;
use pathfinding::prelude::maximal_cliques_collect;

advent_of_code::solution!(23);

type Edges<'a> = Vec<(&'a str, &'a str)>;
type Connections<'a> = HashMap<&'a str, HashSet<&'a str>>;

fn parse_input(input: &str) -> (Edges, Connections) {
    let edges: Edges = input
        .lines()
        .map(|l| l.split("-").collect_tuple().unwrap())
        .collect();

    let connections: Connections = edges.iter().fold(HashMap::new(), |mut acc, (e1, e2)| {
        acc.entry(e1).or_default().insert(e2);
        acc.entry(e2).or_default().insert(e1);
        acc
    });

    (edges, connections)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (edges, connections) = parse_input(input);

    Some(
        edges
            .iter()
            .flat_map(|(node1, node2)| {
                (connections[node1].intersection(&connections[node2]))
                    .filter_map(move |node3| {
                        if node1.starts_with('t')
                            || node2.starts_with('t')
                            || node3.starts_with('t')
                        {
                            return Some(BTreeSet::from_iter([node1, node2, node3]));
                        }
                        None
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<HashSet<_>>()
            .len(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let (_edges, connections) = parse_input(input);

    let cliques = maximal_cliques_collect(connections.keys(), &mut |&v1, &v2| {
        connections[v1].contains(v2)
    });

    Some(
        cliques
            .iter()
            .max_by_key(|n| n.len())
            .unwrap()
            .iter()
            .sorted()
            .join(","),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}

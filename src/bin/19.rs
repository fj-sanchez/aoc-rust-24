use std::collections::BTreeMap;

use cached::proc_macro::cached;

advent_of_code::solution!(19);

type AvailableTowels = BTreeMap<char, Vec<String>>;

fn parse_input(input: &str) -> (AvailableTowels, Vec<String>) {
    let mut available = input
        .lines()
        .take(1)
        .flat_map(|l| {
            l.split(", ")
                .map(|s| (s.chars().next().unwrap(), s.to_string()))
        })
        .fold(AvailableTowels::new(), |mut acc, (k, v)| {
            (*acc.entry(k).or_default()).push(v);
            acc
        });

    available
        .iter_mut()
        .for_each(|(_k, v)| v.sort_by_key(|s| s.len()));

    let patterns = input.lines().skip(2).map(|s| s.to_string()).collect();

    (available, patterns)
}

fn is_possible(pattern: &str, towels: &AvailableTowels) -> bool {
    let Some(next_stripe) = pattern.chars().next() else {
        return true;
    };

    if let Some(towels_with_stripe) = towels.get(&next_stripe) {
        for towel in towels_with_stripe {
            if pattern.starts_with(towel) && is_possible(&pattern[towel.len()..], towels) {
                return true;
            }
        }
    }

    false
}

#[cached(key = "String", convert = r#"{ format!("{}", pattern) }"#)]
fn possible_arrangements(pattern: &str, towels: &AvailableTowels) -> u64 {
    let Some(next_stripe) = pattern.chars().next() else {
        return 1;
    };

    if let Some(towels_with_stripe) = towels.get(&next_stripe) {
        return towels_with_stripe
            .iter()
            .filter(|&towel| pattern.starts_with(towel))
            .map(|towel| possible_arrangements(&pattern[towel.len()..], towels))
            .sum();
    }
    0
}

pub fn part_one(input: &str) -> Option<usize> {
    let (available, patterns) = parse_input(input);

    Some(
        patterns
            .iter()
            .filter(|pattern| is_possible(pattern, &available))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (available, patterns) = parse_input(input);

    Some(
        patterns
            .iter()
            .map(|pattern| possible_arrangements(pattern, &available))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}

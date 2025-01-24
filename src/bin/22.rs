use gxhash::{HashMap, HashMapExt};

use itertools::Itertools;

advent_of_code::solution!(22);

#[inline]
fn mix(secret: u64, value: u64) -> u64 {
    secret ^ value
}

#[inline]
fn prune(secret: u64) -> u64 {
    secret % 16777216
}

fn next_secret(secret: u64) -> u64 {
    let secret = prune(mix(secret, secret << 6));
    let secret = prune(mix(secret, secret >> 5));
    prune(mix(secret, secret << 11))
}

pub fn part_one(input: &str) -> Option<u64> {
    let initial_numbers: Vec<u64> = input.lines().map(|l| l.parse().unwrap()).collect();

    Some(
        initial_numbers
            .into_iter()
            .map(|secret| (0..2000).fold(secret, |acc, _| next_secret(acc)))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let initial_numbers: Vec<u64> = input.lines().map(|l| l.parse().unwrap()).collect();
    let num_buyers = initial_numbers.len();

    let sequences_values: HashMap<i32, Vec<i8>> = initial_numbers
        .into_iter()
        .map(|secret| {
            (0..2000).fold(vec![(secret, (secret % 10) as i8, 0)], |mut acc, _| {
                let (secret, bananas, _price_change) = *acc.last().unwrap();
                let next_secret = next_secret(secret);
                let next_bananas = (next_secret % 10) as i8;

                acc.push((next_secret, next_bananas, bananas - next_bananas));
                acc
            })
        })
        .enumerate()
        .fold(
            HashMap::new(),
            |mut acc, (buyer_idx, buyer_bananas_and_seq)| {
                buyer_bananas_and_seq
                    .into_iter()
                    .tuple_windows()
                    .for_each(|(a, b, c, d)| {
                        // convert sequence into single i32 for quicker hashing/look-up
                        let seq = i32::from_ne_bytes([a.2 as u8, b.2 as u8, c.2 as u8, d.2 as u8]);
                        let entry = acc.entry(seq).or_insert_with(|| vec![-1; num_buyers]);
                        if entry[buyer_idx] < 0 {
                            entry[buyer_idx] = d.1;
                        };
                    });

                acc
            },
        );

    Some(
        sequences_values
            .into_iter()
            .map(|(seq, bananas)| {
                (
                    seq,
                    bananas
                        .iter()
                        .map(|&x| match x {
                            -1 => 0,
                            _ => x as i64,
                        })
                        .sum::<i64>(),
                )
            })
            .sorted_by_key(|&(_, total_bananas)| total_bananas)
            .last()
            .unwrap()
            .1,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }
}

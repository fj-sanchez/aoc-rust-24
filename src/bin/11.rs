use cached::proc_macro::cached;

advent_of_code::solution!(11);

fn parse_input(input: &str) -> Vec<u64> {
    input
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}

#[cached]
fn process_stone(stone: u64, blinks: u64) -> u64 {
    let digits = stone.checked_ilog10().unwrap_or(0) + 1;

    match (stone, blinks) {
        (_, 0) => 1,
        (0, _) => process_stone(1, blinks - 1),
        (_s, _) if (digits % 2) == 0 => {
            let op = 10_u64.pow(digits / 2);
            process_stone(stone / op, blinks - 1) + process_stone(stone % op, blinks - 1)
        }
        _ => process_stone(stone * 2024, blinks - 1),
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let stones = parse_input(input);

    Some(
        stones
            .into_iter()
            .map(|stone| process_stone(stone, 25))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones = parse_input(input);

    Some(
        stones
            .into_iter()
            .map(|stone| process_stone(stone, 75))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}

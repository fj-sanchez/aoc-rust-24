advent_of_code::solution!(11);

fn parse_input(input: &str) -> Vec<String> {
    input
        .lines()
        .next()
        .map(|l| l.split_ascii_whitespace().map(String::from).collect())
        .unwrap()
}

fn process_stone(stone: &mut String) -> Option<String> {
    let is_even = (stone.len() % 2) == 0;
    let hlf_len = stone.len() / 2;
    match stone.as_str() {
        "0" => stone="1",
        _ if is_even => vec![
            stone[0..(hlf_len)].parse::<u64>().unwrap().to_string(),
            stone[hlf_len..].parse::<u64>().unwrap().to_string(),
        ],
        _ => vec![(stone.parse::<u64>().unwrap() * 2024).to_string()],
    }
    None
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut stones = parse_input(input);

    for _ in 0..25 {
        stones = stones.iter().flat_map(|s|process_stone(s)).collect();
    }

    Some(stones.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut stones = parse_input(input);

    for _ in 0..75 {
        stones = stones.iter().flat_map(|s|process_stone(s)).collect();
    }
    print!("There are {} stones.", stones.len());

    Some(stones.len())
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
        assert_eq!(result, Some(0));
    }
}

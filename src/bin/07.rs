advent_of_code::solution!(7);

fn is_solvable(test_value: u64, rev_nums: &[u64], use_concat: bool) -> bool {
    let &num = rev_nums.first().unwrap();
    if rev_nums.len() == 1 {
        return test_value == num;
    }

    let (r, q) = (test_value % num, test_value / num);
    if r == 0 && is_solvable(q, &rev_nums[1..], use_concat) {
        return true;
    }

    if use_concat {
        let num_digits = 10u64.pow(num.ilog10() + 1);
        let end_equal = (test_value.abs_diff(num) % num_digits) == 0;
        if end_equal && is_solvable(test_value / num_digits, &rev_nums[1..], use_concat) {
            return true;
        }
    }

    if let Some(new_test_value) = test_value.checked_sub(num) {
        return is_solvable(new_test_value, &rev_nums[1..], use_concat);
    }

    false
}

fn parse_equation(line: &str) -> (u64, Vec<u64>) {
    let parts: Vec<&str> = line.split(": ").collect();
    let test_value: u64 = parts[0].parse().unwrap();
    let nums: Vec<u64> = parts[1]
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    (test_value, nums)
}

fn solve(input: &str, use_concat: bool) -> Option<u64> {
    Some(
        input
            .lines()
            .map(parse_equation)
            .map(|(test_value, nums)| (test_value, nums.into_iter().rev().collect::<Vec<_>>()))
            .filter(|(test_value, nums)| is_solvable(*test_value, nums, use_concat))
            .map(|(test_value, _)| test_value)
            .sum(),
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}

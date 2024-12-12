advent_of_code::solution!(7);

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    // Easy to add more operators in the future, e.g.:
    // Subtract,
    // Divide,
    // Modulo,
}

impl Operator {
    fn apply(&self, left: i64, right: i64) -> i64 {
        match self {
            Operator::Add => left + right,
            Operator::Multiply => left * right,
            // Additional operators would be added here
        }
    }
}

fn solve_equation(test_value: i64, nums: &[i64]) -> bool {
    // Generate all possible operator configurations
    let operator_count = nums.len() - 1;

    // Early exit if impossible
    if operator_count == 0 {
        return nums[0] == test_value;
    }

    // Try all possible combinations of operators
    for config in 0..(1 << (2 * operator_count)) {
        let mut operators = vec![Operator::Add; operator_count];

        // Convert the current configuration to operator choices
        for i in 0..operator_count {
            match (config >> (2 * i)) & 3 {
                0 => operators[i] = Operator::Add,
                1 => operators[i] = Operator::Multiply,
                _ => continue, // Skip invalid configurations
            }
        }

        let result = evaluate_expression(nums, &operators);
        if result == test_value {
            return true;
        }
    }

    false
}

fn evaluate_expression(nums: &[i64], operators: &[Operator]) -> i64 {
    let mut result = nums[0];
    for i in 0..operators.len() {
        result = operators[i].apply(result, nums[i + 1]);
    }
    result
}

fn parse_equation(line: &str) -> (i64, Vec<i64>) {
    let parts: Vec<&str> = line.split(": ").collect();
    let test_value: i64 = parts[0].parse().unwrap();
    let nums: Vec<i64> = parts[1]
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    (test_value, nums)
}

pub fn part_one(input: &str) -> Option<i64> {
    Some(
        input
            .lines()
            .map(parse_equation)
            .filter(|(test_value, nums)| solve_equation(*test_value, nums))
            .map(|(test_value, _)| test_value)
            .sum(),
    )
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

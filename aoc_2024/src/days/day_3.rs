use aoc_lib::Solution;
use regex::Regex;

pub struct Day3;

impl Solution for Day3 {
    fn part_1(&self, input: &str) -> String {
        part_1(input).to_string()
    }

    fn part_2(&self, input: &str) -> String {
        part_2(input).to_string()
    }
}

fn part_1(input: &str) -> i32 {
    sum_instructions(input)
}

fn part_2(input: &str) -> i32 {
    let re = Regex::new(r"don't\(\)[\s\S]*?do\(\)").expect("Invalid regex");
    let replaced = re.replace_all(input, "").to_string();
    sum_instructions(&replaced)
}

fn sum_instructions(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex");
    re.captures_iter(input)
        .filter_map(|cap| {
            let num1 = cap[1].parse::<i32>().ok();
            let num2 = cap[2].parse::<i32>().ok();
            if let (Some(num1), Some(num2)) = (num1, num2) {
                Some(num1 * num2)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_should_return_161() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = Day3.part_1(input);

        assert_eq!(result, "161".to_string());
    }

    #[test]
    fn part_2_should_return_48() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = Day3.part_2(input);

        assert_eq!(result, "48".to_string());
    }
}

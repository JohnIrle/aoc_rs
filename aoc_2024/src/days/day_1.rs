use std::collections::HashMap;
use std::iter::zip;

use aoc_lib::Solution;

pub struct Day1;

impl Solution for Day1 {
    fn part_1(&self, input: &str) -> String {
        part_1(input).to_string()
    }

    fn part_2(&self, input: &str) -> String {
        part_2(input).to_string()
    }
}

fn part_1(input: &str) -> i32 {
    let (mut left, mut right) = parse_lines(input);

    left.sort_unstable();
    right.sort_unstable();

    zip(left, right).map(|(a, b)| (a - b).abs()).sum()
}

fn part_2(input: &str) -> i32 {
    let mut frequency_map = HashMap::new();
    let (left, right) = parse_lines(input);

    for num in right {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    left.iter().fold(0, |acc, num| {
        acc + num * frequency_map.get(num).unwrap_or(&0)
    })
}

fn parse_lines(input: &str) -> (Vec<i32>, Vec<i32>) {
    let (left, right): (Vec<_>, Vec<_>) = input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let left = parts.next()?;
            let right = parts.next()?;
            Some((left.parse::<i32>().ok()?, right.parse::<i32>().ok()?))
        })
        .unzip();
    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part1_returns_11_with_example_input() {
        let total = Day1.part_1(INPUT);

        assert_eq!(total, "11".to_string());
    }

    #[test]
    fn part2_returns_31_with_example_input() {
        let total = Day1.part_2(INPUT);

        assert_eq!(total, "31".to_string());
    }
}

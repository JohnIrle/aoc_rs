use aoc_lib::Solution;

pub struct Day3;

impl Solution for Day3 {
    fn part1(&self, input: &str) -> String {
        format!("Part 1: {}", get_total(input))
    }
    fn part2(&self, input: &str) -> String {
        format!("Part 2: {}", get_total_for_three(input))
    }
}

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn get_total(input: &str) -> usize {
    input
        .lines()
        .filter_map(|x| {
            find_intersection(x.to_string()).and_then(|c| ALPHABET.find(c).map(|x| x + 1))
        })
        .sum()
}

fn get_total_for_three(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();

    lines
        .chunks(3)
        .filter_map(|group| {
            find_intersection_3(group[0], group[1], group[2])
                .and_then(|c| ALPHABET.find(c).map(|x| x + 1))
        })
        .sum()
}

fn find_intersection(line: String) -> Option<char> {
    let m = line.len() / 2;

    for c1 in line[..m].chars() {
        for c2 in line[m..].chars() {
            if c1 == c2 {
                return Some(c1);
            }
        }
    }

    None
}

fn find_intersection_3(line1: &str, line2: &str, line3: &str) -> Option<char> {
    for c1 in line1.chars() {
        for c2 in line2.chars() {
            if c1 == c2 {
                for c3 in line3.chars() {
                    if c2 == c3 {
                        return Some(c1);
                    }
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
        ";

        assert_eq!(get_total(input), 157)
    }

    #[test]
    fn test_part_2() {
        let input = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

        assert_eq!(get_total_for_three(input), 70)
    }
}

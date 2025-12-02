use aoc_lib::Solution;

pub struct Day1;

impl Solution for Day1 {
    fn part_1(&self, input: &str) -> String {
        let mut start = 50;
        let mut num_zeros = 0;
        let lines = parse_lines(input);
        for direction in lines {
            let next_location = get_next_location(&direction, start);

            start = next_location;

            if start == 0 {
                num_zeros += 1;
            }
        }
        num_zeros.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let lines = parse_lines(input);

        let mut current = 50;
        let mut num_zeros = 0;

        for direction in lines {
            let next_location = get_next_location(&direction, current);

            let num_wraps = count_wraps(&direction, current);
            println!("Dir: {direction:?}, start: {current}, {num_wraps}");
            num_zeros += num_wraps;

            current = next_location;
        }

        num_zeros.to_string()
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Left(i32),
    Right(i32),
}

fn get_next_location(direction: &Direction, start: i32) -> i32 {
    match direction {
        Direction::Left(amount) => (start - amount).rem_euclid(100),
        Direction::Right(amount) => (start + amount).rem_euclid(100),
    }
}

fn count_wraps(direction: &Direction, starting: i32) -> i32 {
    match direction {
        Direction::Right(amount) => (starting + amount) / 100,
        Direction::Left(amount) => {
            if *amount <= starting {
                0
            } else {
                (amount - starting + 99) / 100
            }
        }
    }
}

fn parse_lines(input: &str) -> Vec<Direction> {
    input
        .lines()
        .map(|line| {
            let dir = &line[0..1];
            let amount = &line[1..];
            let parsed = amount
                .parse::<i32>()
                .expect("Number is always present and 0 to 90");
            let direction = match dir {
                "L" => Direction::Left(parsed),
                "R" => Direction::Right(parsed),
                _ => unreachable!(),
            };
            direction
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_parse_line() {
        let lines = parse_lines(INPUT);

        assert_eq!(
            lines,
            vec![
                Direction::Left(68),
                Direction::Left(30),
                Direction::Right(48),
                Direction::Left(5),
                Direction::Right(60),
                Direction::Left(55),
                Direction::Left(1),
                Direction::Left(99),
                Direction::Right(14),
                Direction::Left(82)
            ]
        );
    }

    #[test]
    fn get_next_location_correctly_wraps_right() {
        let result = get_next_location(&Direction::Right(101), 99);

        assert_eq!(result, 0);
    }

    #[test]
    fn get_next_location_correctly_wraps_left() {
        let result = get_next_location(&Direction::Left(1), 0);

        assert_eq!(result, 99);
    }

    #[test]
    fn count_wraps_correctly_counts_left() {
        let one_wrap = count_wraps(&Direction::Left(5), 0);
        assert_eq!(one_wrap, 1);

        let one_wrap = count_wraps(&Direction::Left(68), 50);
        assert_eq!(one_wrap, 1);
    }

    #[test]
    fn count_wraps_correctly_counts_right() {
        let zero_wrap = count_wraps(&Direction::Right(10), 3);
        let two_wrap = count_wraps(&Direction::Right(210), 3);
        let ten_wrap = count_wraps(&Direction::Right(1000), 50);

        assert_eq!(zero_wrap, 0);
        assert_eq!(two_wrap, 2);
        assert_eq!(ten_wrap, 10);
    }

    #[test]
    fn part1_returns_3_with_example_input() {
        let total = Day1.part_1(INPUT);

        assert_eq!(total, "3".to_string());
    }

    #[test]
    fn part2_returns_6_with_example_input() {
        let total = Day1.part_2(INPUT);

        assert_eq!(total, "6".to_string());
    }
}

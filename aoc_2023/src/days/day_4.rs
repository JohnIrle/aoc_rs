use aoc_lib::Solution;
pub struct Day4;

impl Solution for Day4 {
    fn part_1(&self, input: &str) -> String {
        total_points(input).to_string()
    }

    fn part_2(&self, input: &str) -> String {
        total_cards(input).to_string()
    }
}

fn total_points(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        let (winning_numbers, cards) = parse_line(line);
        let winners_score = cards.iter().filter(|card| winning_numbers.contains(card));
        match winners_score.count() {
            0 => {}
            1 => total += 1,
            n => total += 2_usize.pow(u32::try_from(n).expect("could not get u32") - 1),
        }
    }
    total
}

fn total_cards(input: &str) -> usize {
    let mut cards_at_index = vec![1; input.lines().count()];

    input.lines().enumerate().for_each(|(index, line)| {
        let (winning_numbers, cards) = parse_line(line);
        let winners_score = cards.iter().filter(|card| winning_numbers.contains(card));

        let num_winners = winners_score.count();
        let number_of_cards_at_index = cards_at_index.get(index);

        if let Some(number) = number_of_cards_at_index {
            for _ in 0..*number {
                for i in 0..num_winners {
                    if let Some(value) = cards_at_index.get_mut((index + i) + 1) {
                        *value += 1;
                    }
                }
            }
        }
    });

    cards_at_index.iter().sum()
}

fn parse_line(line: &str) -> (Vec<&str>, Vec<&str>) {
    let numbers = line.split(':').collect::<Vec<&str>>()[1].trim();
    let mut rest = numbers.split(" | ");
    let (winning, cards) = (
        rest.next().expect("couldn't get winning"),
        rest.next().expect("couldn't get cards"),
    );

    (
        winning.split_whitespace().collect(),
        cards.split_whitespace().collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn test_part_1() {
        assert_eq!(Day4.part_1(INPUT), "13".to_string());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Day4.part_2(INPUT), "30".to_string());
    }
}

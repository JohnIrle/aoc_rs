mod days;

use crate::days::{Day1, Day2, Day3, Day4, Day5};

use aoc_lib::Solution;

#[must_use]
pub fn solutions() -> Vec<(u32, Box<dyn Solution>)> {
    vec![
        (1, Box::new(Day1)),
        (2, Box::new(Day2)),
        (3, Box::new(Day3)),
        (4, Box::new(Day4)),
        (5, Box::new(Day5)),
    ]
}

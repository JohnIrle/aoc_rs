mod days;

use aoc_lib::Solution;
use days::Day1;

#[must_use]
pub fn solutions() -> Vec<(u32, Box<dyn Solution>)> {
    vec![
        (1, Box::new(Day1)),
    ]
}

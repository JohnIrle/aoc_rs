mod days;

use aoc_lib::Solution;
use days::Day3;

#[must_use]
pub fn solutions() -> Vec<(u32, Box<dyn Solution>)> {
    vec![(3, Box::new(Day3))]
}

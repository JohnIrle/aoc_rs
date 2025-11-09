use std::fs;

pub trait Solution {
    fn part_1(&self, input: &str) -> String;
    fn part_2(&self, input: &str) -> String;
}

pub fn read_input(year: u32, day: u32) -> String {
    let path = format!("inputs/{year}/day{day}.txt");
    fs::read_to_string(path).expect("Failed to read input file")
}

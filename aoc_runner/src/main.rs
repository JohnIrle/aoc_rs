use aoc_lib::{Solution, read_input};
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    year: u32,
    #[arg(long)]
    day: u32,
}

fn main() {
    let args = Args::parse();

    let solution: Box<dyn Solution> = match args.year {
        2022 => lookup(args.day, aoc_2022::solutions()),
        _ => panic!("Year not supported"),
    };

    let input = read_input(args.year, args.day);
    println!("Part 1: {}", solution.part1(&input));
    println!("Part 2: {}", solution.part2(&input));
}

fn lookup(day: u32, list: Vec<(u32, Box<dyn Solution>)>) -> Box<dyn Solution> {
    list.into_iter()
        .find(|(d, _)| *d == day)
        .map(|(_, s)| s)
        .unwrap_or_else(|| panic!("Day {} not implemented", day))
}

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
        _ => panic!("Year not supported"),
    };

    let input = read_input(args.year, args.day);
    println!("Part 1: {}", solution.part1(&input));
    println!("Part 2: {}", solution.part2(&input));
}

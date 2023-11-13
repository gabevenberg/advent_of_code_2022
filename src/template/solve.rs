#![allow(dead_code)]
mod part1;
mod part2;
mod utilities;

fn main() {
    let input = include_str!("./input.txt");
    let structured_input = utilities::parse(input);

    println!("Part One");
    println!("Result: {}", part1::part1());

    println!("Part Two");
    println!("Result: {}", part2::part2());
}

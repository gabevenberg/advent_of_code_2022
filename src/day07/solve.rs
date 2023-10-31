#![allow(unused)]
mod part1;
mod part2;
mod parser;
mod file_tree;

fn main() {
    let input = include_str!("./input.txt");
    let structured_input = parser::parse(input);

    println!("Part One");
    println!("Result: {}", part1::part1());

    println!("Part Two");
    println!("Result: {}", part2::part2());
}

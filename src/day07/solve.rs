mod part1;
mod part2;
mod parser;
mod fileTree;

fn main() {
    let _input = include_str!("./input.txt");
    let _structured_input = parser::parse(_input);

    println!("Part One");
    println!("Result: {}", part1::part1());

    println!("Part Two");
    println!("Result: {}", part2::part2());
}

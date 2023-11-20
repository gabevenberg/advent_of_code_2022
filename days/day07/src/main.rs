use std::rc::Rc;

use crate::file_tree::NodeRef;

mod part1;
mod part2;
mod parser;
mod file_tree;

fn main() {
    let input = include_str!("./input.txt");
    let structured_input = parser::parse(input);

    println!("Part One");
    println!("Result: {}", part1::part1(NodeRef(Rc::clone(&structured_input))));

    println!("Part Two");
    println!("Result: {}", part2::part2(NodeRef(Rc::clone(&structured_input))));
}

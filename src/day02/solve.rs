mod part1;
mod part2;
mod utilities;

fn main() {
    let _input = include_str!("./input.txt");

    println!("Part One");
    println!("Result: {}", part1::part1(_input));

    println!("Part Two");
    println!("Result: {}", part2::part2(_input));
}

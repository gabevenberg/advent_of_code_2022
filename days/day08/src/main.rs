mod part1;
mod part2;
mod utilities;

type StructuredInput = Vec<Vec<u8>>;

fn main() {
    let input = include_str!("./input.txt");
    let structured_input = utilities::parse(input);

    println!("Part One");
    println!("Result: {}", part1::part1(&structured_input));

    println!("Part Two");
    println!("Result: {}", part2::part2(&structured_input));
}

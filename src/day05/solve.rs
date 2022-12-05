mod part1;
mod part2;
mod utilities;

fn main() {
    let _input = include_str!("./input.txt");
    let _structured_input = utilities::parse(_input);

    println!("Part One");
    println!("Result: {:?}", part1::part1(&_structured_input));

    println!("Part Two");
    println!("Result: {:?}", part2::part2(&_structured_input));
}

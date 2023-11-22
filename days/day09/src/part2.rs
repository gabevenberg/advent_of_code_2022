use std::collections::HashSet;

use aoc_libs::points::Point;

use crate::{parse::Direction, rope::Rope};

pub fn part2(input: &Vec<Direction>) -> usize {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut rope: Rope<10> = Rope::new();
    visited.insert(*rope.get_tail_pos());
    for direction in input {
        rope.update_rope(direction);
        visited.insert(*rope.get_tail_pos());
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use crate::parse::parse;

    use super::*;

    #[test]
    fn test_part2() {
        let input = parse(concat!(
            "R 5\n", "U 8\n", "L 8\n", "D 3\n", "R 17\n", "D 10\n", "L 25\n", "U 20\n",
        ));
        assert_eq!(part2(&input), 36);
    }
}

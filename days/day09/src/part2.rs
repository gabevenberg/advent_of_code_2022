use std::collections::HashSet;

use aoc_libs::points::Point;

use crate::{rope::Rope, parse::Direction};

pub fn part2(input: &Vec<Direction>) -> usize {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut rope: Rope<10> = Rope::new();
    for direction in input {
        visited.insert(*rope.get_tail_pos());
        rope.update_rope(direction);
        println!("{}", rope)
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(0, 0);
    }
}

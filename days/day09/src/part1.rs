use std::collections::HashSet;

use aoc_libs::points::Point;

use crate::parse::Direction;
use crate::rope::Rope;

pub fn part1(input: &Vec<Direction>) -> usize {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut rope: Rope<2> = Rope::new();
    visited.insert(*rope.get_tail_pos());
    for direction in input {
        rope.update_rope(direction);
        visited.insert(*rope.get_tail_pos());
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Up,
            Direction::Up,
            Direction::Up,
            Direction::Up,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Down,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Right,
            Direction::Right,
        ];
        assert_eq!(part1(&input), 13);
    }
}

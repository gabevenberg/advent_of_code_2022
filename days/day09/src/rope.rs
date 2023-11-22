use std::fmt::Display;

use crate::parse::Direction;
use aoc_libs::points::{Point, UPoint};

// L is the length of the rope in segments.
#[derive(Debug)]
pub struct Rope<const L: usize> {
    segments: [Point; L],
}

impl<const L: usize> Rope<L> {
    pub fn new() -> Rope<L> {
        Rope {
            segments: [Point::default(); L],
        }
    }

    pub fn update_rope(&mut self, direction: &Direction) {
        self.segments[0] += match direction {
            Direction::Up => Point { x: 0, y: 1 },
            Direction::Right => Point { x: 1, y: 0 },
            Direction::Down => Point { x: 0, y: -1 },
            Direction::Left => Point { x: -1, y: 0 },
        };
        for segment in 1..self.segments.len() {
            self.segments[segment] += Rope::<L>::update_single_segment_pair(
                &self.segments[segment - 1],
                &self.segments[segment],
            )
        }
    }

    pub fn get_tail_pos(&self) -> &Point {
        &self.segments[self.segments.len() - 1]
    }

    // the rope segment will not move if the segment ahead of it is only at most one away, and will
    // move with the following rules if it is 2 away: It moves straight towards the head if the
    // head is directly up/down/left/right, and diagonally towards it if its not straight
    // up/down/left/right.
    fn update_single_segment_pair(head: &Point, tail: &Point) -> Point {
        let delta = *head - *tail;
        if delta.x.abs() > 2 || delta.y.abs() > 2 {
            panic!("invalid delta ({}, {})", delta.y, delta.x)
        }
        match (delta.x, delta.y) {
            (0, 2) => Point { x: 0, y: 1 },
            (2, 0) => Point { x: 1, y: 0 },
            (0, -2) => Point { x: 0, y: -1 },
            (-2, 0) => Point { x: -1, y: 0 },
            (1, 2) | (2, 2) | (2, 1) => Point { x: 1, y: 1 },
            (2, -1) | (2, -2) | (1, -2) => Point { x: 1, y: -1 },
            (-1, -2) | (-2, -2) | (-2, -1) => Point { x: -1, y: -1 },
            (-2, 1) | (-2, 2) | (-1, 2) => Point { x: -1, y: 1 },
            _ => Point { x: 0, y: 0 },
        }
    }
}

impl<const L: usize> Display for Rope<{ L }> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let grid_size = (self.segments.len() * 2) - 1;
        let zero_point = UPoint {
            x: self.segments.len() - 1,
            y: self.segments.len() - 1,
        };

        let mut grid: Vec<Vec<char>> = Vec::with_capacity(grid_size);
        for y in 0..grid_size {
            grid.push(Vec::with_capacity(grid_size));
            for _ in 0..grid_size {
                grid[y].push('.')
            }
        }

        for segment in self.segments.iter().skip(1) {
            let delta = *segment - self.segments[0];
            let upoint = delta.to_upoint(&zero_point).unwrap();
            grid[upoint.y][upoint.x] = 'T'
        }

        writeln!(
            f,
            "head is at {}, {}",
            self.segments[0].x, self.segments[0].y
        )?;
        let upoint = Point { x: 0, y: 0 }.to_upoint(&zero_point).unwrap();
        grid[upoint.y][upoint.x] = 'H';

        for line in grid {
            let mut writeline = "".to_string();
            for char in line {
                writeline.push(char)
            }
            writeln!(f, "{}", writeline)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_single_segment() {
        let input = [
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
        let mut visited: HashSet<Point> = HashSet::new();
        let mut rope: Rope<2> = Rope::new();
        visited.insert(*rope.get_tail_pos());
        println!("{}", rope);
        for direction in input {
            rope.update_rope(&direction);
            visited.insert(*rope.get_tail_pos());
            println!("{}", rope);
        }
        // let mut graph = [
        //     ['.', '.', '.', '.', '.', '.'],
        //     ['.', '.', '.', '.', '.', '.'],
        //     ['.', '.', '.', '.', '.', '.'],
        //     ['.', '.', '.', '.', '.', '.'],
        //     ['s', '.', '.', '.', '.', '.'],
        // ];
        // for point in &visited {
        //     graph[4 - point.y as usize][point.x as usize] = '#';
        // }
        // for line in graph {
        //     println!("{:?}", line)
        // }
        assert_eq!(visited.len(), 13)
    }
}

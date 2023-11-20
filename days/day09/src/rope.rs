use crate::parse::Direction;
#[derive(Debug, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

// L is the length of the rope in segments.
#[derive(Debug)]
struct Rope<const L: usize> {
    segments: [Point; L],
}

impl<const L: usize> Rope<L> {
    pub fn new()->Rope{
        Rope { segments: () }
    }

    pub fn update_rope(&mut self, direction: Direction) {
        todo!()
    }

    pub fn get_tail_pos(&self)->&Point{
        &self.segments[self.segments.len()-1]
    }

    fn update_single_segment_pair(head: Point, tail: Point) -> Point {
        let delta = head - tail;
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

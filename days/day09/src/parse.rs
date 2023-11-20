#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub fn parse(input: &str) -> Vec<Direction> {
    let mut ret = Vec::new();
    for line in input.lines() {
        let mut chars = line.chars();
        let dir = match chars.next().unwrap() {
            'U' => Direction::Up,
            'R' => Direction::Right,
            'D' => Direction::Down,
            'L' => Direction::Left,
            _ => panic!("invalid direction char"),
        };
        for _ in 0..chars.nth(1).unwrap().to_digit(10).unwrap() {
            ret.push(dir)
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input =
            concat!("R 4\n", "U 4\n", "L 3\n", "D 1\n", "R 4\n", "D 1\n", "L 5\n", "R 2\n",);
        assert_eq!(
            parse(input),
            vec![
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
            ]
        );
    }
}

use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

static PARSE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^([URDL]) (\d+)$").unwrap());

pub fn parse(input: &str) -> Vec<Direction> {
    let mut ret = Vec::new();
    for line in input.lines() {
        let captures = PARSE_REGEX
            .captures(line)
            .unwrap_or_else(|| panic!("invalid line {}", line));
        let dir = match &captures[1] {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => panic!("invalid direction char"),
        };
        for _ in 0..captures[2]
            .parse()
            .unwrap_or_else(|_| panic!("invalid line {}", line))
        {
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

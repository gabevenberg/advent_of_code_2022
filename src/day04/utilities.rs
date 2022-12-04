use once_cell::sync::Lazy;
use regex::Regex;
#[derive(Debug, PartialEq, Eq)]
pub struct Range {
    start: u16,
    end: u16,
}

impl Range {
    pub fn new(start: u16, end: u16) -> Self {
        Self {
            start: start.min(end),
            end: end.max(start),
        }
    }

    pub fn calc_size(&self) -> u16 {
        self.start.abs_diff(self.end)
    }

    pub fn any_overlap(&self, other: &Self) -> bool {
        self.start <= other.end && self.end >= other.start
    }

    pub fn calc_overlap(&self, other: &Self) -> Range {
        let overlap_start = self.start.min(other.start);
        let overlap_end = self.end.max(other.end);
        Range::new(overlap_start, overlap_end)
    }

    pub fn complete_overlap(&self, other: &Self) -> bool {
        self.calc_overlap(other) == *self || self.calc_overlap(other) == *other
    }

    pub fn start(&self) -> u16 {
        self.start
    }

    pub fn end(&self) -> u16 {
        self.end
    }
}

static PARSE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)").unwrap());

pub fn parse(input: &str) -> Vec<(Range, Range)> {
    input
        .lines()
        .map(|line| {
            let cap = PARSE_REGEX.captures(line).unwrap();
            (
                Range::new(
                    cap.get(1).unwrap().as_str().parse().unwrap(),
                    cap.get(2).unwrap().as_str().parse().unwrap(),
                ),
                Range::new(
                    cap.get(3).unwrap().as_str().parse().unwrap(),
                    cap.get(4).unwrap().as_str().parse().unwrap(),
                ),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
19-30,5-18";
        assert_eq!(
            parse(input),
            vec![
                (Range::new(2, 4), Range::new(6, 8)),
                (Range::new(2, 3), Range::new(4, 5)),
                (Range::new(5, 7), Range::new(7, 9)),
                (Range::new(2, 8), Range::new(3, 7)),
                (Range::new(6, 6), Range::new(4, 6)),
                (Range::new(2, 6), Range::new(4, 8)),
                (Range::new(19, 30), Range::new(5, 18)),
            ]
        );
    }
}

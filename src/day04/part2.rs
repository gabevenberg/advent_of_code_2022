use crate::utilities::*;

pub fn part2(input: &[(Range, Range)]) -> usize {
    input
        .iter()
        .filter(|tuple| tuple.0.any_overlap(&tuple.1))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = vec![
            (Range::new(2, 4), Range::new(6, 8)),
            (Range::new(2, 3), Range::new(4, 5)),
            (Range::new(5, 7), Range::new(7, 9)),
            (Range::new(2, 8), Range::new(3, 7)),
            (Range::new(6, 6), Range::new(4, 6)),
            (Range::new(2, 6), Range::new(4, 8)),
            (Range::new(19, 30), Range::new(5, 18)),
        ];
        assert_eq!(part2(&input), 4);
    }
}

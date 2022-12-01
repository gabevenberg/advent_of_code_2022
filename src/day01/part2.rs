use crate::parse;

pub fn part2(input: &[parse::Elf]) -> usize {
    input
        .iter()
        .map(|elf| elf.0.iter().sum::<usize>())
        // not sure what to do next here, how do I get the 3 max values from here?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = vec![
            parse::Elf(vec![1000, 2000, 3000]),
            parse::Elf(vec![4000]),
            parse::Elf(vec![5000, 6000]),
            parse::Elf(vec![7000, 8000, 9000]),
            parse::Elf(vec![10000]),
        ];
        assert_eq!(part2(&input), 45000);
    }
}

use crate::parse;

pub fn part2(input: &[parse::Elf]) -> usize {
    let mut input = input
        .iter()
        .map(|elf| elf.0.iter().sum::<usize>())
        .collect::<Vec<usize>>();
    input.sort_unstable();
    input[input.len() - 3..].iter().sum()
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

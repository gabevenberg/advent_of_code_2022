#[derive(Debug, PartialEq, Eq)]
pub struct Elf(pub Vec<usize>);

pub fn parse(input: &str) -> Vec<Elf> {
    input
        .trim()
        .split("\n\n")
        .map(|group| {
            Elf(group
                .split('\n')
                .map(|line| line.parse().unwrap())
                .collect())
        })
        .collect::<Vec<Elf>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!(
            parse(input),
            vec![
                Elf(vec![1000, 2000, 3000]),
                Elf(vec![4000]),
                Elf(vec![5000, 6000]),
                Elf(vec![7000, 8000, 9000]),
                Elf(vec![10000])
            ]
        );
    }
}

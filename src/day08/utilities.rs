#![allow(unused)]
pub fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = concat!("30373\n", "25512\n", "65332\n", "33549\n", "35390\n",);
        assert_eq!(
            parse(input),
            vec![
                vec![3, 0, 3, 7, 3],
                vec![2, 5, 5, 1, 2],
                vec![6, 5, 3, 3, 2],
                vec![3, 3, 5, 4, 9],
                vec![3, 5, 3, 9, 0]
            ]
        );
    }
}

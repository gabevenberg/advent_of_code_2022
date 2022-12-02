pub fn parse(input: &str) -> usize {
    println!("{}", input);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input =
"test"
;
        assert_eq!(parse(input), 0);
    }
}

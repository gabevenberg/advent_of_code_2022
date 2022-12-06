/* pub fn parse(input: &str) -> usize {
    unimplemented!()
} */

pub fn find_dupes_stupid<T: PartialEq>(slice: &[T]) -> bool {
    for i in 1..slice.len() {
        if slice[i..].contains(&slice[i - 1]) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "test";
        assert_eq!(parse(input), 0);
    }
}

use std::collections::HashSet;
#[derive(Debug, PartialEq, Eq)]
pub struct Rucksack(pub HashSet<char>, pub HashSet<char>);

pub fn parse(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);
            Rucksack(first.chars().collect(), second.chars().collect())
        })
        .collect()
}

pub fn find_char_score(input: &char) -> usize {
    static CHARS: [char; 52] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    CHARS
        .iter()
        .position(|character| character == input)
        .unwrap()+1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(
            parse(input),
            vec![
                Rucksack(
                    HashSet::from(['v', 'J', 'r', 'w', 'p', 'W', 't', 'w', 'J', 'g', 'W', 'r']),
                    HashSet::from(['h', 'c', 's', 'F', 'M', 'M', 'f', 'F', 'F', 'h', 'F', 'p'])
                ),
                Rucksack(
                    HashSet::from([
                        'j', 'q', 'H', 'R', 'N', 'q', 'R', 'j', 'q', 'z', 'j', 'G', 'D', 'L', 'G',
                        'L'
                    ]),
                    HashSet::from([
                        'r', 's', 'F', 'M', 'f', 'F', 'Z', 'S', 'r', 'L', 'r', 'F', 'Z', 's', 'S',
                        'L'
                    ])
                ),
                Rucksack(
                    HashSet::from(['P', 'm', 'm', 'd', 'z', 'q', 'P', 'r', 'V']),
                    HashSet::from(['v', 'P', 'w', 'w', 'T', 'W', 'B', 'w', 'g'])
                ),
                Rucksack(
                    HashSet::from([
                        'w', 'M', 'q', 'v', 'L', 'M', 'Z', 'H', 'h', 'H', 'M', 'v', 'w', 'L', 'H',
                    ]),
                    HashSet::from([
                        'j', 'b', 'v', 'c', 'j', 'n', 'n', 'S', 'B', 'n', 'v', 'T', 'Q', 'F', 'n',
                    ])
                ),
                Rucksack(
                    HashSet::from(['t', 't', 'g', 'J', 't', 'R', 'G', 'J',]),
                    HashSet::from(['Q', 'c', 't', 'T', 'Z', 't', 'Z', 'T',])
                ),
                Rucksack(
                    HashSet::from(['C', 'r', 'Z', 's', 'J', 's', 'P', 'P', 'Z', 's', 'G', 'z',]),
                    HashSet::from(['w', 'w', 's', 'L', 'w', 'L', 'm', 'p', 'w', 'M', 'D', 'w',])
                ),
            ]
        );
    }
}

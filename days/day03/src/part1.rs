use crate::utilities::*;

pub fn part1(input: &[Rucksack]) -> usize {
    input
        .iter()
        .map(|rucksack| rucksack.0.intersection(&rucksack.1).next().unwrap())
        .map(find_char_score)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_part1() {
        let input = vec![
            Rucksack(
                HashSet::from(['v', 'J', 'r', 'w', 'p', 'W', 't', 'w', 'J', 'g', 'W', 'r']),
                HashSet::from(['h', 'c', 's', 'F', 'M', 'M', 'f', 'F', 'F', 'h', 'F', 'p']),
            ),
            Rucksack(
                HashSet::from([
                    'j', 'q', 'H', 'R', 'N', 'q', 'R', 'j', 'q', 'z', 'j', 'G', 'D', 'L', 'G', 'L',
                ]),
                HashSet::from([
                    'r', 's', 'F', 'M', 'f', 'F', 'Z', 'S', 'r', 'L', 'r', 'F', 'Z', 's', 'S', 'L',
                ]),
            ),
            Rucksack(
                HashSet::from(['P', 'm', 'm', 'd', 'z', 'q', 'P', 'r', 'V']),
                HashSet::from(['v', 'P', 'w', 'w', 'T', 'W', 'B', 'w', 'g']),
            ),
            Rucksack(
                HashSet::from([
                    'w', 'M', 'q', 'v', 'L', 'M', 'Z', 'H', 'h', 'H', 'M', 'v', 'w', 'L', 'H',
                ]),
                HashSet::from([
                    'j', 'b', 'v', 'c', 'j', 'n', 'n', 'S', 'B', 'n', 'v', 'T', 'Q', 'F', 'n',
                ]),
            ),
            Rucksack(
                HashSet::from(['t', 't', 'g', 'J', 't', 'R', 'G', 'J']),
                HashSet::from(['Q', 'c', 't', 'T', 'Z', 't', 'Z', 'T']),
            ),
            Rucksack(
                HashSet::from(['C', 'r', 'Z', 's', 'J', 's', 'P', 'P', 'Z', 's', 'G', 'z']),
                HashSet::from(['w', 'w', 's', 'L', 'w', 'L', 'm', 'p', 'w', 'M', 'D', 'w']),
            ),
        ];
        assert_eq!(part1(&input), 157);
    }
}

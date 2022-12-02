use crate::utilities::*;
pub fn part1(input: &str) -> usize {
    parse(input).iter().map(calc_score).sum()
}

pub fn parse(input: &str) -> Vec<Strategy> {
    input
        .lines()
        .map(|line| {
            let elf = match line.as_bytes()[0] {
                b'A' => Play::Rock,
                b'B' => Play::Paper,
                b'C' => Play::Scissors,
                _ => panic!("your opponent not playing defined strategy!"),
            };
            let you = match line.as_bytes()[2] {
                b'X' => Play::Rock,
                b'Y' => Play::Paper,
                b'Z' => Play::Scissors,
                _ => panic!("you are not playing defined strategy!"),
            };
            Strategy { elf, you }
        })
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "A Y
B X
C Z";
        assert_eq!(part1(input), 15);
    }

    #[test]
    fn test_parse() {
        let input = "A Y
B X
C Z";
        assert_eq!(
            parse(input),
            vec![
                Strategy {
                    elf: Play::Rock,
                    you: Play::Paper
                },
                Strategy {
                    elf: Play::Paper,
                    you: Play::Rock
                },
                Strategy {
                    elf: Play::Scissors,
                    you: Play::Scissors
                }
            ]
        );
    }
}

use crate::utilities::*;

pub enum GameResult {
    Win,
    Loss,
    Draw,
}

pub struct ResultStrategy {
    pub elf: Play,
    pub you: GameResult,
}

pub fn part2(input: &str) -> usize {
    parse(input)
        .iter()
        .map(gen_strategy)
        .map(|strat| calc_score(&strat))
        .sum()
}

fn gen_strategy(input: &ResultStrategy) -> Strategy {
    match input.you {
        GameResult::Win => Strategy {
            elf: input.elf,
            you: gen_win(input.elf),
        },
        GameResult::Draw => Strategy {
            elf: input.elf,
            you: input.elf,
        },
        GameResult::Loss => Strategy {
            elf: input.elf,
            you: gen_loss(input.elf),
        },
    }
}

fn gen_win(opponent: Play) -> Play {
    match opponent {
        Play::Rock => Play::Paper,
        Play::Paper => Play::Scissors,
        Play::Scissors => Play::Rock,
    }
}

fn gen_loss(opponent: Play) -> Play {
    match opponent {
        Play::Rock => Play::Scissors,
        Play::Paper => Play::Rock,
        Play::Scissors => Play::Paper,
    }
}

pub fn parse(input: &str) -> Vec<ResultStrategy> {
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
                b'X' => GameResult::Loss,
                b'Y' => GameResult::Draw,
                b'Z' => GameResult::Win,
                _ => panic!("you are not playing defined strategy!"),
            };
            ResultStrategy { elf, you }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "A Y
B X
C Z";
        assert_eq!(part2(input), 12);
    }
}

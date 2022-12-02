#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Strategy {
    pub elf: Play,
    pub you: Play,
}

pub fn calc_score(input: &Strategy) -> usize{
    //play enum has value corresponding to its score.
    let mut score = input.you as usize;
    //an enum wins if (you-elf)%3 = 1, looses if it = 2
    let gamestatus = (input.you as i8 - input.elf as i8).rem_euclid(3);
    match gamestatus {
        1 => score+=6,
        2 => (),
        0 => score += 3,
        _ => panic!("gamestatus was {}!, you were {}, elf was {}", gamestatus, input.you as i8, input.elf as i8)
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_score() {
            let input = Strategy {
                    elf: Play::Scissors,
                    you: Play::Scissors
                };
        assert_eq!(calc_score(&input), 6);
    }
}

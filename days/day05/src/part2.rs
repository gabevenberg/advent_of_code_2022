use crate::utilities::*;

pub fn part2(input: &(WorkArea, Vec<Move>)) -> Vec<char> {
    let (mut work_area, moves) = input.to_owned();
    for r#move in moves {
        work_area.apply_move_cratemover9001(&r#move)
    }
    work_area
        .get_stacks()
        .iter()
        .map(|stack| stack.last().unwrap().to_owned())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = (
            WorkArea::new(vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]),
            vec![
                Move {
                    to: 1,
                    from: 2,
                    number: 1,
                },
                Move {
                    to: 3,
                    from: 1,
                    number: 3,
                },
                Move {
                    to: 1,
                    from: 2,
                    number: 2,
                },
                Move {
                    to: 2,
                    from: 1,
                    number: 1,
                },
            ],
        );
        assert_eq!(part2(&input), vec!['M', 'C', 'D']);
    }
}

use once_cell::sync::Lazy;
use regex::Regex;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Move {
    pub to: usize,
    pub from: usize,
    pub number: u8,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WorkArea {
    stacks: Vec<Vec<char>>,
}

impl WorkArea {
    pub fn new(stacks: Vec<Vec<char>>) -> Self {
        Self { stacks }
    }
    pub fn apply_move_cratemover9000(&mut self, action: &Move) {
        for _ in 0..action.number {
            let cargo = self.stacks.get_mut(action.from - 1).unwrap().pop().unwrap();
            self.stacks.get_mut(action.to - 1).unwrap().push(cargo);
        }
    }
    pub fn apply_move_cratemover9001(&mut self, action: &Move) {
        let mut crane_holder: Vec<char> = Vec::new();
        for _ in 0..action.number {
            let cargo = self.stacks.get_mut(action.from - 1).unwrap().pop().unwrap();
            crane_holder.push(cargo);
        }
        for cargo in crane_holder.iter().rev() {
            self.stacks.get_mut(action.to - 1).unwrap().push(*cargo);
        }
    }
    pub fn get_stacks(&self) -> &Vec<Vec<char>> {
        &self.stacks
    }
}

pub fn parse(input: &str) -> (WorkArea, Vec<Move>) {
    let mut input = input.split("\n\n");
    let work_area = parse_work_area(input.next().unwrap());
    let moves = parse_moves(input.next().unwrap());
    (work_area, moves)
}

pub fn parse_work_area(input: &str) -> WorkArea {
    //decode those bottom index numbers
    let index_row = input.lines().rev().next().unwrap();
    //some ascii math and array math to get the second to last char and convert it into a number.
    let index_max: usize = (index_row.as_bytes()[index_row.len() - 2] - b'0') as usize;
    //initalize the work area:
    let mut work_area: Vec<Vec<char>> = Vec::new();
    for _ in 0..index_max {
        work_area.push(Vec::new())
    }

    //now parse the rest
    for line in input.lines().rev() {
        for (y, cargo_crate) in line.as_bytes().chunks(4).enumerate() {
            let cargo = cargo_crate[1] as char;
            //easiest way to filter out that last line is just to filter out digits.
            if cargo != ' ' && !cargo.is_ascii_digit() {
                work_area.get_mut(y).unwrap().push(cargo)
            }
        }
    }

    WorkArea::new(work_area)
}

pub fn parse_moves(input: &str) -> Vec<Move> {
    static PARSE_MOVES_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap());
    input
        .lines()
        .map(|line| {
            let cap = PARSE_MOVES_REGEX.captures(line).unwrap();
            Move {
                to: cap.get(3).unwrap().as_str().parse().unwrap(),
                from: cap.get(2).unwrap().as_str().parse().unwrap(),
                number: cap.get(1).unwrap().as_str().parse().unwrap(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_work_area() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 ";
        assert_eq!(
            parse_work_area(input),
            WorkArea::new(vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P'],],)
        )
    }

    #[test]
    fn test_parse() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!(
            parse(input),
            (
                WorkArea::new(vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P'],],),
                vec![
                    Move {
                        to: 1,
                        from: 2,
                        number: 1
                    },
                    Move {
                        to: 3,
                        from: 1,
                        number: 3
                    },
                    Move {
                        to: 1,
                        from: 2,
                        number: 2
                    },
                    Move {
                        to: 2,
                        from: 1,
                        number: 1
                    },
                ]
            )
        );
    }
}

use std::collections::VecDeque;

use crate::{parse::*, machine::Machine};

pub fn part1(input: VecDeque<Instruction>) -> i32 {
    let mut machine = Machine::load_program(input.into());
    let mut total = 0;

    //20th
    total+=machine.step20();

    machine.step20();
    //60th
    total+=machine.step20();

    machine.step20();
    //100th
    total+=machine.step20();

    machine.step20();
    //140th
    total+=machine.step20();

    machine.step20();
    //180th
    total+=machine.step20();

    machine.step20();
    //220th
    total+=machine.step20();

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![
            Instruction::Addx(15),
            Instruction::Addx(-11),
            Instruction::Addx(6),
            Instruction::Addx(-3),
            Instruction::Addx(5),
            Instruction::Addx(-1),
            Instruction::Addx(-8),
            Instruction::Addx(13),
            Instruction::Addx(4),
            Instruction::NoOp,
            Instruction::Addx(-1),
            Instruction::Addx(5),
            Instruction::Addx(-1),
            Instruction::Addx(5),
            Instruction::Addx(-1),
            Instruction::Addx(5),
            Instruction::Addx(-1),
            Instruction::Addx(5),
            Instruction::Addx(-1),
            Instruction::Addx(-35),
            Instruction::Addx(1),
            Instruction::Addx(24),
            Instruction::Addx(-19),
            Instruction::Addx(1),
            Instruction::Addx(16),
            Instruction::Addx(-11),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::Addx(21),
            Instruction::Addx(-15),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::Addx(-3),
            Instruction::Addx(9),
            Instruction::Addx(1),
            Instruction::Addx(-3),
            Instruction::Addx(8),
            Instruction::Addx(1),
            Instruction::Addx(5),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::Addx(-36),
            Instruction::NoOp,
            Instruction::Addx(1),
            Instruction::Addx(7),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::Addx(2),
            Instruction::Addx(6),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::Addx(1),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::Addx(7),
            Instruction::Addx(1),
            Instruction::NoOp,
            Instruction::Addx(-13),
            Instruction::Addx(13),
            Instruction::Addx(7),
            Instruction::NoOp,
            Instruction::Addx(1),
            Instruction::Addx(-33),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::Addx(2),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::Addx(8),
            Instruction::NoOp,
            Instruction::Addx(-1),
            Instruction::Addx(2),
            Instruction::Addx(1),
            Instruction::NoOp,
            Instruction::Addx(17),
            Instruction::Addx(-9),
            Instruction::Addx(1),
            Instruction::Addx(1),
            Instruction::Addx(-3),
            Instruction::Addx(11),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::Addx(1),
            Instruction::NoOp,
            Instruction::Addx(1),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::Addx(-13),
            Instruction::Addx(-19),
            Instruction::Addx(1),
            Instruction::Addx(3),
            Instruction::Addx(26),
            Instruction::Addx(-30),
            Instruction::Addx(12),
            Instruction::Addx(-1),
            Instruction::Addx(3),
            Instruction::Addx(1),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::Addx(-9),
            Instruction::Addx(18),
            Instruction::Addx(1),
            Instruction::Addx(2),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::Addx(9),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::Addx(-1),
            Instruction::Addx(2),
            Instruction::Addx(-37),
            Instruction::Addx(1),
            Instruction::Addx(3),
            Instruction::NoOp,
            Instruction::Addx(15),
            Instruction::Addx(-21),
            Instruction::Addx(22),
            Instruction::Addx(-6),
            Instruction::Addx(1),
            Instruction::NoOp,
            Instruction::Addx(2),
            Instruction::Addx(1),
            Instruction::NoOp,
            Instruction::Addx(-10),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::Addx(20),
            Instruction::Addx(1),
            Instruction::Addx(2),
            Instruction::Addx(2),
            Instruction::Addx(-6),
            Instruction::Addx(-11),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
        ];
        assert_eq!(part1(input.into()), 13140)
    }
}

use std::{collections::VecDeque, fmt};

use crate::parse::*;

pub struct Machine {
    instructions: VecDeque<Instruction>,
    current_instruction: Option<(Instruction, usize)>,
    cycle: usize,
    x: i32,
}

impl Machine {
    pub fn load_program(instructions: VecDeque<Instruction>) -> Self {
        let mut res = Machine {
            instructions,
            current_instruction: None,
            cycle: 0,
            x: 1,
        };
        res.fetch_next_instruction();
        res
    }

    pub fn step(&mut self) -> bool {
        if self.current_instruction.is_none() {
            return false;
        }

        let (instruction, cycles_left) = self.current_instruction.as_mut().unwrap();
        *cycles_left -= 1;
        if *cycles_left == 0 {
            if let Instruction::Addx(i) = instruction {
                self.x += *i
            }

            self.fetch_next_instruction();
        };

        self.cycle += 1;
        true
    }

    fn fetch_next_instruction(&mut self) {
        self.current_instruction = self.instructions.pop_front().map(|i| (i, i.cycles()));
    }

    //returns the signal strength during the middle of the 20th cycle.
    pub fn step20(&mut self) -> i32 {
        for _ in 0..19 {
            if !self.step() {
                break;
            }
        }
        let ret = self.x*(self.cycle as i32 +1);
        self.step();
        ret
    }

    pub fn signal_strength(&self) -> i32 {
        self.x * (self.cycle as i32)
    }
}

impl fmt::Debug for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "cycle={} x={} current instruction: {:?} ({} instructions left)",
            self.cycle,
            self.x,
            self.current_instruction,
            self.instructions.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_machine() {
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
        let mut machine = Machine::load_program(input.into());

        //20th
        assert_eq!(machine.step20(), 420);

        machine.step20();
        //60th
        assert_eq!(machine.step20(), 1140);

        machine.step20();
        //100th
        assert_eq!(machine.step20(), 1800);

        machine.step20();
        //140th
        assert_eq!(machine.step20(), 2940);

        machine.step20();
        //180th
        assert_eq!(machine.step20(), 2880);

        machine.step20();
        //220th
        assert_eq!(machine.step20(), 3960);
    }
}

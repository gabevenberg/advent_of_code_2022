use std::collections::VecDeque;

use crate::{crt::Crt, parse::*, machine::Machine};

pub fn part2(input: VecDeque<Instruction>) -> Crt {
    let mut machine = Machine::load_program(input);
    while machine.step().0 {};
    machine.crt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(0, 0);
    }
}

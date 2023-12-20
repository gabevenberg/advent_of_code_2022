use std::fmt::Display;

use crate::machine::Machine;

#[derive(Debug, PartialEq, Eq)]
pub struct Crt {
    pub screen: [[bool; 40]; 6],
}

impl Crt {
    pub fn draw_pixel(&mut self, sprite_pos: i32, cycle: usize) {
        let x_coord = (cycle - 1) % self.screen[0].len();
        if sprite_pos.abs_diff(x_coord as i32) <= 1 {
            self[cycle] = true
        }
    }
}

impl Default for Crt {
    fn default() -> Self {
        Self {
            screen: [[false; 40]; 6],
        }
    }
}

impl Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.screen {
            for char in line {
                if char {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl std::ops::Index<usize> for Crt {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        let index = index - 1;
        let x = index % self.screen[0].len();
        let y = index / self.screen[0].len();
        &self.screen[y][x]
    }
}

impl std::ops::IndexMut<usize> for Crt {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let index = index - 1;
        let x = index % self.screen[0].len();
        let y = index / self.screen[0].len();
        &mut self.screen[y][x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        let mut input = Crt::default();
        input[1] = true;
        input[40] = true;
        input[41] = true;
        println!("{}", input);
        assert_eq!(
            input,
            Crt {
                screen: [
                    [
                        true, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, true
                    ],
                    [
                        true, false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false
                    ],
                    [
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false
                    ],
                    [
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false
                    ],
                    [
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false
                    ],
                    [
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false, false, false, false, false, false
                    ]
                ]
            }
        )
    }
}

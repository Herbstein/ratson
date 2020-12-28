use std::slice;

#[derive(Clone, Copy)]
enum LexerMode {
    A,
    S,
}

impl LexerMode {
    fn flipped(&self) -> Self {
        match self {
            LexerMode::A => LexerMode::S,
            LexerMode::S => LexerMode::A,
        }
    }

    fn byte_to_instruction(&mut self, c: u8) -> Option<Instruction> {
        Some(match self {
            LexerMode::A => match c {
                b'B' => Instruction::Inew,
                b'u' => Instruction::Iinc,
                b'b' => Instruction::Ishl,
                b'a' => Instruction::Iadd,
                b'A' => Instruction::Ineg,
                b'e' => Instruction::Isht,
                b'i' => Instruction::Itof,
                b'\'' => Instruction::Itou,
                b'q' => Instruction::Finf,
                b't' => Instruction::Fnan,
                b'p' => Instruction::Fneg,
                b'?' => {
                    *self = self.flipped();
                    Instruction::Snew
                }
                b'!' => Instruction::Sadd,
                b'~' => Instruction::Onew,
                b'M' => Instruction::Oadd,
                b'@' => Instruction::Anew,
                b's' => Instruction::Aadd,
                b'z' => Instruction::Bnew,
                b'o' => Instruction::Bneg,
                b'.' => Instruction::Nnew,
                b'E' => Instruction::Gdup,
                b'#' => Instruction::Gpop,
                b'%' => Instruction::Gswp,
                _ => None?,
            },
            LexerMode::S => match c {
                b'S' => Instruction::Inew,
                b'h' => Instruction::Iinc,
                b'a' => Instruction::Ishl,
                b'k' => Instruction::Iadd,
                b'r' => Instruction::Ineg,
                b'A' => Instruction::Isht,
                b'z' => Instruction::Itof,
                b'i' => Instruction::Itou,
                b'm' => Instruction::Finf,
                b'b' => Instruction::Fnan,
                b'u' => Instruction::Fneg,
                b'$' => {
                    *self = self.flipped();
                    Instruction::Snew
                }
                b'-' => Instruction::Sadd,
                b'+' => Instruction::Onew,
                b'g' => Instruction::Oadd,
                b'v' => Instruction::Anew,
                b'?' => Instruction::Aadd,
                b'^' => Instruction::Bnew,
                b'!' => Instruction::Bneg,
                b'y' => Instruction::Nnew,
                b'/' => Instruction::Gdup,
                b'e' => Instruction::Gpop,
                b':' => Instruction::Gswp,
                _ => None?,
            },
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Inew,
    Iinc,
    Ishl,
    Iadd,
    Ineg,
    Isht,
    Itof,
    Itou,
    Finf,
    Fnan,
    Fneg,
    Snew,
    Sadd,
    Onew,
    Oadd,
    Anew,
    Aadd,
    Bnew,
    Bneg,
    Nnew,
    Gdup,
    Gpop,
    Gswp,
}

#[derive(Clone)]
pub struct Lexer<'a> {
    input: slice::Iter<'a, u8>,
    mode: LexerMode,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Lexer {
            input: input.iter(),
            mode: LexerMode::A,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Option<Instruction>;

    fn next(&mut self) -> Option<Self::Item> {
        self.input.next().map(|&c| self.mode.byte_to_instruction(c))
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::{Instruction, Lexer};

    #[test]
    fn test_flip() {
        let mut lexer = Lexer::new(b"b?b");
        assert_eq!(Some(Some(Instruction::Ishl)), lexer.next());
        assert_eq!(Some(Some(Instruction::Snew)), lexer.next());
        assert_eq!(Some(Some(Instruction::Fnan)), lexer.next());
    }
}

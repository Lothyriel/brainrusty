use crate::{lexer::Token, Error};

pub fn parse(instructions: &[Token]) -> Result<Vec<Construct>, ParsingError> {
    instructions
        .iter()
        .enumerate()
        .map(|(idx, i)| {
            let construct = match i {
                Token::MoveRight => Construct::MoveRight,
                Token::MoveLeft => Construct::MoveLeft,
                Token::Increment => Construct::Increment,
                Token::Decrement => Construct::Decrement,
                Token::Write => Construct::Write,
                Token::Read => Construct::Read,
                Token::JumpIfFalsy => Construct::JumpIfFalsy {
                    jump_idx: get_falsy_idx(&instructions[idx + 1..])
                        .ok_or(ParsingError::UnmatchedLoopConstruct { idx })?
                        + idx
                        + 1,
                },
                Token::JumpIfTruthy => Construct::JumpIfTruthy {
                    jump_idx: get_truthy_idx(instructions[..idx].iter().copied().enumerate().rev())
                        .ok_or(ParsingError::UnmatchedLoopConstruct { idx })?,
                },
            };
            Ok(construct)
        })
        .collect()
}

fn get_truthy_idx(instructions: impl Iterator<Item = (usize, Token)>) -> Option<usize> {
    for (idx, instruction) in instructions {
        return match instruction {
            Token::JumpIfFalsy => Some(idx),
            _ => continue,
        };
    }

    None
}

fn get_falsy_idx(instructions: &[Token]) -> Option<usize> {
    for (idx, instruction) in instructions.iter().enumerate() {
        return match instruction {
            Token::JumpIfTruthy => Some(idx),
            Token::JumpIfFalsy => get_falsy_idx(&instructions[idx..]).map(|i| i + idx),
            _ => continue,
        };
    }

    None
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Construct {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Write,
    Read,
    JumpIfFalsy { jump_idx: usize },
    JumpIfTruthy { jump_idx: usize },
}

#[derive(Debug, PartialEq)]
pub enum ParsingError {
    UnmatchedLoopConstruct { idx: usize },
}

impl From<ParsingError> for Error {
    fn from(value: ParsingError) -> Self {
        Error::Parsing(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_loop() {
        let instructions = vec![
            Token::Increment,
            Token::JumpIfFalsy,
            Token::MoveRight,
            Token::Increment,
            Token::JumpIfTruthy,
        ];

        let result = parse(&instructions);

        let expected = vec![
            Construct::Increment,
            Construct::JumpIfFalsy { jump_idx: 4 },
            Construct::MoveRight,
            Construct::Increment,
            Construct::JumpIfTruthy { jump_idx: 1 },
        ];

        assert_eq!(result, Ok(expected));
    }
}

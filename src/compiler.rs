use anyhow::{bail, Result};

use crate::vm::{Opcode, StackType};
use std::{collections::HashMap, iter::Peekable, str::Chars};

#[derive(Debug)]
enum ParserState {
    LabelOrInstruction,
    Instruction,
    Argument,
}

fn is_letter(c: char) -> bool {
    c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z'
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\n'
}

fn requires_argument(instruction: &str) -> Result<bool> {
    match instruction.to_lowercase().as_str() {
        "push" | "dup" | "bnz" => Ok(true),
        "swp" | "add" | "sub" | "mul" => Ok(false),
        _ => bail!("Unknown instruction: {}", instruction),
    }
}

#[derive(Debug)]
pub struct Compiler<'a> {
    iit: Peekable<Chars<'a>>,
    state: ParserState,
    labels: HashMap<String, u16>,
    output: Vec<Opcode>,
    current_token: Vec<char>,
    current_character: Option<char>,
    current_instruction: Option<String>,
}

impl<'a> Compiler<'a> {
    pub fn new(input: &'static str) -> Self {
        Compiler {
            iit: input.chars().peekable(),
            state: ParserState::LabelOrInstruction,
            labels: HashMap::new(),
            output: vec![],
            current_token: vec![],
            current_character: None,
            current_instruction: None,
        }
    }

    fn read(&mut self) {
        self.current_character = self.iit.next();
    }

    fn read_while<P>(&mut self, pred: P)
    where
        P: Fn(char) -> bool,
    {
        loop {
            match self.iit.peek() {
                None => break,
                Some(c) => {
                    if pred(*c) {
                        self.current_token.push(*c);
                        // advance iterator if peek was successful
                        self.iit.next();
                    } else {
                        break;
                    }
                }
            }
        }
    }

    fn compile_instruction(
        &mut self,
        instruction: &str,
        argument: Option<StackType>,
    ) -> Result<()> {
        self.output.push(match instruction {
            "push" => Opcode::PushConstant(argument.unwrap()),
            "dup" => Opcode::Dup(argument.unwrap() as u16),
            "bnz" => Opcode::BranchIfNotZero(argument.unwrap() as u16),

            "drop" => Opcode::Swap,
            "swp" => Opcode::Swap,
            "add" => Opcode::Add,
            "sub" => Opcode::Subtract,
            "mul" => Opcode::Multiply,
            _ => bail!("unknown instruction {}", instruction),
        });
        Ok(())
    }

    pub fn compile(&mut self) -> Result<()> {
        loop {
            match self.state {
                ParserState::LabelOrInstruction => {
                    self.read_while(is_whitespace);
                    self.current_token = vec![];

                    self.read_while(is_letter);

                    let c = self.iit.peek();

                    let token = String::from_iter(&self.current_token);
                    println!("label or instr: {:?}, p: {:?}", token, c);

                    match c {
                        Some(c) if c == &':' => {
                            // is a label
                            self.iit.next(); // consume complete label
                            self.labels.insert(token, self.output.len() as u16);
                            self.state = ParserState::Instruction;
                        }
                        _ if token.len() > 0 => {
                            // is an instruction
                            if requires_argument(&token).unwrap() {
                                self.current_instruction = Some(String::from(token));
                                self.state = ParserState::Argument;
                            } else {
                                self.compile_instruction(&token, None).unwrap();
                                self.current_instruction = None;
                            }
                        }
                        _ => {} // ignore empty tokens
                    }
                }

                ParserState::Argument => {
                    self.read_while(is_whitespace);

                    self.current_token = vec![];
                    self.read_while(is_digit);

                    let arg = String::from_iter(&self.current_token)
                        .parse::<i16>()
                        .unwrap();

                    println!("argument: {:?}", arg);

                    self.compile_instruction(
                        &(self.current_instruction.clone()).unwrap(),
                        Some(arg),
                    )
                    .unwrap();

                    self.state = ParserState::LabelOrInstruction;
                }

                _ => break,
            }

            if self.iit.peek() == None {
                break;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_arguments() {
        let mut c = Compiler::new("add");
        c.compile().unwrap();
        assert_eq!(c.output, vec![Opcode::Add]);
    }

    #[test]
    fn with_argument() {
        let mut c = Compiler::new("push 99");
        c.compile().unwrap();

        assert_eq!(c.output, vec![Opcode::PushConstant(99i16)]);
    }

    #[test]
    fn trailing_leading_ws() {
        let mut c = Compiler::new("\n\n push    \t 99  \n\n");
        c.compile().unwrap();

        assert_eq!(c.output, vec![Opcode::PushConstant(99i16)]);
    }

    #[test]
    fn multiple_instructions() {
        let mut c = Compiler::new("  push 1\n  push 1\n  add");
        c.compile().unwrap();

        assert_eq!(
            c.output,
            vec![
                Opcode::PushConstant(1i16),
                Opcode::PushConstant(1i16),
                Opcode::Add
            ]
        );
    }
}

use anyhow::{bail, Result};

use crate::opcode::{Opcode, Reference, StackType};
use std::{collections::HashMap, iter::Peekable, str::Chars};

#[derive(Debug)]
enum ParserState {
    LabelOrInstruction,
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

fn parse_number(s: &str) -> StackType {
    s.parse::<StackType>().unwrap()
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
    current_token: Vec<char>,
    current_instruction: Option<String>,
}

impl<'a> Compiler<'a> {
    pub fn new(input: &'a str) -> Self {
        Compiler {
            iit: input.chars().peekable(),
            state: ParserState::LabelOrInstruction,
            labels: HashMap::new(),
            current_token: vec![],
            current_instruction: None,
        }
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

    fn compile_instruction(&mut self, instruction: &str, argument: Option<&str>) -> Result<Opcode> {
        Ok(match instruction {
            "push" => Opcode::PushConstant(parse_number(argument.unwrap())),
            "dup" => Opcode::Dup(parse_number(argument.unwrap()) as u16),
            "bnz" => {
                Opcode::BranchIfNotZero(Reference::Unresolved(String::from(argument.unwrap())))
            }
            "drop" => Opcode::Drop,
            "swp" => Opcode::Swap,
            "add" => Opcode::Add,
            "sub" => Opcode::Subtract,
            "mul" => Opcode::Multiply,
            "halt" => Opcode::Halt,
            _ => bail!("unknown instruction {}", instruction),
        })
    }

    pub fn compile(&mut self) -> Result<Vec<Opcode>> {
        let mut pass1 = vec![];

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
                            self.labels.insert(token, pass1.len() as u16);
                        }
                        _ if token.len() > 0 => {
                            // is an instruction
                            if requires_argument(&token).unwrap() {
                                self.current_instruction = Some(String::from(token));
                                self.state = ParserState::Argument;
                            } else {
                                pass1.push(self.compile_instruction(&token, None).unwrap());
                                self.current_instruction = None;
                            }
                        }
                        _ => {} // ignore empty tokens
                    }
                }

                ParserState::Argument => {
                    self.read_while(is_whitespace);

                    self.current_token = vec![];
                    self.read_while(|c| is_digit(c) || is_letter(c));

                    pass1.push(
                        self.compile_instruction(
                            &(self.current_instruction.clone()).unwrap(),
                            Some(&String::from_iter(&self.current_token)),
                        )
                        .unwrap(),
                    );

                    self.state = ParserState::LabelOrInstruction;
                }
            }

            if self.iit.peek() == None {
                break;
            }
        }

        Ok(pass1
            .into_iter()
            .map(|opcode| match opcode {
                Opcode::BranchIfNotZero(Reference::Unresolved(reference)) => {
                    if let Some(adr) = self.labels.get(&reference) {
                        Opcode::BranchIfNotZero(Reference::Resolved(*adr))
                    } else {
                        panic!("undefined reference: {}", reference);
                    }
                }
                opcode => opcode,
            })
            .collect::<Vec<Opcode>>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_arguments() {
        let mut c = Compiler::new("add");
        let output = c.compile().unwrap();
        assert_eq!(output, vec![Opcode::Add]);
    }

    #[test]
    fn with_argument() {
        let mut c = Compiler::new("push 99");
        let output = c.compile().unwrap();

        assert_eq!(output, vec![Opcode::PushConstant(99i16)]);
    }

    #[test]
    fn trailing_leading_ws() {
        let mut c = Compiler::new("\n\n push    \t 99  \n\n");
        let output = c.compile().unwrap();

        assert_eq!(output, vec![Opcode::PushConstant(99i16)]);
    }

    #[test]
    fn multiple_instructions() {
        let mut c = Compiler::new("  push 1\n  push 1\n  add");
        let output = c.compile().unwrap();

        assert_eq!(
            output,
            vec![
                Opcode::PushConstant(1i16),
                Opcode::PushConstant(1i16),
                Opcode::Add
            ]
        );
    }

    #[test]
    fn label_bnz() {
        let mut c = Compiler::new("loopa: push 10\n loopb:  push 10\nbnz loopb");
        let output = c.compile().unwrap();
        println!("{:?}", output);

        assert_eq!(
            output,
            vec![
                Opcode::PushConstant(10i16),
                Opcode::PushConstant(10i16),
                Opcode::BranchIfNotZero(Reference::Resolved(1))
            ]
        );
    }
}

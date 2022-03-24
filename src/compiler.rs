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
    c == ' ' || c == '\t'
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
    currentToken: Vec<char>,
    currentCharacter: Option<char>,
    currentInstruction: Option<String>,
}

impl<'a> Compiler<'a> {
    pub fn new(input: &'static str) -> Self {
        Compiler {
            iit: input.chars().peekable(),
            state: ParserState::LabelOrInstruction,
            labels: HashMap::new(),
            output: vec![],
            currentToken: vec![],
            currentCharacter: None,
            currentInstruction: None,
        }
    }

    fn read(&mut self) {
        self.currentCharacter = self.iit.next();
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
                        self.currentToken.push(*c);
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
        match instruction {
            _ => bail!("unknown instruction {}", instruction),
        }
        // self.output.push(instruction_meta.opcode);
    }

    pub fn compile(&mut self) {
        loop {
            match self.state {
                ParserState::LabelOrInstruction => {
                    self.read_while(is_whitespace);
                    self.currentToken = vec![];

                    self.read_while(is_letter);

                    let c = self.iit.peek();

                    let token = String::from_iter(&self.currentToken);
                    println!("label or instr: {:?}, p: {:?}", token, c);

                    match c {
                        Some(c) if c == &':' => {
                            // is a label
                            self.iit.next(); // consume complete label
                            self.labels.insert(token, self.output.len() as u16);
                            self.state = ParserState::Instruction;
                        }
                        _ => {
                            // is an instruction
                            if requires_argument(&token).unwrap() {
                                self.currentInstruction = Some(String::from(token));
                                self.state = ParserState::Argument;
                            } else {
                                self.compile_instruction(&token, None).unwrap();
                                self.state = ParserState::LabelOrInstruction;
                                self.currentInstruction = None;
                            }
                        }
                    }
                }

                ParserState::Argument => {
                    self.read_while(is_whitespace);

                    self.currentToken = vec![];
                    self.read_while(is_digit);

                    if let Ok(arg) = String::from_iter(&self.currentToken).parse::<i16>() {
                        println!("argument: {:?}", arg);
                    }
                }

                _ => break,
            }

            if self.iit.peek() == None {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut c = Compiler::new("   loop:\n add");
        c.compile();
        println!("output: {:#?}", c);
    }

    #[test]
    fn test2() {
        let mut c = Compiler::new("  push 12");
        c.compile();
        println!("output: {:#?}", c);
    }
}

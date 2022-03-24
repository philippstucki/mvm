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
        match instruction {
            "push" => Ok(self.output.push(Opcode::PushConstant(argument.unwrap()))),
            "dup" => Ok(self.output.push(Opcode::Dup(argument.unwrap() as u16))),
            "bnz" => Ok(self
                .output
                .push(Opcode::BranchIfNotZero(argument.unwrap() as u16))),

            "drop" => Ok(self.output.push(Opcode::Swap)),
            "swp" => Ok(self.output.push(Opcode::Swap)),
            "add" => Ok(self.output.push(Opcode::Add)),
            "sub" => Ok(self.output.push(Opcode::Subtract)),
            "mul" => Ok(self.output.push(Opcode::Multiply)),

            _ => bail!("unknown instruction {}", instruction),
        }
        // self.output.push(instruction_meta.opcode);
    }

    pub fn compile(&mut self) {
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
                                self.state = ParserState::LabelOrInstruction;
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

                    if let Ok(arg) = String::from_iter(&self.current_token).parse::<i16>() {
                        println!("argument: {:?}", arg);

                        self.compile_instruction(
                            &(self.current_instruction.clone()).unwrap(),
                            Some(arg),
                        )
                        .unwrap();
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
        let mut c = Compiler::new("  push   12");
        c.compile();
        println!("output: {:#?}", c);
    }

    #[test]
    fn test3() {
        let mut c = Compiler::new("  add\n  sub\n");
        c.compile();
        println!("output: {:#?}", c);
    }
}

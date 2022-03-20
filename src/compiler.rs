use crate::vm::Opcode;
use std::{collections::HashMap, iter::Peekable, str::Chars};

type Predicate = fn(char) -> bool;
// type CharPredicate = Fn(char) -> bool;

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
#[derive(Debug)]
pub struct Compiler<'a> {
    iit: Peekable<Chars<'a>>,
    state: ParserState,
    labels: HashMap<String, u16>,
    output: Vec<Opcode>,
    currentToken: Vec<char>,
    currentCharacter: Option<char>,
}

impl<'a> Compiler<'a> {
    pub fn new(input: &'static str) -> Self {
        Compiler {
            iit: input.chars().peekable(),
            state: ParserState::LabelOrInstruction,
            labels: HashMap::new(),
            output: Vec::new(),
            currentToken: Vec::new(),
            currentCharacter: None,
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

    fn compile_instruction(&mut self, instruction: String) {
        println!("compiling instruction: {}", instruction);

        match instruction.to_lowercase().as_str() {
            "push" => {}
            "add" => {
                self.output.push(Opcode::Add);
            }
            _ => println!("unknown instruction: {}", instruction),
        }
    }

    pub fn compile(&mut self) {
        loop {
            match self.state {
                ParserState::LabelOrInstruction => {
                    // skip whitespace here
                    self.read_while(is_letter);

                    let c = self.iit.peek();

                    let token = String::from_iter(&self.currentToken);
                    println!("loi: {:?}, p: {:?}", token, c);

                    match c {
                        Some(c) if c == &':' => {
                            // is a label
                            self.iit.next(); // consume complete label
                            self.labels.insert(token, self.output.len() as u16);
                            self.state = ParserState::Instruction;
                        }
                        _ => {
                            // is an instruction

                            self.compile_instruction(token);
                            self.state = ParserState::Argument;
                        }
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
        let mut c = Compiler::new("loop: add  12 ;comment\nsub");
        c.compile();
        println!("output: {:#?}", c);
    }

    #[test]
    fn test2() {
        let mut c = Compiler::new("addi");
        c.compile();
        println!("output: {:#?}", c);
    }
}

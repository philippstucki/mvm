use crate::vm::Opcode;
use std::str::Chars;

type Predicate = fn(char) -> bool;
// type CharPredicate = Fn(char) -> bool;

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
pub struct Compiler<'a> {
    iit: Chars<'a>,
    state: ParserState,
    output: Vec<Opcode>,
}

impl<'a> Compiler<'a> {
    pub fn new(input: &'static str) -> Self {
        Compiler {
            iit: input.chars(),
            state: ParserState::LabelOrInstruction,
            output: Vec::new(),
        }
    }

    fn read(&mut self) -> Option<char> {
        match self.iit.next() {
            None => None,
            Some(c) => Some(c),
        }
    }

    fn read_until<P>(&mut self, pred: P) -> Vec<char>
    where
        P: Fn(char) -> bool,
    {
        let mut token = Vec::new();
        loop {
            let c = self.iit.next();
            match c {
                None => break,
                Some(c) => {
                    if pred(c) {
                        break;
                    }
                }
            }
            token.push(c.unwrap());
        }
        token
    }

    fn add_label(&mut self, label: String) {
        println!("adding label: {}", label);
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
        while let Some(c) = self.iit.next() {
            if is_letter(c) {
                match self.state {
                    ParserState::LabelOrInstruction => {
                        let mut token = vec![c];
                        let mut word = self.read_until(|c| is_whitespace(c));
                        token.append(&mut word);

                        println!("token: {:?}", token);

                        let last_c = token.last();

                        match last_c {
                            Some(c) if c == &':' => {
                                self.add_label(String::from_iter(&token[0..token.len() - 1]));
                            }
                            Some(_) => {
                                self.compile_instruction(String::from_iter(token));
                            }
                            _ => {}
                        }
                    }
                    _ => panic!("Unexpected character: {}", c),
                }

                // println!("letter: {}", c);
            } else if c == ';' {
                let comment = self.read_until(|c| c == '\n');
                println!("comment: {:?}", comment)
            } else {
                println!("_  => {:?}", c)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut c = Compiler::new("add\nloop: add  12 ;comment\nsub");
        c.compile();
        println!("output: {:?}", c.output);
    }
}

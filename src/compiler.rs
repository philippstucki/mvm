use crate::vm::Opcode;
use std::str::Chars;

type Predicate = fn(char) -> bool;
// type CharPredicate = Fn(char) -> bool;

enum ParserState {
    StartOfLine,
    LabelOrInstruction,
    Argument,
    Comment,
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
}

impl<'a> Compiler<'a> {
    pub fn new(input: &'static str) -> Self {
        Compiler {
            iit: input.chars(),
            state: ParserState::StartOfLine,
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

    pub fn compile(&mut self) -> Vec<Opcode> {
        let opcodes = Vec::new();
        while let Some(c) = self.iit.next() {
            if is_letter(c) {
                match self.state {
                    ParserState::StartOfLine => {
                        let mut token = self.read_until(|c| is_whitespace(c));
                        token.insert(0, c);
                        println!("token: {:?}", token)
                    }
                    _ => panic!("Unexpected character: {}", c),
                }

                println!("letter: {}", c);
            } else if c == ';' {
                let comment = self.read_until(|c| c == '\n');
                println!("comment: {:?}", comment)
            } else {
                println!("_  => {:?}", c)
            }
        }
        opcodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut c = Compiler::new("loop: add  12 ;comment");
        let opcodes = c.compile();
    }
}

use core::str::Chars;

#[derive(Debug)]
pub enum Token {
    LABEL(String),
    INSTRUCTION(String),
    EOL,
}

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'static str) -> Self {
        Lexer {
            input: input.chars(),
        }
    }

    fn is_letter(&self, c: char) -> bool {
        c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z'
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_whitespace(&self, c: char) -> bool {
        c == ' ' || c == '\t'
    }

    fn process(&self, c: char) -> Token {
        match c {
            _ => Token::LABEL(String::from(c)),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.input.next() {
            Some(ch) => Some(self.process(ch)),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut l = Lexer::new("label: instr 1 2 3 ;comment äöü");
        for t in l {
            println!("{:?}", t);
        }
    }
}

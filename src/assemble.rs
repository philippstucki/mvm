// use std::fs
use anyhow::Result;
use std::collections::HashMap;

use crate::vm;

pub fn parse(input: &str) -> Result<Vec<vm::Opcode>> {
    let opcodes = Vec::<vm::Opcode>::new();
    let mut labels = HashMap::<&str, u16>::new();

    for c in input.chars() {
        println!("{:?}", c);

        // match c {
        //     'a'..='z' => {
        //         // let mut token =
        //     }

        // }
    }

    Ok(opcodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // parse("     add     ; äöü");
    }
}

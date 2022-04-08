use std::env;
use std::fs;

mod compiler;
mod opcode;
mod vm;

fn main() {
    let program_filename = &env::args().collect::<Vec<String>>()[1];
    let program = fs::read_to_string(program_filename).unwrap();

    println!("running {}", program_filename);

    let mut c = compiler::Compiler::new(&program);
    let opcodes = c.compile().unwrap();
    println!("compiled output: {:?}", opcodes);

    vm::run_program(vm::Program { opcodes });
}

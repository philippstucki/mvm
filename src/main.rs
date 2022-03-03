#[derive(Debug)]
enum Opcode {
    PushConstant(u16),
    Add,
}

#[derive(Debug)]
struct Program {
    opcodes: Vec<Opcode>,
}

struct Vm {
    stack: Vec<u16>,
}

fn run_program(program: Program) {
    let mut vm = Vm { stack: Vec::new() };

    for opcode in program.opcodes.into_iter() {
        match opcode {
            Opcode::PushConstant(value) => {
                println!("op: push const {}", value);
                vm.stack.push(value);
            }
            Opcode::Add => {
                println!("op: add");
                let op1 = vm.stack.pop();
                let op2 = vm.stack.pop();
                if op1.is_some() && op2.is_some() {
                    vm.stack.push(op1.unwrap() + op2.unwrap());
                } else {
                    println!("Add: not enough arguments");
                }
            }
        }
    }

    println!("{:?}", vm.stack);
}

fn main() {
    let test: Program = Program {
        opcodes: vec![
            Opcode::PushConstant(4),
            Opcode::PushConstant(3),
            Opcode::Add,
        ],
    };

    // println!("{:?}", test);
    run_program(test)
}

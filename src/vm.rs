type StackType = i16;

#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    PushConstant(StackType),

    Drop,
    Dup(u16),
    Swap,

    Add,
    Subtract,
    Multiply,

    BranchIfNotZero(u16),

    DumpStack,
    Halt,
}

#[derive(Debug)]
pub struct Program {
    pub opcodes: Vec<Opcode>,
}

struct Vm {
    stack: Vec<StackType>,
    pc: usize,
}

pub fn run_program(program: Program) {
    let mut vm = Vm {
        stack: Vec::with_capacity(200),
        pc: 0,
    };

    loop {
        if vm.pc >= program.opcodes.len() {
            break;
        }

        let opcode = program.opcodes[vm.pc];
        print!("{:04}: {:?}", vm.pc, opcode);

        vm.pc += 1;

        match opcode {
            Opcode::PushConstant(value) => {
                vm.stack.push(value);
            }

            Opcode::Drop => {
                vm.stack.pop();
            }

            Opcode::Dup(idx) => {
                vm.stack.push(vm.stack[vm.stack.len() - 1 - (idx as usize)]);
            }

            Opcode::Swap => {
                let op1 = vm.stack.pop().unwrap();
                let op2 = vm.stack.pop().unwrap();
                vm.stack.push(op1);
                vm.stack.push(op2);
            }

            Opcode::Add => {
                let op1 = vm.stack.pop();
                let op2 = vm.stack.pop();
                if op1.is_some() && op2.is_some() {
                    vm.stack.push(op1.unwrap() + op2.unwrap());
                } else {
                    println!("\nAdd: not enough arguments");
                    break;
                }
            }

            Opcode::Subtract => {
                let op1 = vm.stack.pop();
                let op2 = vm.stack.pop();
                if op1.is_some() && op2.is_some() {
                    vm.stack.push(op1.unwrap() - op2.unwrap());
                } else {
                    println!("\nSubtract: not enough arguments");
                    break;
                }
            }

            Opcode::Multiply => {
                let op1 = vm.stack.pop().unwrap();
                let op2 = vm.stack.pop().unwrap();
                vm.stack.push(op1 * op2);
            }

            Opcode::BranchIfNotZero(destination) => {
                let op = vm.stack.pop().unwrap();
                if op != 0 {
                    vm.pc = destination as usize;
                }
            }

            Opcode::DumpStack => {
                println!("\n{:?}", vm.stack);
            }

            Opcode::Halt => {
                break;
            }
        }

        println!();
    }

    println!("{:?} \n", vm.stack);
}

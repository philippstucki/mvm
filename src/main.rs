#[derive(Debug, Copy, Clone)]
struct BranchCondition {
    zero: bool,
    negative: bool,
    positive: bool,
}

#[derive(Debug, Copy, Clone)]
enum Opcode {
    PushConstant(i16),

    Drop,
    Dup(u8),
    Swap,

    Add,
    Subtract,
    Multiply,

    BranchNotZero(u16),

    DumpStack,
    Halt,
}

#[derive(Debug)]
struct Program {
    opcodes: Vec<Opcode>,
}

struct Vm {
    stack: Vec<i16>,
    pc: usize,
}

fn run_program(program: Program) {
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

            Opcode::BranchNotZero(destination) => {
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

fn main() {
    // let add_ints: Program = Program {
    //     opcodes: vec![
    //         Opcode::PushConstant(-199),
    //         Opcode::PushConstant(1),
    //         Opcode::Swap,
    //         Opcode::PushConstant(99),
    //         Opcode::Add,
    //         Opcode::Swap,
    //         Opcode::Drop,
    //     ],
    // };
    // run_program(add_ints);

    let mul_ints: Program = Program {
        opcodes: vec![
            Opcode::PushConstant(3),
            Opcode::PushConstant(4),
            Opcode::Multiply,
        ],
    };
    run_program(mul_ints);

    // let int_sum: Program = Program {
    //     opcodes: vec![
    //         // init: [max, i, sum]
    //         Opcode::PushConstant(6),
    //         Opcode::PushConstant(1),
    //         Opcode::PushConstant(0),
    //         // sum += i
    //         Opcode::Dup(1),
    //         Opcode::Add,
    //         // i++
    //         Opcode::Swap,
    //         Opcode::PushConstant(1),
    //         Opcode::Add,
    //         Opcode::Swap,
    //         // i <= max ?
    //         Opcode::Dup(2),
    //         Opcode::PushConstant(1),
    //         Opcode::Add,
    //         Opcode::Dup(2),
    //         Opcode::DumpStack,
    //         Opcode::Subtract,
    //         Opcode::BranchNotZero(3),
    //     ],
    // };
    // run_program(int_sum);

    let factorial: Program = Program {
        opcodes: vec![
            // init: [max, i, prod]
            Opcode::PushConstant(7),
            Opcode::PushConstant(1),
            Opcode::PushConstant(1),
            // sum *= i
            Opcode::Dup(1),
            Opcode::Multiply,
            // i++
            Opcode::Swap,
            Opcode::PushConstant(1),
            Opcode::Add,
            Opcode::Swap,
            // i <= max ?
            Opcode::Dup(2),
            Opcode::Dup(2),
            Opcode::DumpStack,
            Opcode::Subtract,
            Opcode::BranchNotZero(3),
        ],
    };
    run_program(factorial);
}

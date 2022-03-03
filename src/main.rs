#[derive(Debug, Copy, Clone)]
struct BranchCondition {
    zero: bool,
    negative: bool,
    positive: bool,
}

#[derive(Debug, Copy, Clone)]
enum Opcode {
    PushConstant(i16),
    Pop,
    Dup,
    Add,
    Subtract,
    JumpConditional(BranchCondition, u16),
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
        stack: Vec::new(),
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

            Opcode::Pop => {
                vm.stack.pop();
            }

            Opcode::Dup => {
                vm.stack.push(vm.stack[vm.stack.len() - 1]);
            }

            Opcode::Add => {
                let op1 = vm.stack.pop();
                let op2 = vm.stack.pop();
                if op1.is_some() && op2.is_some() {
                    vm.stack.push(op1.unwrap() + op2.unwrap());
                } else {
                    println!("Add: not enough arguments");
                    break;
                }
            }

            Opcode::Subtract => {
                let op1 = vm.stack.pop();
                let op2 = vm.stack.pop();
                if op1.is_some() && op2.is_some() {
                    vm.stack.push(op1.unwrap() - op2.unwrap());
                } else {
                    println!("Add: not enough arguments");
                    break;
                }
            }

            Opcode::JumpConditional(branch_condition, address) => {
                if let Some(op) = vm.stack.pop() {
                    vm.stack.push(op);
                    if (branch_condition.negative && op < 0)
                        || (branch_condition.zero && op == 0)
                        || (branch_condition.positive && op > 0)
                    {
                        vm.pc = address as usize;
                    }
                } else {
                    println!("JumpConditional: not enough arguments");
                    break;
                }
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
    let add_ints: Program = Program {
        opcodes: vec![
            /* 0 */
            Opcode::PushConstant(99),
            /* 1 */
            Opcode::PushConstant(1),
            /* 2 */
            Opcode::Add,
        ],
    };
    run_program(add_ints);

    let int_sum: Program = Program {
        opcodes: vec![
            /* 0 */
            Opcode::PushConstant(1),
            /* 1 */
            Opcode::Dup,
            /* 2 */
            Opcode::Add,
            /* 3 */
            // Opcode::JumpConditional(
            //     BranchCondition {
            //         zero: false,
            //         negative: false,
            //         positive: true,
            //     },
            //     1,
            // ),
        ],
    };
    run_program(int_sum);
}

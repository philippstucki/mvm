mod compiler;
mod opcode;
mod vm;

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

    let mul_ints: vm::Program = vm::Program {
        opcodes: vec![
            opcode::Opcode::PushConstant(3),
            opcode::Opcode::PushConstant(4),
            opcode::Opcode::Multiply,
        ],
    };
    vm::run_program(mul_ints);

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

    let factorial: vm::Program = vm::Program {
        opcodes: vec![
            // init: [max, i, prod]
            opcode::Opcode::PushConstant(6),
            opcode::Opcode::PushConstant(1),
            opcode::Opcode::PushConstant(1),
            // sum *= i
            opcode::Opcode::Dup(1),
            opcode::Opcode::Multiply,
            // i++
            opcode::Opcode::Swap,
            opcode::Opcode::PushConstant(1),
            opcode::Opcode::Add,
            opcode::Opcode::Swap,
            // i <= max ?
            opcode::Opcode::Dup(2),
            opcode::Opcode::Dup(2),
            opcode::Opcode::Subtract,
            opcode::Opcode::BranchIfNotZero(opcode::Reference::Resolved(3)),
        ],
    };
    vm::run_program(factorial);
}

mod compiler;
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
            vm::Opcode::PushConstant(3),
            vm::Opcode::PushConstant(4),
            vm::Opcode::Multiply,
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
            vm::Opcode::PushConstant(6),
            vm::Opcode::PushConstant(1),
            vm::Opcode::PushConstant(1),
            // sum *= i
            vm::Opcode::Dup(1),
            vm::Opcode::Multiply,
            // i++
            vm::Opcode::Swap,
            vm::Opcode::PushConstant(1),
            vm::Opcode::Add,
            vm::Opcode::Swap,
            // i <= max ?
            vm::Opcode::Dup(2),
            vm::Opcode::Dup(2),
            vm::Opcode::Subtract,
            vm::Opcode::BranchIfNotZero(3),
        ],
    };
    vm::run_program(factorial);
}

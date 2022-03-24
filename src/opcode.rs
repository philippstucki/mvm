type StackType = i16;
type AddressType = u16;

enum Opcode<S, A> {
    PushConstant(S),
    Drop,
    Dup(A),
    Swap,
    Add,
    Subtract,
    Multiply,
    BranchIfNotZero(A),
    Halt,
}

pub type Instruction = Opcode<StackType, AddressType>;

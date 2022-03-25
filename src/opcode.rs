pub type StackType = i16;
pub type AddressType = u16;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GenericOpcode<S, A> {
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

pub type Opcode = GenericOpcode<StackType, AddressType>;

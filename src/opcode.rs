pub type StackType = i16;
pub type AddressType = u16;

#[derive(Debug, PartialEq)]
pub enum Reference<A> {
    Resolved(A),
    Unresolved(String),
}

#[derive(Debug, PartialEq)]
pub enum GenericOpcode<S, A> {
    PushConstant(S),
    Drop,
    Dup(A),
    Swap,
    Add,
    Subtract,
    Multiply,
    BranchIfNotZero(Reference<A>),
    Halt,
}

pub type Opcode = GenericOpcode<StackType, AddressType>;

pub type StackType = i16;
pub type AddressType = u16;
pub type ReferenceType = [char; 20];

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Reference<A> {
    Resolved(A),
    Unresolved(ReferenceType),
}

#[derive(Debug, Copy, Clone, PartialEq)]
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

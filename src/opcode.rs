#[derive(PartialEq)]
pub enum OpCode {
    Halt,
    Add,
    Subtract,
    And,
    Xor,
    LeftShift,
    RightShift,
    LoadAddress,
    Load,
    Store,
    LoadIndirect,
    StoreIndirect,
    BranchZero,
    BranchPositive,
    JumpRegister,
    JumpAndLink,
}

impl OpCode {
    pub fn new(op: i16) -> Self {
        match op {
            0 => Self::Halt,
            1 => Self::Add,
            2 => Self::Subtract,
            3 => Self::And,
            4 => Self::Xor,
            5 => Self::LeftShift,
            6 => Self::RightShift,
            7 => Self::LoadAddress,
            8 => Self::Load,
            9 => Self::Store,
            10 => Self::LoadIndirect,
            11 => Self::StoreIndirect,
            12 => Self::BranchZero,
            13 => Self::BranchPositive,
            14 => Self::JumpRegister,
            15 => Self::JumpAndLink,
            _ => unreachable!(),
        }
    }
}

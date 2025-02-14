#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    Hlt,

    // move to register
    MoveImmRegByte,
    MoveImmRegShort,
    MoveImmRegLong,
    
    MoveRegRegByte,
    MoveRegRegShort,
    MoveRegRegLong,

    MoveAbsRegByte,
    MoveAbsRegShort,
    MoveAbsRegLong,

    MoveMemRegByte,
    MoveMemRegShort,
    MoveMemRegLong,

    MoveIndirectRegByte,
    MoveIndirectRegShort,
    MoveIndirectRegLong,
    
    // move to absolute memory
    MoveImmAbsByte,
    MoveImmAbsShort,
    MoveImmAbsLong,

    MoveRegAbsByte,
    MoveRegAbsShort,
    MoveRegAbsLong,

    MoveAbsAbsByte,
    MoveAbsAbsShort,
    MoveAbsAbsLong,

    MoveMemAbsByte,
    MoveMemAbsShort,
    MoveMemAbsLong,

    MoveIndirectAbsByte,
    MoveIndirectAbsShort,
    MoveIndirectAbsLong,
    
    // move to relative memory
    MoveImmMemByte,
    MoveImmMemShort,
    MoveImmMemLong,
    
    MoveRegMemByte,
    MoveRegMemShort,
    MoveRegMemLong,
    
    MoveAbsMemByte,
    MoveAbsMemShort,
    MoveAbsMemLong,

    MoveMemMemByte,
    MoveMemMemShort,
    MoveMemMemLong,

    MoveIndirectMemByte,
    MoveIndirectMemShort,
    MoveIndirectMemLong,
    
    // move to indirect memory
    MoveImmIndirectByte,
    MoveImmIndirectShort,
    MoveImmIndirectLong,
    
    MoveRegIndirectByte,
    MoveRegIndirectShort,
    MoveRegIndirectLong,
    
    MoveAbsIndirectByte,
    MoveAbsIndirectShort,
    MoveAbsIndirectLong,
    
    MoveMemIndirectByte,
    MoveMemIndirectShort,
    MoveMemIndirectLong,
    
    MoveIndirectIndirectByte,
    MoveIndirectIndirectShort,
    MoveIndirectIndirectLong,

    // Add
    AddByteImm,
    AddShortImm,
    AddLongImm,

    AddByteReg,
    AddShortReg,
    AddLongReg,

    // Add Carry
    AddCarryByteImm,
    AddCarryShortImm,
    AddCarryLongImm,

    AddCarryByteReg,
    AddCarryShortReg,
    AddCarryLongReg,

    // Sub
    SubByteImm,
    SubShortImm,
    SubLongImm,

    SubByteReg,
    SubShortReg,
    SubLongReg,

    // Sub Borrow
    SubBorrowByteImm,
    SubBorrowShortImm,
    SubBorrowLongImm,

    SubBorrowByteReg,
    SubBorrowShortReg,
    SubBorrowLongReg,

    // Mul
    MulByteImm,
    MulShortImm,
    MulLongImm,

    MulByteReg,
    MulShortReg,
    MulLongReg,

    // Div
    DivByteImm,
    DivShortImm,
    DivLongImm,

    DivByteReg,
    DivShortReg,
    DivLongReg,

    // Signed Mul
    SignedMulByteImm,
    SignedMulShortImm,
    SignedMulLongImm,

    SignedMulByteReg,
    SignedMulShortReg,
    SignedMulLongReg,

    // Signed Div
    SignedDivByteImm,
    SignedDivShortImm,
    SignedDivLongImm,

    SignedDivByteReg,
    SignedDivShortReg,
    SignedDivLongReg,

    // And
    AndByteImm,
    AndShortImm,
    AndLongImm,

    AndByteReg,
    AndShortReg,
    AndLongReg,

    // Or
    OrByteImm,
    OrShortImm,
    OrLongImm,

    OrByteReg,
    OrShortReg,
    OrLongReg,

    // Xor
    XorByteImm,
    XorShortImm,
    XorLongImm,

    XorByteReg,
    XorShortReg,
    XorLongReg,

    // Push
    PushByteImm,
    PushShortImm,
    PushLongImm,

    PushByteReg,
    PushShortReg,
    PushLongReg,

    // Compare
    CompareByteImm,
    CompareShortImm,
    CompareLongImm,

    CompareByteReg,
    CompareShortReg,
    CompareLongReg,

    // Logical Shift Left
    LogShiftLeftByteImm,
    LogShiftLeftShortImm,
    LogShiftLeftLongImm,

    LogShiftLeftByteReg,
    LogShiftLeftShortReg,
    LogShiftLeftLongReg,


    // Logical Shift Right
    LogShiftRightByteImm,
    LogShiftRightShortImm,
    LogShiftRightLongImm,

    LogShiftRightByteReg,
    LogShiftRightShortReg,
    LogShiftRightLongReg,


    // Arithmetic Shift Left
    ArithShiftLeftByteImm,
    ArithShiftLeftShortImm,
    ArithShiftLeftLongImm,

    ArithShiftLeftByteReg,
    ArithShiftLeftShortReg,
    ArithShiftLeftLongReg,


    // Arithmetic Shift Right
    ArithShiftRightByteImm,
    ArithShiftRightShortImm,
    ArithShiftRightLongImm,

    ArithShiftRightByteReg,
    ArithShiftRightShortReg,
    ArithShiftRightLongReg,


    // Rotate Left
    RotateLeftByteImm,
    RotateLeftShortImm,
    RotateLeftLongImm,

    RotateLeftByteReg,
    RotateLeftShortReg,
    RotateLeftLongReg,


    // Rotate Right
    RotateRightByteImm,
    RotateRightShortImm,
    RotateRightLongImm,

    RotateRightByteReg,
    RotateRightShortReg,
    RotateRightLongReg,


    // Pop
    PopByte,
    PopShort,
    PopLong,

    // Negate
    NegateByte,
    NegateShort,
    NegateLong,

    // Not
    NotByte,
    NotShort,
    NotLong,

    // Increment
    IncrementByte,
    IncrementShort,
    IncrementLong,

    // Decrement
    DecrementByte,
    DecrementShort,
    DecrementLong,

    // Jumps
    JumpEqual,
    JumpNotEqual,
    JumpGreater,
    JumpGreaterEqual,
    JumpLess,
    JumpLessEqual,
    JumpSignedGreater,
    JumpSignedGreaterEqual,
    JumpSignedLess,
    JumpSignedLessEqual,
    JumpImm,
    JumpReg,
    
    // TODO: implement idt & io ports.
    Interrupt,
    InterruptReturn,
    // OutByte,
    // InByte,
    Call,
    Return,
    Syscall,
    ClearCarry,
    
    Nop,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        if value > Opcode::Nop as u8 {
            panic!("Invalid opcode: {}", value);
        }
        unsafe {
            return std::mem::transmute::<u8, Opcode>(value);
        }
    }
}

impl Opcode {
    pub fn operand_sizes(&self) -> (usize, usize) {
        match *self {
            // binary ops

            // first arg byte
            // second arg byte
            Opcode::MoveImmRegByte
            | Opcode::MoveImmIndirectByte
            | Opcode::MoveIndirectRegByte
            | Opcode::MoveIndirectRegShort
            | Opcode::MoveIndirectRegLong
            | Opcode::MoveIndirectIndirectByte
            | Opcode::MoveIndirectIndirectShort
            | Opcode::MoveIndirectIndirectLong
            | Opcode::MoveRegRegByte
            | Opcode::MoveRegRegShort
            | Opcode::MoveRegRegLong
            | Opcode::MoveRegIndirectByte
            | Opcode::MoveRegIndirectShort
            | Opcode::MoveRegIndirectLong => (1, 1),
            // second arg short
            Opcode::MoveImmRegShort
            | Opcode::MoveImmIndirectShort => (1, 2),
            // second arg long
            Opcode::MoveImmRegLong
            | Opcode::MoveImmIndirectLong
            | Opcode::MoveAbsRegByte
            | Opcode::MoveAbsRegShort
            | Opcode::MoveAbsRegLong
            | Opcode::MoveMemRegByte
            | Opcode::MoveMemRegShort
            | Opcode::MoveMemRegLong
            | Opcode::MoveAbsIndirectByte
            | Opcode::MoveAbsIndirectShort
            | Opcode::MoveAbsIndirectLong
            | Opcode::MoveMemIndirectByte
            | Opcode::MoveMemIndirectShort
            | Opcode::MoveMemIndirectLong => (1, 4),

            // first arg long
            // second arg byte            
            Opcode::MoveImmMemByte
            | Opcode::MoveImmAbsByte
            | Opcode::MoveRegMemByte
            | Opcode::MoveRegMemShort
            | Opcode::MoveRegMemLong
            | Opcode::MoveRegAbsByte
            | Opcode::MoveRegAbsShort
            | Opcode::MoveRegAbsLong
            | Opcode::MoveIndirectMemByte
            | Opcode::MoveIndirectMemShort
            | Opcode::MoveIndirectMemLong
            | Opcode::MoveIndirectAbsByte
            | Opcode::MoveIndirectAbsShort
            | Opcode::MoveIndirectAbsLong => (4, 1),
            // second arg short
            Opcode::MoveImmMemShort
            | Opcode::MoveImmAbsShort => (4, 2),
            // second arg long
            Opcode::MoveImmMemLong
            | Opcode::MoveAbsMemByte
            | Opcode::MoveAbsMemShort
            | Opcode::MoveAbsMemLong
            | Opcode::MoveMemMemByte
            | Opcode::MoveMemMemShort
            | Opcode::MoveMemMemLong
            | Opcode::MoveImmAbsLong
            | Opcode::MoveAbsAbsByte
            | Opcode::MoveAbsAbsShort
            | Opcode::MoveAbsAbsLong
            | Opcode::MoveMemAbsByte
            | Opcode::MoveMemAbsShort
            | Opcode::MoveMemAbsLong => (4, 4),

            // unary ops

            // regs
            Opcode::AddByteReg
            | Opcode::AddShortReg
            | Opcode::AddLongReg
            | Opcode::SubByteReg
            | Opcode::SubShortReg
            | Opcode::SubLongReg
            | Opcode::MulByteReg
            | Opcode::MulShortReg
            | Opcode::MulLongReg
            | Opcode::DivByteReg
            | Opcode::DivShortReg
            | Opcode::DivLongReg
            | Opcode::AndByteReg
            | Opcode::AndShortReg
            | Opcode::AndLongReg
            | Opcode::PushByteReg
            | Opcode::PushShortReg
            | Opcode::PushLongReg
            | Opcode::AddCarryByteReg
            | Opcode::AddCarryShortReg
            | Opcode::AddCarryLongReg
            | Opcode::SubBorrowByteReg
            | Opcode::SubBorrowShortReg
            | Opcode::SubBorrowLongReg
            | Opcode::SignedMulByteReg
            | Opcode::SignedMulShortReg
            | Opcode::SignedMulLongReg
            | Opcode::SignedDivByteReg
            | Opcode::SignedDivShortReg
            | Opcode::SignedDivLongReg
            | Opcode::OrByteReg
            | Opcode::OrShortReg
            | Opcode::OrLongReg
            | Opcode::XorByteReg
            | Opcode::XorShortReg
            | Opcode::XorLongReg
            | Opcode::CompareByteReg
            | Opcode::CompareShortReg
            | Opcode::CompareLongReg
            | Opcode::JumpReg
            | Opcode::LogShiftLeftByteReg
            | Opcode::LogShiftLeftShortReg
            | Opcode::LogShiftLeftLongReg
            | Opcode::LogShiftRightByteReg
            | Opcode::LogShiftRightShortReg
            | Opcode::LogShiftRightLongReg
            | Opcode::ArithShiftLeftByteReg
            | Opcode::ArithShiftLeftShortReg
            | Opcode::ArithShiftLeftLongReg
            | Opcode::ArithShiftRightByteReg
            | Opcode::ArithShiftRightShortReg
            | Opcode::ArithShiftRightLongReg
            | Opcode::RotateLeftByteReg
            | Opcode::RotateLeftShortReg
            | Opcode::RotateLeftLongReg
            | Opcode::RotateRightByteReg
            | Opcode::RotateRightShortReg
            | Opcode::RotateRightLongReg
            // reg only
            | Opcode::PopByte
            | Opcode::PopShort
            | Opcode::PopLong
            | Opcode::NegateByte
            | Opcode::NegateShort
            | Opcode::NegateLong
            | Opcode::IncrementByte
            | Opcode::IncrementShort
            | Opcode::IncrementLong
            | Opcode::DecrementByte
            | Opcode::DecrementShort
            | Opcode::DecrementLong
            | Opcode::NotByte
            | Opcode::NotShort
            | Opcode::NotLong
            // immediate bytes
            | Opcode::AddByteImm
            | Opcode::SubByteImm
            | Opcode::MulByteImm
            | Opcode::DivByteImm
            | Opcode::AndByteImm
            | Opcode::PushByteImm
            | Opcode::CompareByteImm
            | Opcode::AddCarryByteImm
            | Opcode::SubBorrowByteImm
            | Opcode::SignedMulByteImm
            | Opcode::SignedDivByteImm
            | Opcode::OrByteImm
            | Opcode::XorByteImm
            | Opcode::LogShiftLeftByteImm
            | Opcode::LogShiftRightByteImm
            | Opcode::ArithShiftLeftByteImm
            | Opcode::ArithShiftRightByteImm
            | Opcode::RotateLeftByteImm
            | Opcode::RotateRightByteImm
            // other
            | Opcode::Interrupt
            | Opcode::Syscall => (1,0),

            // short imm
            Opcode::AddShortImm
            | Opcode::SubShortImm
            | Opcode::MulShortImm
            | Opcode::DivShortImm
            | Opcode::AndShortImm
            | Opcode::PushShortImm
            | Opcode::CompareShortImm
            | Opcode::AddCarryShortImm
            | Opcode::SubBorrowShortImm
            | Opcode::SignedMulShortImm
            | Opcode::SignedDivShortImm
            | Opcode::OrShortImm
            | Opcode::XorShortImm
            | Opcode::LogShiftLeftShortImm
            | Opcode::LogShiftRightShortImm
            | Opcode::ArithShiftLeftShortImm
            | Opcode::ArithShiftRightShortImm
            | Opcode::RotateLeftShortImm
            | Opcode::RotateRightShortImm => (2,0),

            // long imm
            Opcode::AddLongImm
            | Opcode::SubLongImm
            | Opcode::MulLongImm
            | Opcode::DivLongImm
            | Opcode::AndLongImm
            | Opcode::PushLongImm
            | Opcode::CompareLongImm
            | Opcode::AddCarryLongImm
            | Opcode::SubBorrowLongImm
            | Opcode::SignedMulLongImm
            | Opcode::SignedDivLongImm
            | Opcode::OrLongImm
            | Opcode::XorLongImm
            | Opcode::LogShiftLeftLongImm
            | Opcode::LogShiftRightLongImm
            | Opcode::ArithShiftLeftLongImm
            | Opcode::ArithShiftRightLongImm
            | Opcode::RotateLeftLongImm
            | Opcode::RotateRightLongImm
            // jumps
            | Opcode::JumpEqual
            | Opcode::JumpNotEqual
            | Opcode::JumpGreater
            | Opcode::JumpLess
            | Opcode::JumpImm
            | Opcode::JumpGreaterEqual
            | Opcode::JumpLessEqual
            | Opcode::JumpSignedGreater
            | Opcode::JumpSignedGreaterEqual
            | Opcode::JumpSignedLess
            | Opcode::JumpSignedLessEqual
            // call
            | Opcode::Call => (4, 0),

            // nullary ops
            Opcode::InterruptReturn
            | Opcode::Return
            | Opcode::Hlt
            | Opcode::ClearCarry
            | Opcode::Nop => (0,0),
        }
    }
}
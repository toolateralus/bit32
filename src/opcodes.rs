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

    AddByteMem,
    AddShortMem,
    AddLongMem,

    // Sub
    SubByteImm,
    SubShortImm,
    SubLongImm,

    SubByteReg,
    SubShortReg,
    SubLongReg,

    SubByteMem,
    SubShortMem,
    SubLongMem,

    // Mul
    MulByteImm,
    MulShortImm,
    MulLongImm,

    MulByteReg,
    MulShortReg,
    MulLongReg,

    MulByteMem,
    MulShortMem,
    MulLongMem,

    // Div
    DivByteImm,
    DivShortImm,
    DivLongImm,

    DivByteReg,
    DivShortReg,
    DivLongReg,

    DivByteMem,
    DivShortMem,
    DivLongMem,

    // And
    AndByteImm,
    AndShortImm,
    AndLongImm,

    AndByteReg,
    AndShortReg,
    AndLongReg,

    AndByteMem,
    AndShortMem,
    AndLongMem,

    // Push
    PushByteImm,
    PushShortImm,
    PushLongImm,

    PushByteReg,
    PushShortReg,
    PushLongReg,

    PushByteMem,
    PushShortMem,
    PushLongMem,

    // Pop
    PopByteReg,
    PopShortReg,
    PopLongReg,

    PopByteMem,
    PopShortMem,
    PopLongMem,

    // Jumps
    JumpEqual,
    JumpNotEqual,
    JumpGreater,
    JumpLess,
    JumpImm,
    JumpReg,

    // Compare
    CompareByteImm,
    CompareShortImm,
    CompareLongImm,

    CompareReg,
    
    // TODO: implement idt & io ports.
    Interrupt,
    InterruptReturn,
    // OutByte,
    // InByte,
    Call,
    Return,
    RustCall,
    
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
            | Opcode::PopByteReg
            | Opcode::PopShortReg
            | Opcode::PopLongReg
            | Opcode::JumpReg
            | Opcode::CompareReg
            // immediate bytes
            | Opcode::AddByteImm
            | Opcode::SubByteImm
            | Opcode::MulByteImm
            | Opcode::DivByteImm
            | Opcode::AndByteImm
            | Opcode::PushByteImm
            | Opcode::CompareByteImm
            // other
            | Opcode::Interrupt
            | Opcode::RustCall => (1,0),

            // shorts imm
            Opcode::AddShortImm
            | Opcode::SubShortImm
            | Opcode::MulShortImm
            | Opcode::DivShortImm
            | Opcode::AndShortImm
            | Opcode::PushShortImm
            | Opcode::CompareShortImm => (2,0),

            // long imm
            Opcode::AddLongImm
            | Opcode::SubLongImm
            | Opcode::MulLongImm
            | Opcode::DivLongImm
            | Opcode::AndLongImm
            | Opcode::PushLongImm
            | Opcode::CompareLongImm
            // mem
            | Opcode::AddByteMem
            | Opcode::AddShortMem
            | Opcode::AddLongMem
            | Opcode::SubByteMem
            | Opcode::SubShortMem
            | Opcode::SubLongMem
            | Opcode::MulByteMem
            | Opcode::MulShortMem
            | Opcode::MulLongMem
            | Opcode::DivByteMem
            | Opcode::DivShortMem
            | Opcode::DivLongMem
            | Opcode::AndByteMem
            | Opcode::AndShortMem
            | Opcode::AndLongMem
            | Opcode::PushByteMem
            | Opcode::PushShortMem
            | Opcode::PushLongMem
            | Opcode::PopByteMem
            | Opcode::PopShortMem
            | Opcode::PopLongMem
            // other
            | Opcode::JumpEqual
            | Opcode::JumpNotEqual
            | Opcode::JumpGreater
            | Opcode::JumpLess
            | Opcode::JumpImm
            | Opcode::Call => (4, 0),

            // nullary ops
            Opcode::InterruptReturn
            | Opcode::Return
            | Opcode::Nop
            | Opcode::Hlt => (0,0)
        }
    }
}
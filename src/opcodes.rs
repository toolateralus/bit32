#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    Hlt,
    // Move instructions
    MoveImmRegByte,
    MoveImmRegShort,
    MoveImmRegLong,
    
    MoveRegRegByte,
    MoveRegRegShort,
    MoveRegRegLong,
    
    MoveRegMemByte,
    MoveRegMemShort,
    MoveRegMemLong,

    MoveMemRegByte,
    MoveMemRegShort,
    MoveMemRegLong,

    MoveMemMemByte,
    MoveMemMemShort,
    MoveMemMemLong,
    
    MoveRegIndirectByte,
    MoveRegIndirectShort,
    MoveRegIndirectLong,
    
    MoveMemIndirectByte,
    MoveMemIndirectShort,
    MoveMemIndirectLong,

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

    // Jumps
    JumpEqual,
    JumpNotEqual,
    JumpImm,
    JumpReg,

    // Compare
    CompareByteImm,
    CompareShortImm,
    CompareLongImm,

    CompareReg,

    // TODO: implement idt & io ports.
    // Interrupt,
    // OutByte,
    // InByte,
    Call,
    Return,

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

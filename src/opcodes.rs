
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Opcode {
  Hlt,
  // Move instructions
  // Byte
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
  
  PushByteImm,
  PushShortImm,
  PushLongImm,
  
  PushByteReg,
  PushShortReg,
  PushLongReg,
  
  PushByteMem,
  PushShortMem,
  PushLongMem,
  
  PopByteReg,
  PopShortReg,
  PopLongReg,
  
  PopByteMem,
  PopShortMem,
  PopLongMem,
  
  AddByte,
  AddShort,
  AddLong,
  
  SubByte,
  SubShort,
  SubLong,
  
  MulByte,
  MulShort,
  MulLong,
  
  DivByte,
  DivShort,
  DivLong,
  
  AndByteImm,
  AndShortImm,
  AndLongImm,
  
  
  JumpEqual,
  JumpNotEqual,
  
  CompareByteImm,
  CompareShortImm,
  CompareLongImm,
  
  CompareReg,
  
  // TODO: implement idt & io ports.
  // Interrupt,
  // OutByte,
  // InByte,
  
  Jump,
  JumpReg,
  
  Call,
  Return,
  
  Nop,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
      let opcode : Opcode;
      unsafe {
        opcode = std::mem::transmute::<u8, Opcode>(value);
      }
      return opcode;
    }
}

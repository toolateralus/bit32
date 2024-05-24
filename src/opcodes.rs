
#[repr(u8)]
#[derive(Debug)]
pub enum Opcode {
  Hlt,
  // Move instructions
  // Byte
  MoveImmRegByte,
  MoveRegRegByte,
  MoveRegMemByte,
  MoveMemMemByte,
  MoveMemRegByte,
  
  // Short
  MoveImmRegShort,
  MoveRegRegShort,
  MoveRegMemShort,
  MoveMemRegShort,
  MoveMemMemShort,
  
  // Longs
  MoveImmRegLong,
  MoveRegRegLong,
  MoveRegMemLong,
  MoveMemRegLong,
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
  AndByteReg,
  AndByteMem,
  
  AndShortImm,
  AndShortReg,
  AndShortMem,
  
  AndLongImm,
  AndLongReg,
  AndLongMem,
  
  
  Call,
  Return,
  
  // NOT YET IMPLEMENTED BELOW
  
  OrByteImm,
  OrByteReg,
  OrByteMem,
  
  OrShortImm,
  OrShortReg,
  OrShortMem,
  
  
  OrLongImm,
  OrLongReg,
  OrLongMem,
  
  XorByteImm,
  XorByteReg,
  XorByteMem,
  
  XorShortImm,
  XorShortReg,
  XorShortMem,
  
  XorLongImm,
  XorLongReg,
  XorLongMem,
  
  NotByteImm,
  NotByteReg,
  NotByteMem,
  
  NotShortImm,
  NotShortReg,
  NotShortMem,
  
  NotLongImm,
  NotLongReg,
  NotLongMem,
  
  ShiftLeftByteImm,
  ShiftLeftByteReg,
  ShiftLeftByteMem,
  
  ShiftLeftShortImm,
  ShiftLeftShortReg,
  ShiftLeftShortMem,
  
  ShiftLeftLongImm,
  ShiftLeftLongReg,
  ShiftLeftLongMem,
  
  ShiftRightByteImm,
  ShiftRightByteReg,
  ShiftRightByteMem,
  
  ShiftRightShortImm,
  ShiftRightShortReg,
  ShiftRightShortMem,
  
  ShiftRightLongImm,
  ShiftRightLongReg,
  ShiftRightLongMem,
  
  EqualByte,
  EqualShort,
  EqualLong,
  NotEqualByte,
  NotEqualShort,
  NotEqualLong,
  LessThanByte,
  LessThanShort,
  LessThanLong,
  GreaterThanByte,
  GreaterThanShort,
  GreaterThanLong,
 
  
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

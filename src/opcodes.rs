
#[repr(u8)]
#[derive(Debug)]
pub enum Opcode {
  Hlt,
  // Move instructions
  // Byte
  MoveRegRegByte,
  MoveRegMemByte,
  MoveMemMemByte,
  MoveMemRegByte,
  
  // Short
  MoveRegRegShort,
  MoveRegMemShort,
  MoveMemRegShort,
  MoveMemMemShort,
  
  // Longs
  MoveRegRegLong,
  MoveRegMemLong,
  MoveMemRegLong,
  MoveMemMemLong,
  
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
  
  AndByteImm,
  AndByteReg,
  AndByteMem,
  
  // NOT YET IMPLEMENTED BELOW
  AndShortImm,
  AndShortReg,
  AndShortMem,
  
  AndLongImm,
  AndLongReg,
  AndLongMem,
  
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

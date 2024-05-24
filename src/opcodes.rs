
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
  
  // NOT YET IMPLEMENTED BELOW
  AndByte,
  AndShort,
  AndLong,
  
  OrByte,
  OrShort,
  OrLong,
  
  XorByte,
  XorShort,
  XorLong,
  
  NotByte,
  NotShort,
  NotLong,
  
  ShiftLeftByte,
  ShiftLeftShort,
  ShiftLeftLong,
  ShiftRightByte,
  ShiftRightShort,
  ShiftRightLong,
  
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

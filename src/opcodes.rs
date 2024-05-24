
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

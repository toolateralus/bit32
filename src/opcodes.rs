
#[repr(u8)]
#[derive(Debug)]
pub enum Opcode {
  Hlt,
  MoveRegRegLong,
  MoveRegRegShort,
  MoveRegRegByte,
  
  MoveRegMemByte,
  MoveMemMemByte,
  MoveMemRegByte,
  MoveRegMemShort,
  MoveMemRegShort,
  MoveRegMemLong,
  MoveMemRegLong,
  MoveMemMemShort,
  MoveMemMemLong,
  Add,
  Sub,
  Mul,
  Div,
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

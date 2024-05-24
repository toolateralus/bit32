
#[cfg(test)]
mod tests {
  mod mov_tests {
    use crate::{cpu::Cpu, opcodes::Opcode};
    
    #[test]
  fn test_mov_reg_reg_byte() {
    let mut cpu = Cpu::new();
    cpu.registers[1] = 10;
    cpu.load_program(&[Opcode::MoveRegRegByte as u8, 0, 1]);
    cpu.run();
    assert_eq!(cpu.registers[0], cpu.registers[1]);
    assert_eq!(cpu.ip, 4);
  }
  #[test]
  fn test_mov_reg_mem_byte() {
    let mut cpu = Cpu::new();   
    cpu.registers[0] = 10;
    cpu.load_program(&[Opcode::MoveRegMemByte as u8, 100, 0]);
    cpu.run();
    assert_eq!(cpu.memory.buffer[100], 10);
    assert_eq!(cpu.ip, 7)
  }
  #[test]
  fn test_mov_mem_mem_byte() {
    let mut cpu = Cpu::new();
    cpu.memory.buffer[10] = 100;
    
    cpu.load_program(&[Opcode::MoveMemMemByte as u8, 255, 1, 0, 0, 10]);
    
    cpu.run();
    
    assert_eq!(100, cpu.memory.buffer[511]);
  }
  #[test]
  fn test_move_mem_reg_byte() {
    let mut cpu = Cpu::new();
    
    cpu.memory.buffer[511] = 250;
    cpu.load_program(&[Opcode::MoveMemRegByte as u8, 10, 255, 1, 0, 0]);
    cpu.run();
    
    assert_eq!(cpu.registers[10], 250);
  }
  

  #[test]
  fn test_mov_mem_reg_short() {
    let mut cpu = Cpu::new();
    
    cpu.memory.set_short(511, 0xBEEF);
    cpu.load_program(&[Opcode::MoveMemRegShort as u8, 10, 255, 1, 0, 0]);
    cpu.run();
    
    assert_eq!(cpu.registers[10], 0xBEEF);
  }
  #[test]
  fn test_mov_reg_reg_short() {
    let mut cpu = Cpu::new();
    cpu.registers[0] = 65535;
    cpu.load_program(&[Opcode::MoveRegRegShort as u8, 1, 0]);
    cpu.run();
    assert_eq!(cpu.registers[1], 65535);
    
    
    let mut cpu = Cpu::new();
    cpu.registers[0] = 65538;
    cpu.load_program(&[Opcode::MoveRegRegShort as u8, 1, 0]);
    cpu.run();
    assert_eq!(cpu.registers[1], 2);
  }
  #[test]
  fn test_mov_mem_mem_short() {
    let mut cpu = Cpu::new();
    cpu.memory.set_long(511, 0xDEADBEEF);
    cpu.load_program(&[Opcode::MoveMemMemShort as u8, 255, 2, 0 , 0, 255, 1, 0, 0]);
    cpu.run();
    
    assert_eq!(cpu.memory.long(767), 0xBEEF);
  }
  #[test]
  
  // Longs
  fn test_move_mem_reg_long() {
    let mut cpu = Cpu::new();
    cpu.memory.set_long(511, 0xDEADBEEF);
    cpu.load_program(&[Opcode::MoveMemRegLong as u8, 10, 255, 1, 0, 0]);
    cpu.run();
    
    assert_eq!(cpu.registers[10], 0xDEADBEEF);
  }
  #[test]
  fn test_mov_reg_reg_long() {
    let mut cpu = Cpu::new();
    cpu.registers[0] = 0xDEADBEEF;
    cpu.load_program(&[Opcode::MoveRegRegLong as u8, 1, 0]);
    cpu.run();
    assert_eq!(cpu.registers[1], 0xDEADBEEF);
  }
  #[test]
  fn test_mov_reg_mem_long() {
    let mut cpu = Cpu::new();
    cpu.registers[0] = 0xDEADBEEF;
    cpu.load_program(&[Opcode::MoveRegMemLong as u8, 255, 1, 0, 0, 0]);
    cpu.run();
    assert_eq!(cpu.ip, 7);
    assert_eq!(cpu.memory.long(511), 0xDEADBEEF);
  }
  #[test]
  fn test_mov_mem_mem_long() {
    let mut cpu = Cpu::new();
    cpu.memory.set_long(511, 0xDEADBEEF);
    cpu.load_program(&[Opcode::MoveMemMemLong as u8, 255, 2, 0 ,0, 255, 1, 0, 0]);
    cpu.run();
    
    assert_eq!(cpu.memory.long(767), 0xDEADBEEF);
  }
}
  
}
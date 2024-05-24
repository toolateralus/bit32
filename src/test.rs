
#[cfg(test)]
mod tests {
    use crate::{cpu::Cpu, opcodes::Opcode};

  #[test]
  fn test_mov_reg_red() {
    let mut cpu = Cpu::new();
    cpu.registers[0] = 10;
    cpu.load_program(&[3, 0, 1, 0]);
    cpu.run();
    assert_eq!(cpu.registers[0], cpu.registers[1]);
    assert_eq!(cpu.ip, 4);
  }
  
  #[test]
  fn test_mov_reg_mem() {
    let mut cpu = Cpu::new();   
    cpu.registers[0] = 10;
    cpu.load_program(&[4, 0, 100, 0, 0, 0, 0]);
    cpu.run();
    
    assert_eq!(cpu.memory.buffer[100], 10);
    assert_eq!(cpu.ip, 7)
  }
  
  #[test]
  fn test_mov_mem_mem() {
    let mut cpu = Cpu::new();
    
    cpu.memory.buffer[10] = 100;
    cpu.memory.buffer[100] = 0;
    
    cpu.load_program(&[5, 10, 0, 0, 0, 255, 1, 0, 0, 0]);
    
    cpu.cycle();
    
    assert_eq!(cpu.ip, 9);
    assert_eq!(cpu.memory.buffer[10], cpu.memory.buffer[511]);
  }
  
  #[test]
  fn test_mov_reg_reg_short() {
    let mut cpu = Cpu::new();
    cpu.registers[0] = 65535;
    cpu.load_program(&[Opcode::MoveRegRegShort as u8, 0, 1]);
    cpu.run();
    assert_eq!(cpu.registers[1], 65535);
  }
  #[test]
  fn test_mov_reg_reg_long() {
    let mut cpu = Cpu::new();
    cpu.registers[0] = 0xDEADBEEF;
    cpu.load_program(&[Opcode::MoveRegRegLong as u8, 0, 1]);
    cpu.run();
    assert_eq!(cpu.registers[1], 0xDEADBEEF);
  }
  
}
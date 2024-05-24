use crate::opcodes::Opcode;
use std::fmt::Debug;

const REGISTERS_COUNT: usize = 16;
// 100 MiB
const MEMORY_SIZE: usize = 100 * 1024 * 1024;

#[derive(Debug)]
pub struct Memory {
    pub buffer: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        return Self {
            buffer: vec![0; MEMORY_SIZE],
        };
    }
    pub fn byte(&mut self, addr: usize) -> u8 {
        if addr >= self.buffer.len() {
            panic!("memory access out of bounds {addr}");
        }
        let b = self.buffer[addr];
        return b;
    }
    pub fn short(&mut self, addr: usize) -> u16 {
        let low = self.byte(addr) as u16;
        let high = self.byte(addr + 1) as u16;
        return (high << 8) | low;
    }
    pub fn long(&mut self, addr: usize) -> u32 {
        let low = self.short(addr) as u32;
        let high = self.short(addr + 2) as u32;
        return (high << 16) | low;
    }
    pub fn set_long(&mut self, addr: usize, value: u32) {
        self.set_short(addr, value as u16);
        self.set_short(addr + 2, (value >> 16) as u16);
    }

    pub fn set_short(&mut self, addr: usize, value: u16) {
        self.set_byte(addr, value as u8);
        self.set_byte(addr + 1, (value >> 8) as u8);
    }
    pub fn set_byte(&mut self, addr: usize, value: u8) {
        if self.buffer.len() <= addr {
            panic!("memory access out of bounds {addr}");
        }
        self.buffer[addr] = value;
    }
}

pub struct Cpu {
    pub registers: [u32; REGISTERS_COUNT],
    pub memory: Memory,
    pub ip: usize,
    pub bp: usize,
    pub sp: usize,
    pub flags: u8,
}

impl Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cpu")
            .field("\nregisters", &self.registers)
            .field("\nip", &self.ip)
            .field("\nbp", &self.bp)
            .field("\nsp", &self.sp)
            .field("\nflags", &self.flags)
            .finish()
    }
}

impl Cpu {
    pub const HALT_FLAG: u8 = 0x01;
    pub fn new() -> Self {
        return Cpu {
            registers: [0; REGISTERS_COUNT],
            memory: Memory::new(),
            ip: 0,
            bp: 0,
            sp: 0,
            flags: 0,
        };
    }
    
    pub fn run(&mut self) {
        while (self.flags & Cpu::HALT_FLAG) != Cpu::HALT_FLAG {
            self.cycle();
        }
    }
}

impl Cpu {
    pub fn validate_register(&self, reg: usize) {
        if reg >= self.registers.len() {
            panic!("Invalid register {reg}");
        }
    }
    pub fn validate_registers(&self, regs: &[usize]) {
        for r in regs {
            self.validate_register(*r);
        }
    }
}
impl Cpu {
    pub fn mov(&mut self, opcode : Opcode) {
        match opcode {
            
            Opcode::MoveRegRegLong => {
                let dest = self.next_byte() as usize;
                let src = self.next_byte() as usize;
                self.validate_registers(&[dest, src]);
                self.registers[dest] = self.registers[src];
            }
            Opcode::MoveRegRegByte => {
                let dest = self.next_byte() as usize;
                let src = self.next_byte() as usize;
                self.validate_registers(&[dest, src]);
                self.registers[dest] = self.registers[src] & 0xFF; 
            }
            Opcode::MoveRegRegShort => {
                let dest = self.next_byte() as usize;
                let src = self.next_byte() as usize;
                self.validate_registers(&[dest, src]);
                self.registers[dest] = self.registers[src] & 0xFFFF;
            }
            Opcode::MoveRegMemShort => {
                let dest = self.next_long() as usize;
                let src = self.next_byte() as usize;
                self.memory.set_short(dest, self.registers[src] as u16);
            }
            Opcode::MoveMemRegShort => {
                let dest = self.next_byte() as usize;
                let src = self.next_long() as usize;
                self.registers[dest] = self.memory.short(src) as u32;
            }
            Opcode::MoveMemMemShort => {
                let dest = self.next_long() as usize;
                let src = self.next_long() as usize;
                let value = self.memory.short(src);
                self.memory.set_short(dest, value);
            }
            
            Opcode::MoveRegMemLong => {
                let dest = self.next_long() as usize;
                let src = self.next_byte() as usize;
                self.memory.set_long(dest, self.registers[src]);
            }
            Opcode::MoveMemRegLong => {
                let dest = self.next_byte() as usize;
                let src = self.next_long() as usize;
                self.registers[dest] = self.memory.long(src);
            }
            Opcode::MoveMemMemLong => {
                let dest = self.next_long() as usize;
                let src = self.next_long() as usize;
                let value = self.memory.long(src);
                self.memory.set_long(dest, value);
            }
            
            Opcode::MoveMemRegByte => {
                let dest = self.next_byte() as usize;
                let src = self.next_long() as usize;
                self.registers[dest] = self.memory.byte(src) as u32;
            }
            Opcode::MoveMemMemByte => {
                let dest = self.next_long() as usize;
                let src = self.next_long() as usize;
                let value = self.memory.byte(src);
                self.memory.set_byte(dest, value);
            }
            Opcode::MoveRegMemByte => {
                let dest = self.next_byte() as usize;
                let src = self.next_long() as usize;
                self.memory.set_long(dest, self.registers[src]);
            }
            _ => {
                panic!("Invalid move opcode");
            }
        }
    }
}

impl Cpu {
    pub fn next_byte(&mut self) -> u8 {
        let b = self.memory.byte(self.ip);
        self.ip += 1;
        return b;
    }
    pub fn next_short(&mut self) -> u16 {
        let low = self.next_byte() as u16;
        let high = self.next_byte() as u16;
        return (high << 8) | low;
    }
    pub fn next_long(&mut self) -> u32 {
        let low = self.next_short() as u32;
        let high = self.next_short() as u32;
        return (high << 16) | low;
    }
}

impl Cpu {
    pub fn load_program(&mut self, program: &[u8]) { 
        let iter = program.iter().cloned();
        self.memory.buffer.splice(0..program.len(), iter);
    }
    
    pub fn cycle(&mut self) {
        let instruction = self.next_byte();
        let opcode = Opcode::from(instruction);
        
        match opcode {
            Opcode::MoveRegRegLong |
            Opcode::MoveRegRegByte |
            Opcode::MoveRegRegShort |
            Opcode::MoveRegMemShort |
            Opcode::MoveMemRegShort |
            Opcode::MoveMemMemShort |
            Opcode::MoveRegMemLong |
            Opcode::MoveMemRegLong |
            Opcode::MoveMemMemLong |
            Opcode::MoveMemRegByte |
            Opcode::MoveMemMemByte |
            Opcode::MoveRegMemByte => {
                self.mov(opcode);
            }
            
            Opcode::Hlt => {
                self.flags |= Cpu::HALT_FLAG;
            }
            _ => {
                panic!("Invalid opcode : {instruction}");
            }
        }
    }
}

use crate::opcodes::Opcode;
use std::fmt::Debug;

const REGISTERS_COUNT: usize = 21;

#[allow(dead_code)]
pub const SP: usize = 20;
#[allow(dead_code)]
pub const IP: usize = 19;
#[allow(dead_code)]
pub const BP: usize = 18;
#[allow(dead_code)]
pub const FLAGS: usize = 17;

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
}
impl Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cpu")
            .field("\nregisters", &self.registers)
            .field("\nip", &self.ip())
            .field("\nbp", &self.bp())
            .field("\nsp", &self.sp())
            .field("\nflags", &self.flags())
            .finish()
    }
}
// General
impl Cpu {
    pub fn flags(&self) -> u8 {
        self.registers[FLAGS] as u8
    }
    pub fn sp(&self) -> usize {
        self.registers[SP] as usize
    }

    pub fn ip(&self) -> usize {
        self.registers[IP] as usize
    }

    pub fn bp(&self) -> usize {
        self.registers[BP] as usize
    }

    fn dec_sp(&mut self, value: u32) {
        self.registers[SP] -= value;
    }
    fn inc_sp(&mut self, value: u32) {
        self.registers[SP] += value;
    }
    fn inc_ip(&mut self, value: u32) {
        self.registers[IP] += value;
    }

    pub const HALT_FLAG: u8 = 0x01;
    pub fn new() -> Self {
        let mut cpu = Cpu {
            registers: [0; REGISTERS_COUNT],
            memory: Memory::new(),
        };
        
        // TODO: remove this after testing.
        // just a default stack so we don't have to set it up constantly.
        let bp = cpu.memory.buffer.len() - 1;
        cpu.registers[BP] = bp as u32;
        
        let sp = bp - 1000;
        cpu.registers[SP] = sp as u32;
        
        return cpu;
    }

    pub fn run(&mut self) {
        while (self.flags() & Cpu::HALT_FLAG) != Cpu::HALT_FLAG {
            self.cycle();
        }
    }
}


impl Cpu {
    pub fn and(&mut self, op: &Opcode) {
        
        match op  {
            Opcode::AndByteImm => {
                let lhs = (self.registers[0] & 0xFF) as u8;
                let rhs = self.next_byte();
                self.registers[0] = (lhs & rhs) as u32;
            }
            Opcode::AndByteMem => {
                let lhs = (self.registers[0] & 0xFF) as u8;
                let addr = self.next_long() as usize;
                let rhs = self.memory.byte(addr);
                self.registers[0] = (lhs & rhs) as u32;
            }
            Opcode::AndByteReg => {
                let lhs = (self.registers[0] & 0xFF) as u8;
                let index = self.next_byte() as usize;
                let rhs = (self.registers[index] & 0xFF) as u8;
                self.registers[0] = (lhs & rhs) as u32;
            }
            
            Opcode::AndShortImm => {
                let lhs = (self.registers[0] & 0xFFFF) as u16;
                let rhs = self.next_short();
                self.registers[0] = (lhs & rhs) as u32;
            }
            Opcode::AndShortMem => {
                let lhs = (self.registers[0] & 0xFFFF) as u16;
                let addr = self.next_long() as usize;
                let rhs = self.memory.short(addr);
                self.registers[0] = (lhs & rhs) as u32;
            }
            Opcode::AndShortReg => {
                let lhs = (self.registers[0] & 0xFFFF) as u16;
                let index = self.next_byte() as usize;
                let rhs = (self.registers[index] & 0xFFFF) as u16;
                self.registers[0] = (lhs & rhs) as u32;
            }
            
            Opcode::AndLongImm => {
                let lhs = self.registers[0];
                let rhs = self.next_long();
                self.registers[0] = lhs & rhs;
            }
            Opcode::AndLongMem => {
                let lhs = self.registers[0];
                let addr = self.next_long() as usize;
                let rhs = self.memory.long(addr);
                self.registers[0] = lhs & rhs;
            }
            Opcode::AndLongReg => {
                let lhs = self.registers[0];
                let index = self.next_byte() as usize;
                let rhs = self.registers[index];
                self.registers[0] = lhs & rhs;
            }
            _ => {
                panic!("invalid and instruction");
            }
        }
        
        
    }
    pub fn or(&mut self, op: &Opcode) {
        
    }
    pub fn xor(&mut self, op: &Opcode) {
        
    }
}

// Stack
impl Cpu {
    fn push(&mut self, op: &Opcode) {
        match op {
            Opcode::PushByteReg => {
                self.dec_sp(1);
                let index = self.next_byte() as usize;
                let value = (self.registers[index] & 0xFF) as u8;
                self.memory.set_byte(self.sp(), value);
            }
            Opcode::PushByteMem => {
                self.dec_sp(1);
                let addr = self.next_long() as usize;
                let value = self.memory.byte(addr);
                let sp = self.registers[SP] as usize;
                self.memory.set_byte(sp, value);
            }
            Opcode::PushByteImm => {
                self.dec_sp(1);
                let value = self.next_byte();
                self.memory.set_byte(self.sp(), value);
            }

            Opcode::PushShortImm => {
                self.dec_sp(2);
                let value = self.next_short();
                self.memory.set_short(self.sp(), value);
            }
            Opcode::PushShortMem => {
                self.dec_sp(2);
                let addr = self.next_long() as usize;
                let value = self.memory.short(addr);
                self.memory.set_short(self.sp(), value);
            }
            Opcode::PushShortReg => {
                self.dec_sp(2);
                let index = self.next_byte() as usize;
                let value = (self.registers[index] & 0xFFFF) as u16;
                self.memory.set_short(self.sp(), value);
            }

            Opcode::PushLongMem => {
                self.dec_sp(4);
                let addr = self.next_long() as usize;
                let value = self.memory.long(addr);
                self.memory.set_long(self.sp(), value);
            }
            Opcode::PushLongReg => {
                self.dec_sp(4);
                let index = self.next_byte() as usize;
                let value = self.registers[index];
                self.memory.set_long(self.sp(), value);
            }
            Opcode::PushLongImm => {
                self.dec_sp(4);
                let value = self.next_long();
                self.memory.set_long(self.sp(), value);
            }
            _ => {
                panic!("invalid push");
            }
        }
    }

    fn pop(&mut self, op: &Opcode) {
        match op {
            Opcode::PopByteReg => {
                let dest = self.next_byte() as usize;
                let value = self.memory.byte(self.sp());
                self.registers[dest] = value as u32;
                self.inc_sp(1);
            }
            Opcode::PopByteMem => {
                let addr = self.next_long() as usize;
                let value = self.memory.byte(self.sp());
                self.memory.set_byte(addr, value);
                self.inc_sp(1);
            }
            Opcode::PopShortReg => {
                let dest = self.next_byte() as usize;
                let value = self.memory.short(self.sp());
                self.registers[dest] = value as u32;
                self.inc_sp(2);
            }
            Opcode::PopShortMem => {
                let addr = self.next_long() as usize;
                let value = self.memory.short(self.sp());
                self.memory.set_short(addr, value);
                self.inc_sp(2);
            }
            Opcode::PopLongReg => {
                let dest = self.next_byte() as usize;
                let value = self.memory.long(self.sp());
                self.registers[dest] = value as u32;
                self.inc_sp(4);
            }
            Opcode::PopLongMem => {
                let addr = self.next_long() as usize;
                let value = self.memory.long(self.sp());
                self.memory.set_long(addr, value);
                self.inc_sp(4);
            }
            _ => {
                panic!("invalid pop");
            }
        }
    }
}

// Registers
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

// Arithmetic & Move
impl Cpu {
    pub fn arith_long(&mut self, opcode: &Opcode) {
        match opcode {
            Opcode::AddLong => {
                let lhs = self.registers[0];
                let rhs = self.next_long();
                let result = lhs.wrapping_add(rhs);
                self.registers[0] = result;
            }
            Opcode::SubLong => {
                let lhs = self.registers[0];
                let rhs = self.next_long();
                let result = lhs.wrapping_sub(rhs);
                self.registers[0] = result;
            }
            Opcode::DivLong => {
                let lhs = self.registers[0];
                let rhs = self.next_long();
                let quotient = lhs / rhs;
                let remainder = lhs % rhs;
                self.registers[0] = quotient;
                self.registers[1] = remainder;
            }
            Opcode::MulLong => {
                let lhs = self.registers[0];
                let rhs = self.next_long();
                let result = lhs.wrapping_mul(rhs);
                self.registers[0] = result;
            }
            _ => {
                panic!("invalid long arith instruction");
            }
        }
    }
    
    pub fn arith_short(&mut self, opcode: &Opcode) {
        match opcode {
            Opcode::AddShort => {
                let lhs = (self.registers[0] & 0xFFFF) as u16;
                let rhs = self.next_short();
                let result = lhs.wrapping_add(rhs);
                self.registers[0] = (result & 0xFFFF) as u32;
            }
            Opcode::SubShort => {
                let lhs = (self.registers[0] & 0xFFFF) as u16;
                let rhs = self.next_short();
                let result = lhs.wrapping_sub(rhs);
                self.registers[0] = (result & 0xFFFF) as u32;
            }
            Opcode::DivShort => {
                let lhs = (self.registers[0] & 0xFFFF) as u16;
                let rhs = self.next_short();
                let quotient = lhs / rhs;
                let remainder = lhs % rhs;
                self.registers[0] = quotient as u32;
                self.registers[1] = remainder as u32;
            }
            Opcode::MulShort => {
                let lhs = (self.registers[0] & 0xFFFF) as u16;
                let rhs = self.next_short();
                let result = lhs.wrapping_mul(rhs);
                self.registers[0] = (result & 0xFFFF) as u32;
            }
            _ => {
                panic!("invalid short arith instruction");
            }
        }
    }
    
    pub fn arith_byte(&mut self, opcode: &Opcode) {
        match opcode {
            Opcode::AddByte => {
                let lhs = (self.registers[0] & 0xFF) as u8;
                let rhs = self.next_byte();
                let result = lhs.wrapping_add(rhs);
                self.registers[0] = (result & 0xFF) as u32;
            }
            Opcode::SubByte => {
                let lhs = (self.registers[0] & 0xFF) as u8;
                let rhs = self.next_byte();
                let result = lhs.wrapping_sub(rhs);
                self.registers[0] = (result & 0xFF) as u32;
            }
            Opcode::DivByte => {
                let lhs = (self.registers[0] & 0xFF) as u8;
                let rhs = self.next_byte();
                let quotient = lhs / rhs;
                let remainder = lhs % rhs;
                self.registers[0] = quotient as u32;
                self.registers[1] = remainder as u32;
            }
            Opcode::MulByte => {
                let lhs = (self.registers[0] & 0xFF) as u8;
                let rhs = self.next_byte();
                let result = lhs.wrapping_mul(rhs);
                self.registers[0] = (result & 0xFF) as u32;
            }
            _ => {
                panic!("invalid byte arith instruction");
            }
        }
    }
    
    pub fn mov(&mut self, opcode: &Opcode) {
        match opcode {
            
            Opcode::MoveImmRegByte => {
                let index = self.next_byte() as usize;
                let value = self.next_byte();
                self.validate_register(index);
                self.registers[index] = value as u32;
            }
            Opcode::MoveImmRegLong => {
                let index = self.next_byte() as usize;
                let value = self.next_long();
                self.validate_register(index);
                self.registers[index] = value;
            }
            Opcode::MoveImmRegShort => {
                let index = self.next_byte() as usize;
                let value = self.next_short();
                self.validate_register(index);
                self.registers[index] = value as u32;
            }
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

// Memory utils
impl Cpu {
    pub fn next_byte(&mut self) -> u8 {
        let b = self.memory.byte(self.ip());
        self.inc_ip(1);
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

// General, Cycle, Load Program
impl Cpu {
    pub fn load_program(&mut self, program: &[u8]) {
        let iter = program.iter().cloned();
        self.memory.buffer.splice(0..program.len(), iter);
    }

    pub fn cycle(&mut self) {
        let instruction = self.next_byte();
        let opcode = Opcode::from(instruction);

        match opcode {
            Opcode::MoveRegRegLong
            | Opcode::MoveRegRegByte
            | Opcode::MoveRegRegShort
            | Opcode::MoveRegMemShort
            | Opcode::MoveMemRegShort
            | Opcode::MoveMemMemShort
            | Opcode::MoveRegMemLong
            | Opcode::MoveMemRegLong
            | Opcode::MoveMemMemLong
            | Opcode::MoveMemRegByte
            | Opcode::MoveMemMemByte
            | Opcode::MoveImmRegLong
            | Opcode::MoveImmRegShort 
            | Opcode::MoveImmRegByte
            | Opcode::MoveRegMemByte => {
                self.mov(&opcode);
            }
            
            Opcode::Call => {
                let addr = self.next_long();
                // push ret addr
                self.memory.set_long(self.sp(), self.ip() as u32);
                self.dec_sp(4);
                
                self.registers[IP] = addr;
            }
            Opcode::Return => {
                self.inc_sp(4);
                let addr = self.memory.long(self.sp());
                self.registers[IP] = addr;
            }
            
            Opcode::AndShortImm | Opcode::AndShortReg | Opcode::AndShortMem |
            Opcode::AndLongImm | Opcode::AndLongReg | Opcode::AndLongMem |
            Opcode::AndByteImm | Opcode::AndByteReg | Opcode::AndByteMem => {
                self.and(&opcode)
            }
            
            Opcode::AddByte | Opcode::DivByte | Opcode::MulByte | Opcode::SubByte => {
                self.arith_byte(&opcode);
            }
            Opcode::AddShort | Opcode::DivShort | Opcode::MulShort | Opcode::SubShort => {
                self.arith_short(&opcode);
            }
            Opcode::AddLong | Opcode::DivLong | Opcode::MulLong | Opcode::SubLong => {
                self.arith_long(&opcode);
            }

            Opcode::PushByteMem
            | Opcode::PushShortMem
            | Opcode::PushLongMem
            | Opcode::PushByteReg
            | Opcode::PushShortReg
            | Opcode::PushLongReg
            | Opcode::PushByteImm
            | Opcode::PushLongImm
            | Opcode::PushShortImm => {
                self.push(&opcode);
            }

            Opcode::PopLongMem
            | Opcode::PopShortMem
            | Opcode::PopByteMem
            | Opcode::PopByteReg
            | Opcode::PopShortReg
            | Opcode::PopLongReg => {
                self.pop(&opcode);
            }

            Opcode::Hlt => {
                self.registers[FLAGS] = (self.flags() | Cpu::HALT_FLAG) as u32;
            }
            _ => {
                panic!("Invalid opcode : {instruction}");
            }
        }
    }
}

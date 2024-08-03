use crate::functions::{self, log_opcode};
use crate::opcodes::Opcode;
use core::fmt;
use std::fmt::Debug;
use std::io::Read;
use std::str::Utf8Error;

const REGISTERS_COUNT: usize = 22;

#[allow(dead_code)]
pub const IDT: usize = 21;
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
    pub fn utf8(&mut self, addr: usize) -> Result<String, Utf8Error> {
        let mut bytes = Vec::new();
        let mut i = addr;
        loop {
            let b = self.byte(i);
            i += 1;
            if b == 0 {
                break;
            }
            bytes.push(b);
        }
        std::str::from_utf8(&bytes).map(|s| s.to_string())
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let registers = format!(
            "\x1b[0;36m{}\x1b[0m: \x1b[0;32m{:?}\x1b[0m",
            "registers", self.registers
        );
        let ip = format!(
            "\x1b[0;36m{}\x1b[0m: \x1b[0;33m{:?}\x1b[0m",
            "ip",
            self.ip()
        );
        let bp = format!(
            "\x1b[0;36m{}\x1b[0m: \x1b[0;34m{:?}\x1b[0m",
            "bp",
            self.bp()
        );
        let sp = format!(
            "\x1b[0;36m{}\x1b[0m: \x1b[0;35m{:?}\x1b[0m",
            "sp",
            self.sp()
        );
        let flags = format!(
            "\x1b[0;36m{}\x1b[0m: \x1b[0;36m{:?}\x1b[0m",
            "flags",
            self.flags()
        );

        write!(
            f,
            "Cpu {{\n{},\n{},\n{},\n{},\n{}\n}}",
            registers, ip, bp, sp, flags
        )
    }
}

// General, register helpers.
impl Cpu {
    pub const HALT_FLAG: u8 = 0x01;
    pub const INTERRUPT_FLAG: u8 = 0x02;
    
    pub fn new() -> Self {
        let mut cpu = Cpu {
            registers: [0; REGISTERS_COUNT],
            memory: Memory::new(),
        };

        // TODO: remove this after testing.
        // just a default stack so we don't have to set it up constantly.
        let bp = cpu.memory.buffer.len() - 20;
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
        if let Some(result) = self.registers[SP].checked_sub(value) {
            self.registers[SP] = result;
        } else {
            println!("{:?}", self);
            panic!("stack underflow.");
        }
    }
    fn inc_sp(&mut self, value: u32) {
        if let Some(result) = self.registers[SP].checked_add(value) {
            self.registers[SP] = result;
        } else {
            println!("{:?}", self);
            panic!("stack overflow.");
        }
    }
    fn inc_ip(&mut self, value: u32) {
        if let Some(result) = self.registers[IP].checked_add(value) {
            self.registers[IP] = result;
        } else {
            println!("{:?}", self);
            panic!("instruction pointer overflow.");
        }
    }
}

// Comparisons
impl Cpu {
    fn cmp(&mut self, opcode: &Opcode) {
        match opcode {
            Opcode::CompareByteImm => {
                let lhs = self.registers[0];
                let rhs = self.next_byte();
                self.registers[0] = if lhs == rhs as u32 { 1 } else { 0 };
            }
            Opcode::CompareShortImm => {
                let lhs = self.registers[0];
                let rhs = self.next_short();
                self.registers[0] = if lhs == rhs as u32 { 1 } else { 0 };
            }
            Opcode::CompareLongImm => {
                let lhs = self.registers[0];
                let rhs = self.next_long();
                self.registers[0] = if lhs == rhs { 1 } else { 0 };
            }
            Opcode::CompareReg => {
                let lhs = self.registers[0];
                let index = self.next_byte() as usize;
                let rhs = self.registers[index];
                self.registers[0] = if lhs == rhs { 1 } else { 0 };
            }
            _ => panic!("invalid compare"),
        }
    }
    pub fn and(&mut self, op: &Opcode) {
        match op {
            Opcode::AndByteImm => {
                let lhs = (self.registers[0] & 0xFF) as u8;
                let rhs = self.next_byte();
                self.registers[0] = (lhs & rhs) as u32;
            }
            Opcode::AndShortImm => {
                let lhs = (self.registers[0] & 0xFFFF) as u16;
                let rhs = self.next_short();
                self.registers[0] = (lhs & rhs) as u32;
            }
            Opcode::AndLongImm => {
                let lhs = self.registers[0];
                let rhs = self.next_long();
                self.registers[0] = lhs & rhs;
            }
            Opcode::AndByteReg => {
                let lhs = (self.registers[0] & 0xFF) as u8;
                let index = self.next_byte() as usize;
                let rhs = (self.registers[index] & 0xFF) as u8;
                self.registers[0] = (lhs & rhs) as u32;
            }
            Opcode::AndShortReg => {
                let lhs = (self.registers[0] & 0xFFFF) as u16;
                let index = self.next_byte() as usize;
                let rhs = (self.registers[index] & 0xFFFF) as u16;
                self.registers[0] = (lhs & rhs) as u32;
            }
            Opcode::AndLongReg => {
                let lhs = self.registers[0];
                let index = self.next_byte() as usize;
                let rhs = self.registers[index];
                self.registers[0] = lhs & rhs;
            }
            Opcode::AndByteMem => {
                let lhs = (self.registers[0] & 0xFF) as u8;
                let addr = self.next_long() as usize;
                let rhs = self.memory.byte(addr);
                self.registers[0] = (lhs & rhs) as u32;
            }
            Opcode::AndShortMem => {
                let lhs = (self.registers[0] & 0xFFFF) as u16;
                let addr = self.next_long() as usize;
                let rhs = self.memory.short(addr);
                self.registers[0] = (lhs & rhs) as u32;
            }
            Opcode::AndLongMem => {
                let lhs = self.registers[0];
                let addr = self.next_long() as usize;
                let rhs = self.memory.long(addr);
                self.registers[0] = lhs & rhs;
            }
            _ => {
                panic!("invalid and instruction");
            }
        }
    }
}

// Jumps
impl Cpu {
    fn jmp(&mut self) {
        let addr = self.next_long();
        self.registers[IP] = self.registers[IP].wrapping_add(addr);
    }
    fn jg(&mut self) {
        let lhs = self.registers[0];
        let rhs = self.registers[1];
        let addr = self.next_long();
        if lhs > rhs {
            self.registers[IP] = self.registers[IP].wrapping_add(addr);
        }
    }
    fn jl(&mut self) {
        let lhs = self.registers[0];
        let rhs = self.registers[1];
        let addr = self.next_long();
        if lhs < rhs {
            self.registers[IP] = self.registers[IP].wrapping_add(addr);
        }
    }
    fn je(&mut self) {
        let lhs = self.registers[0];
        let rhs = self.registers[1];
        let addr = self.next_long();
        if lhs == rhs {
            self.registers[IP] = self.registers[IP].wrapping_add(addr);
        }
    }
    fn jne(&mut self) {
        let lhs = self.registers[0];
        let rhs = self.registers[1];
        let addr = self.next_long();
        if lhs != rhs {
            self.registers[IP] = self.registers[IP].wrapping_add(addr);
        }
    }
    fn jmp_reg(&mut self) {
        let index = self.next_byte() as usize;
        let addr = self.registers[index];
        self.registers[IP] = self.registers[IP].wrapping_add(addr);
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

// Arithmetic 
impl Cpu {
    pub fn arith_long(&mut self, opcode: &Opcode) {
        let lhs = self.registers[0];
        match opcode {
            Opcode::AddLongImm => {
                let rhs = self.next_long();
                let result = lhs.wrapping_add(rhs);
                self.registers[0] = result;
            }
            Opcode::AddLongReg => {
                let index = self.next_byte() as usize;
                let rhs = self.registers[index];
                let result = lhs.wrapping_add(rhs);
                self.registers[0] = result;
            }
            Opcode::AddLongMem => {
                let addr = self.next_long() as usize;
                let rhs = self.memory.long(addr);
                let result = lhs.wrapping_add(rhs);
                self.registers[0] = result;
            }
            Opcode::SubLongImm => {
                let rhs = self.next_long();
                let result = lhs.wrapping_sub(rhs);
                self.registers[0] = result;
            }
            Opcode::SubLongReg => {
                let index = self.next_byte() as usize;
                let rhs = self.registers[index];
                let result = lhs.wrapping_sub(rhs);
                self.registers[0] = result;
            }
            Opcode::SubLongMem => {
                let addr = self.next_long() as usize;
                let rhs = self.memory.long(addr);
                let result = lhs.wrapping_sub(rhs);
                self.registers[0] = result;
            }
            Opcode::DivLongImm => {
                let rhs = self.next_long();
                let quotient = lhs / rhs;
                let remainder = lhs % rhs;
                self.registers[0] = quotient;
                self.registers[1] = remainder;
            }
            Opcode::DivLongReg => {
                let index = self.next_byte() as usize;
                let rhs = self.registers[index];
                let quotient = lhs / rhs;
                let remainder = lhs % rhs;
                self.registers[0] = quotient;
                self.registers[1] = remainder;
            }
            Opcode::DivLongMem => {
                let addr = self.next_long() as usize;
                let rhs = self.memory.long(addr);
                let quotient = lhs / rhs;
                let remainder = lhs % rhs;
                self.registers[0] = quotient;
                self.registers[1] = remainder;
            }
            Opcode::MulLongImm => {
                let rhs = self.next_long();
                let result = lhs.wrapping_mul(rhs);
                self.registers[0] = result;
            }
            Opcode::MulLongReg => {
                let index = self.next_byte() as usize;
                let rhs = self.registers[index];
                let result = lhs.wrapping_mul(rhs);
                self.registers[0] = result;
            }
            Opcode::MulLongMem => {
                let addr = self.next_long() as usize;
                let rhs = self.memory.long(addr);
                let result = lhs.wrapping_mul(rhs);
                self.registers[0] = result;
            }
            _ => {
                panic!("invalid long arith instruction");
            }
        }
    }
    pub fn arith_short(&mut self, opcode: &Opcode) {
        let lhs = (self.registers[0] & 0xFFFF) as u16;
        match opcode {
            Opcode::AddShortImm => {
                let rhs = self.next_short();
                let result = lhs.wrapping_add(rhs);
                self.registers[0] = (result & 0xFFFF) as u32;
            }
            Opcode::AddShortReg => {
                let index = self.next_byte() as usize;
                let rhs = (self.registers[index] & 0xFFFF) as u16;
                let result = lhs.wrapping_add(rhs);
                self.registers[0] = (result & 0xFFFF) as u32;
            }
            Opcode::AddShortMem => {
                let addr = self.next_long() as usize;
                let rhs = self.memory.short(addr);
                let result = lhs.wrapping_add(rhs);
                self.registers[0] = (result & 0xFFFF) as u32;
            }
            Opcode::SubShortImm => {
                let rhs = self.next_short();
                let result = lhs.wrapping_sub(rhs);
                self.registers[0] = (result & 0xFFFF) as u32;
            }
            Opcode::SubShortReg => {
                let index = self.next_byte() as usize;
                let rhs = (self.registers[index] & 0xFFFF) as u16;
                let result = lhs.wrapping_sub(rhs);
                self.registers[0] = (result & 0xFFFF) as u32;
            }
            Opcode::SubShortMem => {
                let addr = self.next_long() as usize;
                let rhs = self.memory.short(addr);
                let result = lhs.wrapping_sub(rhs);
                self.registers[0] = (result & 0xFFFF) as u32;
            }
            Opcode::DivShortImm => {
                let rhs = self.next_short();
                let quotient = lhs / rhs;
                let remainder = lhs % rhs;
                self.registers[0] = quotient as u32;
                self.registers[1] = remainder as u32;
            }
            Opcode::DivShortReg => {
                let index = self.next_byte() as usize;
                let rhs = (self.registers[index] & 0xFFFF) as u16;
                let quotient = lhs / rhs;
                let remainder = lhs % rhs;
                self.registers[0] = quotient as u32;
                self.registers[1] = remainder as u32;
            }
            Opcode::DivShortMem => {
                let addr = self.next_long() as usize;
                let rhs = self.memory.short(addr);
                let quotient = lhs / rhs;
                let remainder = lhs % rhs;
                self.registers[0] = quotient as u32;
                self.registers[1] = remainder as u32;
            }
            Opcode::MulShortImm => {
                let rhs = self.next_short();
                let result = lhs.wrapping_mul(rhs);
                self.registers[0] = (result & 0xFFFF) as u32;
            }
            Opcode::MulShortReg => {
                let index = self.next_byte() as usize;
                let rhs = (self.registers[index] & 0xFFFF) as u16;
                let result = lhs.wrapping_mul(rhs);
                self.registers[0] = (result & 0xFFFF) as u32;
            }
            Opcode::MulShortMem => {
                let addr = self.next_long() as usize;
                let rhs = self.memory.short(addr);
                let result = lhs.wrapping_mul(rhs);
                self.registers[0] = (result & 0xFFFF) as u32;
            }
            _ => {
                panic!("invalid short arith instruction");
            }
        }
    }
    pub fn arith_byte(&mut self, opcode: &Opcode) {
        let lhs = (self.registers[0] & 0xFF) as u8;
        match opcode {
            Opcode::AddByteImm => {
                let rhs = self.next_byte();
                let result = lhs.wrapping_add(rhs);
                self.registers[0] = (result & 0xFF) as u32;
            }
            Opcode::AddByteReg => {
                let index = self.next_byte() as usize;
                let rhs = (self.registers[index] & 0xFF) as u8;
                let result = lhs.wrapping_add(rhs);
                self.registers[0] = (result & 0xFF) as u32;
            }
            Opcode::AddByteMem => {
                let addr = self.next_long() as usize;
                let rhs = self.memory.byte(addr);
                let result = lhs.wrapping_add(rhs);
                self.registers[0] = (result & 0xFF) as u32;
            }
            Opcode::SubByteImm => {
                let rhs = self.next_byte();
                let result = lhs.wrapping_sub(rhs);
                self.registers[0] = (result & 0xFF) as u32;
            }
            Opcode::SubByteReg => {
                let index = self.next_byte() as usize;
                let rhs = (self.registers[index] & 0xFF) as u8;
                let result = lhs.wrapping_sub(rhs);
                self.registers[0] = (result & 0xFF) as u32;
            }
            Opcode::SubByteMem => {
                let addr = self.next_long() as usize;
                let rhs = self.memory.byte(addr);
                let result = lhs.wrapping_sub(rhs);
                self.registers[0] = (result & 0xFF) as u32;
            }
            Opcode::DivByteImm => {
                let rhs = self.next_byte();
                let quotient = lhs / rhs;
                let remainder = lhs % rhs;
                self.registers[0] = quotient as u32;
                self.registers[1] = remainder as u32;
            }
            Opcode::DivByteReg => {
                let index = self.next_byte() as usize;
                let rhs = (self.registers[index] & 0xFF) as u8;
                let quotient = lhs / rhs;
                let remainder = lhs % rhs;
                self.registers[0] = quotient as u32;
                self.registers[1] = remainder as u32;
            }
            Opcode::DivByteMem => {
                let addr = self.next_long() as usize;
                let rhs = self.memory.byte(addr);
                let quotient = lhs / rhs;
                let remainder = lhs % rhs;
                self.registers[0] = quotient as u32;
                self.registers[1] = remainder as u32;
            }
            Opcode::MulByteImm => {
                let rhs = self.next_byte();
                let result = lhs.wrapping_mul(rhs);
                self.registers[0] = (result & 0xFF) as u32;
            }
            Opcode::MulByteReg => {
                let index = self.next_byte() as usize;
                let rhs = (self.registers[index] & 0xFF) as u8;
                let result = lhs.wrapping_mul(rhs);
                self.registers[0] = (result & 0xFF) as u32;
            }
            Opcode::MulByteMem => {
                let addr = self.next_long() as usize;
                let rhs = self.memory.byte(addr);
                let result = lhs.wrapping_mul(rhs);
                self.registers[0] = (result & 0xFF) as u32;
            }
            _ => {
                panic!("invalid byte arith instruction");
            }
        }
    }
}

// Move
impl Cpu {
    pub fn mov_to_reg(&mut self, opcode: &Opcode) {
        match opcode {
            //from immediate
            Opcode::MoveImmRegByte => {
                let dst_reg = self.next_byte() as usize;
                let src_val = self.next_byte();
                self.validate_register(dst_reg);
                self.registers[dst_reg] = src_val as u32;
            }
            Opcode::MoveImmRegShort => {
                let dst_reg = self.next_byte() as usize;
                let src_val = self.next_short();
                self.validate_register(dst_reg);
                self.registers[dst_reg] = src_val as u32;
            }
            Opcode::MoveImmRegLong => {
                let dst_reg = self.next_byte() as usize;
                let src_val = self.next_long();
                self.validate_register(dst_reg);
                self.registers[dst_reg] = src_val;
            }

            // from register
            Opcode::MoveRegRegByte => {
                let dst_reg = self.next_byte() as usize;
                let src_reg = self.next_byte() as usize;
                self.validate_registers(&[dst_reg, src_reg]);
                self.registers[dst_reg] = self.registers[src_reg] & 0xFF;
            }
            Opcode::MoveRegRegShort => {
                let dst_reg = self.next_byte() as usize;
                let src_reg = self.next_byte() as usize;
                self.validate_registers(&[dst_reg, src_reg]);
                self.registers[dst_reg] = self.registers[src_reg] & 0xFFFF;
            }
            Opcode::MoveRegRegLong => {
                let dst_reg = self.next_byte() as usize;
                let src_reg = self.next_byte() as usize;
                self.validate_registers(&[dst_reg, src_reg]);
                self.registers[dst_reg] = self.registers[src_reg];
            }

            // from relative memory
            Opcode::MoveMemRegByte => {
                let dst_reg = self.next_byte() as usize;
                let src_adr = self.next_long() as usize;
                let src_val = self.memory.byte(src_adr + self.ip()) as u32;
                self.registers[dst_reg] = src_val;
            }
            Opcode::MoveMemRegShort => {
                let dst_reg = self.next_byte() as usize;
                let src_adr = self.next_long() as usize;
                let src_val = self.memory.short(src_adr + self.ip()) as u32;
                self.registers[dst_reg] = src_val;
            }
            Opcode::MoveMemRegLong => {
                let dst_reg = self.next_byte() as usize;
                let src_adr = self.next_long() as usize;
                let src_val = self.memory.long(src_adr + self.ip());
                self.registers[dst_reg] = src_val;
            }

            // from absolute memory
            Opcode::MoveAbsRegByte => {
                let dst_reg = self.next_byte() as usize;
                let src_adr = self.next_long() as usize;
                self.registers[dst_reg] = self.memory.byte(src_adr) as u32;
            }
            Opcode::MoveAbsRegShort => {
                let dst_reg = self.next_byte() as usize;
                let src_adr = self.next_long() as usize;
                self.registers[dst_reg] = self.memory.short(src_adr) as u32;
            }
            Opcode::MoveAbsRegLong => {
                let dst_reg = self.next_byte() as usize;
                let src_adr = self.next_long() as usize;
                self.registers[dst_reg] = self.memory.long(src_adr);
            }

            // from indirect memory
            Opcode::MoveIndirectRegByte => {
                let dst_reg = self.next_byte() as usize;
                let src_reg = self.next_byte() as usize;
                let src_adr = self.registers[src_reg] as usize;
                self.registers[dst_reg] = self.memory.byte(src_adr) as u32;
            }
            Opcode::MoveIndirectRegShort => {
                let dst_reg = self.next_byte() as usize;
                let src_reg = self.next_byte() as usize;
                let src_adr = self.registers[src_reg] as usize;
                self.registers[dst_reg] = self.memory.short(src_adr) as u32;
            }
            Opcode::MoveIndirectRegLong => {
                let dst_reg = self.next_byte() as usize;
                let src_reg = self.next_byte() as usize;
                let src_adr = self.registers[src_reg] as usize;
                self.registers[dst_reg] = self.memory.long(src_adr);
            }

            _ => {
                panic!("Invalid move opcode");
            }
        }
    }
    pub fn mov_to_rel(&mut self, opcode: &Opcode) {
        match opcode {
            // from immediate
            Opcode::MoveImmMemByte => {
                let dst_adr = self.next_long() as usize;
                let src_val = self.next_byte();
                self.memory.set_byte(dst_adr + self.ip(), src_val);
            }
            Opcode::MoveImmMemShort => {
                let dst_adr = self.next_long() as usize;
                let src_val = self.next_short();
                self.memory.set_short(dst_adr + self.ip(), src_val);
            }
            Opcode::MoveImmMemLong => {
                let dst_adr = self.next_long() as usize;
                let src_val = self.next_long();
                self.memory.set_long(dst_adr + self.ip(), src_val);
            }

            // from register 
            Opcode::MoveRegMemByte => {
                let dst_adr = self.next_long() as usize;
                let src_reg = self.next_byte() as usize;
                self.memory.set_byte(dst_adr + self.ip(), self.registers[src_reg] as u8);
            }
            Opcode::MoveRegMemShort => {
                let dst_adr = self.next_long() as usize;
                let src_reg = self.next_byte() as usize;
                self.memory.set_short(dst_adr + self.ip(), self.registers[src_reg] as u16);
            }
            Opcode::MoveRegMemLong => {
                let dst_adr = self.next_long() as usize;
                let src_reg = self.next_byte() as usize;
                self.memory.set_long(dst_adr + self.ip(), self.registers[src_reg]);
            }

            // from relative memory
            Opcode::MoveMemMemByte => {
                let dst_adr = self.next_long() as usize;
                let src_adr = self.next_long() as usize;
                let src_val = self.memory.byte(src_adr + self.ip());
                self.memory.set_byte(dst_adr + self.ip(), src_val);
            }
            Opcode::MoveMemMemShort => {
                let dst_adr = self.next_long() as usize;
                let src_adr = self.next_long() as usize;
                let src_val = self.memory.short(src_adr + self.ip());
                self.memory.set_short(dst_adr + self.ip(), src_val);
            }
            Opcode::MoveMemMemLong => {
                let dst_adr = self.next_long() as usize;
                let src_adr = self.next_long() as usize;
                let src_val = self.memory.long(src_adr + self.ip());
                self.memory.set_long(dst_adr + self.ip(), src_val);
            }

            // from absolute memory
            Opcode::MoveAbsMemByte => {
                let dst_adr = self.next_long() as usize;
                let src_adr = self.next_long() as usize;
                let src_val = self.memory.byte(src_adr);
                self.memory.set_byte(dst_adr + self.ip(), src_val);
            }
            Opcode::MoveAbsMemShort => {
                let dst_adr = self.next_long() as usize;
                let src_adr = self.next_long() as usize;
                let src_val = self.memory.short(src_adr);
                self.memory.set_short(dst_adr + self.ip(), src_val);
            }
            Opcode::MoveAbsMemLong => {
                let dst_adr = self.next_long() as usize;
                let src_adr = self.next_long() as usize;
                let src_val = self.memory.long(src_adr);
                self.memory.set_long(dst_adr + self.ip(), src_val);
            }

            // from Indirect memory
            Opcode::MoveIndirectMemByte => {
                let dst_adr = self.next_long() as usize;
                let src_reg = self.next_byte() as usize;
                let src_val = self.memory.byte(self.registers[src_reg] as usize);
                self.memory.set_byte(dst_adr + self.ip(), src_val);
            }
            Opcode::MoveIndirectMemShort => {
                let dst_adr = self.next_long() as usize;
                let src_reg = self.next_byte() as usize;
                let src_val = self.memory.short(self.registers[src_reg] as usize);
                self.memory.set_short(dst_adr + self.ip(), src_val);
            }
            Opcode::MoveIndirectMemLong => {
                let dst_adr = self.next_long() as usize;
                let src_reg = self.next_byte() as usize;
                let src_val = self.memory.long(self.registers[src_reg] as usize);
                self.memory.set_long(dst_adr + self.ip(), src_val);
            }

            _ => {
                panic!("Invalid move opcode");
            }
        }
    }
    pub fn mov_to_abs(&mut self, opcode: &Opcode) {
        match opcode {
            // from immediate
            Opcode::MoveImmAbsByte => {
                let dst_adr = self.next_long() as usize;
                let src_val = self.next_byte();
                self.memory.set_byte(dst_adr, src_val);
            }
            Opcode::MoveImmAbsShort => {
                let dst_adr = self.next_long() as usize;
                let src_val = self.next_short();
                self.memory.set_short(dst_adr, src_val);
            }
            Opcode::MoveImmAbsLong => {
                let dst_adr = self.next_long() as usize;
                let src_val = self.next_long();
                self.memory.set_long(dst_adr, src_val);
            }

            // from register 
            Opcode::MoveRegAbsByte => {
                let dst_adr = self.next_long() as usize;
                let src_reg = self.next_byte() as usize;
                self.memory.set_byte(dst_adr, self.registers[src_reg] as u8);
            }
            Opcode::MoveRegAbsShort => {
                let dst_adr = self.next_long() as usize;
                let src_reg = self.next_byte() as usize;
                self.memory.set_short(dst_adr, self.registers[src_reg] as u16);
            }
            Opcode::MoveRegAbsLong => {
                let dst_adr = self.next_long() as usize;
                let src_reg = self.next_byte() as usize;
                self.memory.set_long(dst_adr, self.registers[src_reg]);
            }

            // from relative memory
            Opcode::MoveMemAbsByte => {
                let dst_adr = self.next_long() as usize;
                let src_adr = self.next_long() as usize;
                let src_val = self.memory.byte(src_adr + self.ip());
                self.memory.set_byte(dst_adr, src_val);
            }
            Opcode::MoveMemAbsShort => {
                let dst_adr = self.next_long() as usize;
                let src_adr = self.next_long() as usize;
                let src_val = self.memory.short(src_adr + self.ip());
                self.memory.set_short(dst_adr, src_val);
            }
            Opcode::MoveMemAbsLong => {
                let dst_adr = self.next_long() as usize;
                let src_adr = self.next_long() as usize;
                let src_val = self.memory.long(src_adr + self.ip());
                self.memory.set_long(dst_adr, src_val);
            }

            // from absolute memory
            Opcode::MoveAbsAbsByte => {
                let dst_adr = self.next_long() as usize;
                let src_adr = self.next_long() as usize;
                let src_val = self.memory.byte(src_adr);
                self.memory.set_byte(dst_adr, src_val);
            }
            Opcode::MoveAbsAbsShort => {
                let dst_adr = self.next_long() as usize;
                let src_adr = self.next_long() as usize;
                let src_val = self.memory.short(src_adr);
                self.memory.set_short(dst_adr, src_val);
            }
            Opcode::MoveAbsAbsLong => {
                let dst_adr = self.next_long() as usize;
                let src_adr = self.next_long() as usize;
                let src_val = self.memory.long(src_adr);
                self.memory.set_long(dst_adr, src_val);
            }

            // from Indirect memory
            Opcode::MoveIndirectAbsByte => {
                let dst_adr = self.next_long() as usize;
                let src_reg = self.next_byte() as usize;
                let src_val = self.memory.byte(self.registers[src_reg] as usize);
                self.memory.set_byte(dst_adr, src_val);
            }
            Opcode::MoveIndirectAbsShort => {
                let dst_adr = self.next_long() as usize;
                let src_reg = self.next_byte() as usize;
                let src_val = self.memory.short(self.registers[src_reg] as usize);
                self.memory.set_short(dst_adr, src_val);
            }
            Opcode::MoveIndirectAbsLong => {
                let dst_adr = self.next_long() as usize;
                let src_reg = self.next_byte() as usize;
                let src_val = self.memory.long(self.registers[src_reg] as usize);
                self.memory.set_long(dst_adr, src_val);
            }

            _ => {
                panic!("Invalid move opcode");
            }
        }
    }
    pub fn mov_to_ind(&mut self, opcode: &Opcode) {
        match opcode {
            // from immediate
            Opcode::MoveImmIndirectByte => {
                let dst_reg = self.next_byte() as usize;
                let src_val = self.next_byte();
                let dst_adr = self.registers[dst_reg] as usize;
                self.memory.set_byte(dst_adr, src_val);
            }
            Opcode::MoveImmIndirectShort => {
                let dst_reg = self.next_byte() as usize;
                let src_val = self.next_short();
                let dst_adr = self.registers[dst_reg] as usize;
                self.memory.set_short(dst_adr, src_val);
            }
            Opcode::MoveImmIndirectLong => {
                let dst_reg = self.next_byte() as usize;
                let src_val = self.next_long();
                let dst_adr = self.registers[dst_reg] as usize;
                self.memory.set_long(dst_adr, src_val);
            }

            // from register
            Opcode::MoveRegIndirectByte => {
                let dst_reg = self.next_byte() as usize;
                let src_reg = self.next_byte() as usize;
                let dst_adr = self.registers[dst_reg] as usize;
                self.memory.set_byte(dst_adr, self.registers[src_reg] as u8);
            }
            Opcode::MoveRegIndirectShort => {
                let dst_reg = self.next_byte() as usize;
                let src_reg = self.next_byte() as usize;
                let dst_adr = self.registers[dst_reg] as usize;
                self.memory.set_short(dst_adr, self.registers[src_reg] as u16);
            }
            Opcode::MoveRegIndirectLong => {
                let dst_reg = self.next_byte() as usize;
                let src_reg = self.next_byte() as usize;
                let dst_adr = self.registers[dst_reg] as usize;
                self.memory.set_long(dst_adr, self.registers[src_reg]);
            }

            // from relative memory
            Opcode::MoveMemIndirectByte => {
                let dst_reg = self.next_byte() as usize;
                let src_adr = self.next_long() as usize;
                let dst_adr = self.registers[dst_reg] as usize;
                let src_val = self.memory.byte(src_adr + self.ip());
                self.memory.set_byte(dst_adr, src_val);
            }
            Opcode::MoveMemIndirectShort => {
                let dst_reg = self.next_byte() as usize;
                let src_adr = self.next_long() as usize;
                let dst_adr = self.registers[dst_reg] as usize;
                let src_val = self.memory.short(src_adr + self.ip());
                self.memory.set_short(dst_adr, src_val);
            }
            Opcode::MoveMemIndirectLong => {
                let dst_reg = self.next_byte() as usize;
                let src_adr = self.next_long() as usize;
                let dst_adr = self.registers[dst_reg] as usize;
                let src_val = self.memory.long(src_adr + self.ip());
                self.memory.set_long(dst_adr, src_val);
            }

            // from absolute memory
            Opcode::MoveAbsIndirectByte => {
                let dst_reg = self.next_byte() as usize;
                let src_adr = self.next_long() as usize;
                let dst_adr = self.registers[dst_reg] as usize;
                let src_val = self.memory.byte(src_adr);
                self.memory.set_byte(dst_adr, src_val);
            }
            Opcode::MoveAbsIndirectShort => {
                let dst_reg = self.next_byte() as usize;
                let src_adr = self.next_long() as usize;
                let dst_adr = self.registers[dst_reg] as usize;
                let src_val = self.memory.short(src_adr);
                self.memory.set_short(dst_adr, src_val);
            }
            Opcode::MoveAbsIndirectLong => {
                let dst_reg = self.next_byte() as usize;
                let src_adr = self.next_long() as usize;
                let dst_adr = self.registers[dst_reg] as usize;
                let src_val = self.memory.long(src_adr);
                self.memory.set_long(dst_adr, src_val);
            }

            // from indirect memory
            Opcode::MoveIndirectIndirectByte => {
                let dst_reg = self.next_byte() as usize;
                let src_reg = self.next_byte() as usize;
                let dst_adr = self.registers[dst_reg] as usize;
                let src_adr = self.registers[src_reg] as usize;
                let src_val = self.memory.byte(src_adr);
                self.memory.set_byte(dst_adr, src_val);
            }
            Opcode::MoveIndirectIndirectShort => {
                let dst_reg = self.next_byte() as usize;
                let src_reg = self.next_byte() as usize;
                let dst_adr = self.registers[dst_reg] as usize;
                let src_adr = self.registers[src_reg] as usize;
                let src_val = self.memory.short(src_adr);
                self.memory.set_short(dst_adr, src_val);
            }
            Opcode::MoveIndirectIndirectLong => {
                let dst_reg = self.next_byte() as usize;
                let src_reg = self.next_byte() as usize;
                let dst_adr = self.registers[dst_reg] as usize;
                let src_adr = self.registers[src_reg] as usize;
                let src_val = self.memory.long(src_adr);
                self.memory.set_long(dst_adr, src_val);
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
    pub fn next_utf8(&mut self) -> String {
        let string = self.memory.utf8(self.ip()).unwrap();
        self.inc_ip(string.len() as u32);
        return string;
    }
}

// General, Cycle, Load Program
impl Cpu {
    pub fn reg_index_to_str(index: &usize) -> &str {
        match index {
            0 => "rax",
            1 => "rbx",
            2 => "rcx",
            3 => "rdx",
            4 => "rex",
            5 => "rfx",
            6 => "rgx",
            7 => "rhx",
            8 => "rzx",
            9 => "rwx",
            
            10 => "r9",
            11 => "r10",
            12 => "r11",
            13 => "r12",
            14 => "r13",
            15 => "r14",
            16 => "r15",
            
            17 => "flags",
            18 => "bp",
            19 => "ip",
            20 => "sp",
            21 => "idt",
            _ => {
                panic!("invalid register {index}");
            }
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        let iter = program.iter().cloned();
        self.memory.buffer.splice(0..program.len(), iter);
    }

    pub fn load_program_from_file(&mut self, file_path: &str) -> std::io::Result<()> {
        let mut file = std::fs::File::open(file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        self.load_program(&buffer);

        let prog = &self.memory.buffer[..buffer.len()];

        println!("loaded program: {:?}", prog);

        Ok(())
    }

    pub fn cycle(&mut self) {
        let instruction = self.next_byte();
        let opcode = Opcode::from(instruction);
        log_opcode(&opcode);
        match opcode {
            Opcode::Interrupt => {
                if self.registers[FLAGS] & Cpu::INTERRUPT_FLAG as u32 != 0 {
                    return;                    
                }
                
                let irq = self.next_byte() as u32;
                
                // get the base of the idt
                let idt_base = self.registers[IDT] as u32;
                
                // idt entries are exactly 4 bytes long
                let isr_addr = idt_base + (irq * 4);
                
                // push return address
                self.dec_sp(4);
                self.memory.set_long(self.sp(), self.ip() as u32);
                self.registers[FLAGS] |= Cpu::INTERRUPT_FLAG as u32;
                self.registers[IP] = self.registers[IP].wrapping_add(isr_addr);
                
            }
            Opcode::InterruptReturn => {
                let ret_addr = self.memory.long(self.sp());
                self.inc_sp(4);
                self.registers[FLAGS] &= !(Cpu::INTERRUPT_FLAG as u32);
                self.registers[IP] = self.registers[IP].wrapping_add(ret_addr);
            }
            Opcode::Call => {
                // push return address
                self.dec_sp(4);
                let addr = self.next_long();
                self.memory.set_long(self.sp(), self.ip() as u32);
                
                self.registers[IP] = addr;
            }
            
            Opcode::Return => {
                // pop return address
                let addr = self.memory.long(self.sp());
                self.inc_sp(4);
                self.registers[IP] = addr;
            }

            Opcode::JumpEqual => {
                self.je();
            }
            Opcode::JumpNotEqual => {
                self.jne();
            }
            Opcode::JumpReg => {
                self.jmp_reg();
            }
            Opcode::JumpImm => {
                self.jmp();
            }
            
            Opcode::JumpLess => {
                self.jl();
            }
            Opcode::JumpGreater =>{
                self.jg();
            }
            
            Opcode::CompareReg
            | Opcode::CompareByteImm
            | Opcode::CompareShortImm
            | Opcode::CompareLongImm => {
                self.cmp(&opcode);
            }

            Opcode::MoveImmRegByte
            | Opcode::MoveImmRegShort
            | Opcode::MoveImmRegLong
            | Opcode::MoveRegRegByte
            | Opcode::MoveRegRegShort
            | Opcode::MoveRegRegLong
            | Opcode::MoveMemRegByte
            | Opcode::MoveMemRegShort
            | Opcode::MoveMemRegLong
            | Opcode::MoveAbsRegByte
            | Opcode::MoveAbsRegShort
            | Opcode::MoveAbsRegLong
            | Opcode::MoveIndirectRegByte
            | Opcode::MoveIndirectRegShort
            | Opcode::MoveIndirectRegLong => {
                self.mov_to_reg(&opcode);
            }
            Opcode::MoveImmMemByte
            | Opcode::MoveImmMemShort
            | Opcode::MoveImmMemLong
            | Opcode::MoveRegMemByte
            | Opcode::MoveRegMemShort
            | Opcode::MoveRegMemLong
            | Opcode::MoveMemMemByte
            | Opcode::MoveMemMemShort
            | Opcode::MoveMemMemLong
            | Opcode::MoveAbsMemByte
            | Opcode::MoveAbsMemShort
            | Opcode::MoveAbsMemLong
            | Opcode::MoveIndirectMemByte
            | Opcode::MoveIndirectMemShort
            | Opcode::MoveIndirectMemLong => {
                self.mov_to_rel(&opcode);
            }
            Opcode::MoveImmAbsByte
            | Opcode::MoveImmAbsShort
            | Opcode::MoveImmAbsLong
            | Opcode::MoveRegAbsByte
            | Opcode::MoveRegAbsShort
            | Opcode::MoveRegAbsLong
            | Opcode::MoveMemAbsByte
            | Opcode::MoveMemAbsShort
            | Opcode::MoveMemAbsLong
            | Opcode::MoveAbsAbsByte
            | Opcode::MoveAbsAbsShort
            | Opcode::MoveAbsAbsLong
            | Opcode::MoveIndirectAbsByte
            | Opcode::MoveIndirectAbsShort
            | Opcode::MoveIndirectAbsLong => {
                self.mov_to_abs(&opcode);
            }
            Opcode::MoveImmIndirectByte
            | Opcode::MoveImmIndirectShort
            | Opcode::MoveImmIndirectLong
            | Opcode::MoveRegIndirectByte
            | Opcode::MoveRegIndirectShort 
            | Opcode::MoveRegIndirectLong
            | Opcode::MoveMemIndirectByte 
            | Opcode::MoveMemIndirectShort 
            | Opcode::MoveMemIndirectLong
            | Opcode::MoveAbsIndirectByte
            | Opcode::MoveAbsIndirectShort
            | Opcode::MoveAbsIndirectLong
            | Opcode::MoveIndirectIndirectByte
            | Opcode::MoveIndirectIndirectShort
            | Opcode::MoveIndirectIndirectLong => {
                self.mov_to_ind(&opcode);
            }

            Opcode::AndShortImm
            | Opcode::AndShortReg
            | Opcode::AndShortMem
            | Opcode::AndLongImm
            | Opcode::AndLongReg
            | Opcode::AndLongMem
            | Opcode::AndByteImm
            | Opcode::AndByteReg
            | Opcode::AndByteMem => self.and(&opcode),

            Opcode::AddByteImm
            | Opcode::AddByteReg
            | Opcode::AddByteMem
            | Opcode::DivByteImm
            | Opcode::DivByteReg
            | Opcode::DivByteMem
            | Opcode::MulByteImm
            | Opcode::MulByteReg
            | Opcode::MulByteMem
            | Opcode::SubByteImm
            | Opcode::SubByteReg
            | Opcode::SubByteMem => {
                self.arith_byte(&opcode);
            }
            Opcode::AddShortImm
            | Opcode::AddShortReg
            | Opcode::AddShortMem
            | Opcode::DivShortImm
            | Opcode::DivShortReg
            | Opcode::DivShortMem
            | Opcode::MulShortImm
            | Opcode::MulShortReg
            | Opcode::MulShortMem
            | Opcode::SubShortImm
            | Opcode::SubShortReg
            | Opcode::SubShortMem => {
                self.arith_short(&opcode);
            }
            Opcode::AddLongImm
            | Opcode::AddLongReg
            | Opcode::AddLongMem
            | Opcode::DivLongImm
            | Opcode::DivLongReg
            | Opcode::DivLongMem
            | Opcode::MulLongImm
            | Opcode::MulLongReg
            | Opcode::MulLongMem
            | Opcode::SubLongImm
            | Opcode::SubLongReg
            | Opcode::SubLongMem => {
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
            
            
            Opcode::RustCall => {
                let idx = self.next_byte() as usize;
                
                match idx {
                    0 => {
                        functions::log_memory(self);
                    }
                    1 => {
                        functions::print(self);
                    }
                    _ => {
                        panic!("invalid rust function: {}", idx);
                    } 
                }
                
                
            }
            
            
            Opcode::Hlt => {
                self.registers[FLAGS] = (self.flags() | Cpu::HALT_FLAG) as u32;
            }
            
            Opcode::Nop => {
                
            }
        }
    }
}

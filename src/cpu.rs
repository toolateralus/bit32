use crate::functions::{self, log_opcode};
use crate::opcodes::Opcode;
use core::fmt;
use std::fmt::Debug;
use std::io::Read;
use std::ops::{Neg, Not, Shl, Shr};
use std::str::Utf8Error;

pub const NUM_REGISTERS: usize = 22;

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
    pub registers: [u32; NUM_REGISTERS],
    pub memory: Memory,
    pub hardware_interrupt_routine: Option<Box<dyn Fn(&mut Cpu) + Send + Sync>>,
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
    pub const VGA_BUFFER_LEN: usize = 320 * 200 * 2; // Each character has 2 bytes (char + color)
    pub const VGA_BUFFER_ADDRESS: usize = 0xA0000;

    pub const HALT_FLAG: u32 = 1 << 0;
    pub const INTERRUPT_FLAG: u32 = 1 << 1;
    pub const CARRY_FLAG: u32 = 1 << 2;

    pub fn new() -> Self {
        let mut cpu = Cpu {
            registers: [0; NUM_REGISTERS],
            memory: Memory::new(),
            hardware_interrupt_routine: None,
        };

        // TODO: remove this after testing.
        // just a default stack so we don't have to set it up constantly.
        let bp = cpu.memory.buffer.len() - 20;
        cpu.registers[BP] = bp as u32;

        let sp = bp - 1000;
        cpu.registers[SP] = sp as u32;

        for i in (Cpu::VGA_BUFFER_ADDRESS..Cpu::VGA_BUFFER_ADDRESS + Cpu::VGA_BUFFER_LEN).step_by(2)
        {
            cpu.memory.buffer[i] = b' ';
            cpu.memory.buffer[i + 1] = 0x0;
        }

        return cpu;
    }

    pub fn run(&mut self) {
        while !self.has_flag(Cpu::HALT_FLAG) {
            self.cycle();
        }
    }
    pub fn flags(&self) -> u32 {
        self.registers[FLAGS] as u32
    }
    pub fn has_flag(&self, flag: u32) -> bool {
        (self.flags() & flag) == flag
    }
    pub fn set_flag(&mut self, flag: u32, set: bool) {
        self.registers[FLAGS] = (self.registers[FLAGS] & !flag) | (-(set as i32) as u32 & flag);
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
                self.registers[0] = if lhs as u8 == rhs { 1 } else { 0 };
            }
            Opcode::CompareShortImm => {
                let lhs = self.registers[0];
                let rhs = self.next_short();
                self.registers[0] = if lhs as u16 == rhs { 1 } else { 0 };
            }
            Opcode::CompareLongImm => {
                let lhs = self.registers[0];
                let rhs = self.next_long();
                self.registers[0] = if lhs == rhs { 1 } else { 0 };
            }
            Opcode::CompareByteReg => {
                let lhs = self.registers[0];
                let index = self.next_byte() as usize;
                let rhs = self.registers[index] as u8;
                self.registers[0] = if lhs as u8 == rhs { 1 } else { 0 };
            }
            Opcode::CompareShortReg => {
                let lhs = self.registers[0];
                let index = self.next_byte() as usize;
                let rhs = self.registers[index] as u16;
                self.registers[0] = if lhs as u16 == rhs { 1 } else { 0 };
            }
            Opcode::CompareLongReg => {
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
            _ => {
                panic!("invalid and instruction");
            }
        }
    }
}

// Jumps
impl Cpu {
    fn jump_cmp(&mut self, op: &Opcode) {
        let lhs = self.registers[0];
        let rhs = self.registers[1];
        let addr = self.next_long();
        if match op {
            Opcode::JumpEqual => lhs == rhs,
            Opcode::JumpNotEqual => lhs != rhs,
            Opcode::JumpGreater => lhs > rhs,
            Opcode::JumpGreaterEqual => lhs >= rhs,
            Opcode::JumpLess => lhs < rhs,
            Opcode::JumpLessEqual => lhs <= rhs,
            Opcode::JumpSignedGreater => (lhs as i32) > rhs as i32,
            Opcode::JumpSignedGreaterEqual => (lhs as i32) >= rhs as i32,
            Opcode::JumpSignedLess => (lhs as i32) < rhs as i32,
            Opcode::JumpSignedLessEqual => (lhs as i32) <= rhs as i32,
            _ => panic!("invalid jump instruction"),
        } {
            self.registers[IP] = addr;
        }
    }
    fn jmp_imm(&mut self) {
        let addr = self.next_long();
        self.registers[IP] = addr;
    }
    fn jmp_reg(&mut self) {
        let index = self.next_byte() as usize;
        let addr = self.registers[index];
        self.registers[IP] = addr;
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
            Opcode::PushShortReg => {
                self.dec_sp(2);
                let index = self.next_byte() as usize;
                let value = (self.registers[index] & 0xFFFF) as u16;
                self.memory.set_short(self.sp(), value);
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
            Opcode::PopByte => {
                let dest = self.next_byte() as usize;
                let value = self.memory.byte(self.sp());
                self.registers[dest] = value as u32;
                self.inc_sp(1);
            }
            Opcode::PopShort => {
                let dest = self.next_byte() as usize;
                let value = self.memory.short(self.sp());
                self.registers[dest] = value as u32;
                self.inc_sp(2);
            }
            Opcode::PopLong => {
                let dest = self.next_byte() as usize;
                let value = self.memory.long(self.sp());
                self.registers[dest] = value as u32;
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
                let (result, carry) = lhs.overflowing_add(rhs);
                self.registers[0] = result;
                self.set_flag(Cpu::CARRY_FLAG, carry);
            }
            Opcode::AddLongReg => {
                let index = self.next_byte() as usize;
                let rhs = self.registers[index];
                let (result, carry) = lhs.overflowing_add(rhs);
                self.registers[0] = result;
                self.set_flag(Cpu::CARRY_FLAG, carry);
            }
            Opcode::SubLongImm => {
                let rhs = self.next_long();
                let (result, carry) = lhs.overflowing_sub(rhs);
                self.registers[0] = result;
                self.set_flag(Cpu::CARRY_FLAG, carry);
            }
            Opcode::SubLongReg => {
                let index = self.next_byte() as usize;
                let rhs = self.registers[index];
                let (result, carry) = lhs.overflowing_sub(rhs);
                self.registers[0] = result;
                self.set_flag(Cpu::CARRY_FLAG, carry);
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

            Opcode::AddCarryLongImm => {
                let rhs = self.next_long();
                let carry = self.has_flag(Cpu::CARRY_FLAG) as u32;
                let (result, carry0) = lhs.overflowing_add(carry);
                let (result, carry1) = result.overflowing_add(rhs);
                self.registers[0] = result;
                self.set_flag(Cpu::CARRY_FLAG, carry0 | carry1);
            }
            Opcode::AddCarryLongReg => {
                let index = self.next_byte() as usize;
                let rhs = self.registers[index];
                let carry = self.has_flag(Cpu::CARRY_FLAG) as u32;
                let (result, carry0) = lhs.overflowing_add(carry);
                let (result, carry1) = result.overflowing_add(rhs);
                self.registers[0] = result;
                self.set_flag(Cpu::CARRY_FLAG, carry0 | carry1);
            }
            Opcode::SubBorrowLongImm => {
                let rhs = self.next_long();
                let carry = self.has_flag(Cpu::CARRY_FLAG) as u32;
                let (result, carry0) = lhs.overflowing_sub(carry);
                let (result, carry1) = result.overflowing_sub(rhs);
                self.registers[0] = result;
                self.set_flag(Cpu::CARRY_FLAG, carry0 | carry1);
            }
            Opcode::SubBorrowLongReg => {
                let index = self.next_byte() as usize;
                let rhs = self.registers[index];
                let carry = self.has_flag(Cpu::CARRY_FLAG) as u32;
                let (result, carry0) = lhs.overflowing_sub(carry);
                let (result, carry1) = result.overflowing_sub(rhs);
                self.registers[0] = result;
                self.set_flag(Cpu::CARRY_FLAG, carry0 | carry1);
            }
            Opcode::SignedDivLongImm => {
                let rhs = self.next_long() as i32;
                let quotient = lhs as i32 / rhs;
                let remainder = lhs as i32 % rhs;
                self.registers[0] = quotient as u32;
                self.registers[1] = remainder as u32;
            }
            Opcode::SignedDivLongReg => {
                let index = self.next_byte() as usize;
                let rhs = self.registers[index] as i32;
                let quotient = lhs as i32 / rhs;
                let remainder = lhs as i32 % rhs;
                self.registers[0] = quotient as u32;
                self.registers[1] = remainder as u32;
            }
            Opcode::SignedMulLongImm => {
                let rhs = self.next_long() as i32;
                let result = (lhs as i32).wrapping_mul(rhs);
                self.registers[0] = result as u32;
            }
            Opcode::SignedMulLongReg => {
                let index = self.next_byte() as usize;
                let rhs = self.registers[index] as i32;
                let result = (lhs as i32).wrapping_mul(rhs);
                self.registers[0] = result as u32;
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
                let (result, carry) = lhs.overflowing_add(rhs);
                self.registers[0] = result as u32;
                self.set_flag(Cpu::CARRY_FLAG, carry);
            }
            Opcode::AddShortReg => {
                let index = self.next_byte() as usize;
                let rhs = (self.registers[index] & 0xFFFF) as u16;
                let (result, carry) = lhs.overflowing_add(rhs);
                self.registers[0] = result as u32;
                self.set_flag(Cpu::CARRY_FLAG, carry);
            }
            Opcode::SubShortImm => {
                let rhs = self.next_short();
                let (result, carry) = lhs.overflowing_sub(rhs);
                self.registers[0] = result as u32;
                self.set_flag(Cpu::CARRY_FLAG, carry);
            }
            Opcode::SubShortReg => {
                let index = self.next_byte() as usize;
                let rhs = (self.registers[index] & 0xFFFF) as u16;
                let (result, carry) = lhs.overflowing_sub(rhs);
                self.registers[0] = result as u32;
                self.set_flag(Cpu::CARRY_FLAG, carry);
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

            Opcode::AddCarryShortImm => {
                let rhs = self.next_short();
                let carry = self.has_flag(Cpu::CARRY_FLAG) as u16;
                let (result, carry0) = lhs.overflowing_add(carry);
                let (result, carry1) = result.overflowing_add(rhs);
                self.registers[0] = result as u32;
                self.set_flag(Cpu::CARRY_FLAG, carry0 | carry1);
            }
            Opcode::AddCarryShortReg => {
                let index = self.next_byte() as usize;
                let rhs = (self.registers[index] & 0xFFFF) as u16;
                let carry = self.has_flag(Cpu::CARRY_FLAG) as u16;
                let (result, carry0) = lhs.overflowing_add(carry);
                let (result, carry1) = result.overflowing_add(rhs);
                self.registers[0] = result as u32;
                self.set_flag(Cpu::CARRY_FLAG, carry0 | carry1);
            }
            Opcode::SubBorrowShortImm => {
                let rhs = self.next_short();
                let carry = self.has_flag(Cpu::CARRY_FLAG) as u16;
                let (result, carry0) = lhs.overflowing_sub(carry);
                let (result, carry1) = result.overflowing_sub(rhs);
                self.registers[0] = result as u32;
                self.set_flag(Cpu::CARRY_FLAG, carry0 | carry1);
            }
            Opcode::SubBorrowShortReg => {
                let index = self.next_byte() as usize;
                let rhs = (self.registers[index] & 0xFFFF) as u16;
                let carry = self.has_flag(Cpu::CARRY_FLAG) as u16;
                let (result, carry0) = lhs.overflowing_sub(carry);
                let (result, carry1) = result.overflowing_sub(rhs);
                self.registers[0] = result as u32;
                self.set_flag(Cpu::CARRY_FLAG, carry0 | carry1);
            }
            Opcode::SignedDivShortImm => {
                let rhs = self.next_short() as i16;
                let quotient = (lhs as i16) / rhs;
                let remainder = (lhs as i16) % rhs;
                self.registers[0] = quotient as u32;
                self.registers[1] = remainder as u32;
            }
            Opcode::SignedDivShortReg => {
                let index = self.next_byte() as usize;
                let rhs = (self.registers[index] & 0xFFFF) as i16;
                let quotient = (lhs as i16) / rhs;
                let remainder = (lhs as i16) % rhs;
                self.registers[0] = quotient as u32;
                self.registers[1] = remainder as u32;
            }
            Opcode::SignedMulShortImm => {
                let rhs = self.next_short() as i16;
                let result = (lhs as i16).wrapping_mul(rhs);
                self.registers[0] = result as u32;
            }
            Opcode::SignedMulShortReg => {
                let index = self.next_byte() as usize;
                let rhs = (self.registers[index] & 0xFFFF) as i16;
                let result = (lhs as i16).wrapping_mul(rhs);
                self.registers[0] = result as u32;
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
                let (result, carry) = lhs.overflowing_add(rhs);
                self.registers[0] = result as u32;
                self.set_flag(Cpu::CARRY_FLAG, carry);
            }
            Opcode::AddByteReg => {
                let index = self.next_byte() as usize;
                let rhs = (self.registers[index] & 0xFF) as u8;
                let (result, carry) = lhs.overflowing_add(rhs);
                self.registers[0] = result as u32;
                self.set_flag(Cpu::CARRY_FLAG, carry);
            }
            Opcode::SubByteImm => {
                let rhs = self.next_byte();
                let (result, carry) = lhs.overflowing_sub(rhs);
                self.registers[0] = result as u32;
                self.set_flag(Cpu::CARRY_FLAG, carry);
            }
            Opcode::SubByteReg => {
                let index = self.next_byte() as usize;
                let rhs = (self.registers[index] & 0xFF) as u8;
                let (result, carry) = lhs.overflowing_sub(rhs);
                self.registers[0] = result as u32;
                self.set_flag(Cpu::CARRY_FLAG, carry);
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

            Opcode::AddCarryByteImm => {
                let rhs = self.next_byte();
                let carry = self.has_flag(Cpu::CARRY_FLAG) as u8;
                let (result, carry0) = lhs.overflowing_add(carry);
                let (result, carry1) = result.overflowing_add(rhs);
                self.registers[0] = result as u32;
                self.set_flag(Cpu::CARRY_FLAG, carry0 | carry1);
            }
            Opcode::AddCarryByteReg => {
                let index = self.next_byte() as usize;
                let rhs = (self.registers[index] & 0xFF) as u8;
                let carry = self.has_flag(Cpu::CARRY_FLAG) as u8;
                let (result, carry0) = lhs.overflowing_add(carry);
                let (result, carry1) = result.overflowing_add(rhs);
                self.registers[0] = result as u32;
                self.set_flag(Cpu::CARRY_FLAG, carry0 | carry1);
            }
            Opcode::SubBorrowByteImm => {
                let rhs = self.next_byte();
                let carry = self.has_flag(Cpu::CARRY_FLAG) as u8;
                let (result, carry0) = lhs.overflowing_sub(carry);
                let (result, carry1) = result.overflowing_sub(rhs);
                self.registers[0] = result as u32;
                self.set_flag(Cpu::CARRY_FLAG, carry0 | carry1);
            }
            Opcode::SubBorrowByteReg => {
                let index = self.next_byte() as usize;
                let rhs = (self.registers[index] & 0xFF) as u8;
                let carry = self.has_flag(Cpu::CARRY_FLAG) as u8;
                let (result, carry0) = lhs.overflowing_sub(carry);
                let (result, carry1) = result.overflowing_sub(rhs);
                self.registers[0] = result as u32;
                self.set_flag(Cpu::CARRY_FLAG, carry0 | carry1);
            }
            Opcode::SignedDivByteImm => {
                let rhs = self.next_byte() as i8;
                let quotient = (lhs as i8) / rhs;
                let remainder = (lhs as i8) % rhs;
                self.registers[0] = quotient as u32;
                self.registers[1] = remainder as u32;
            }
            Opcode::SignedDivByteReg => {
                let index = self.next_byte() as usize;
                let rhs = self.registers[index] as i8;
                let quotient = (lhs as i8) / rhs;
                let remainder = (lhs as i8) % rhs;
                self.registers[0] = quotient as u32;
                self.registers[1] = remainder as u32;
            }
            Opcode::SignedMulByteImm => {
                let rhs = self.next_byte() as i8;
                let result = (lhs as i8).wrapping_mul(rhs);
                self.registers[0] = result as u32;
            }
            Opcode::SignedMulByteReg => {
                let index = self.next_byte() as usize;
                let rhs = self.registers[index] as i8;
                let result = (lhs as i8).wrapping_mul(rhs);
                self.registers[0] = result as u32;
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
        let dst_reg = self.next_byte() as usize;
        match opcode {
            //from immediate
            Opcode::MoveImmRegByte => {
                let src_val = self.next_byte();
                self.validate_register(dst_reg);
                self.registers[dst_reg] = src_val as u32;
            }
            Opcode::MoveImmRegShort => {
                let src_val = self.next_short();
                self.validate_register(dst_reg);
                self.registers[dst_reg] = src_val as u32;
            }
            Opcode::MoveImmRegLong => {
                let src_val = self.next_long();
                self.validate_register(dst_reg);
                self.registers[dst_reg] = src_val;
            }

            // from register
            Opcode::MoveRegRegByte => {
                let src_reg = self.next_byte() as usize;
                self.validate_registers(&[dst_reg, src_reg]);
                self.registers[dst_reg] = self.registers[src_reg] & 0xFF;
            }
            Opcode::MoveRegRegShort => {
                let src_reg = self.next_byte() as usize;
                self.validate_registers(&[dst_reg, src_reg]);
                self.registers[dst_reg] = self.registers[src_reg] & 0xFFFF;
            }
            Opcode::MoveRegRegLong => {
                let src_reg = self.next_byte() as usize;
                self.validate_registers(&[dst_reg, src_reg]);
                self.registers[dst_reg] = self.registers[src_reg];
            }

            // from relative memory
            Opcode::MoveMemRegByte => {
                let src_adr = self.next_long() as usize;
                let src_val = self.memory.byte(src_adr + self.ip()) as u32;
                self.registers[dst_reg] = src_val;
            }
            Opcode::MoveMemRegShort => {
                let src_adr = self.next_long() as usize;
                let src_val = self.memory.short(src_adr + self.ip()) as u32;
                self.registers[dst_reg] = src_val;
            }
            Opcode::MoveMemRegLong => {
                let src_adr = self.next_long() as usize;
                let src_val = self.memory.long(src_adr + self.ip());
                self.registers[dst_reg] = src_val;
            }

            // from absolute memory
            Opcode::MoveAbsRegByte => {
                let src_adr = self.next_long() as usize;
                self.registers[dst_reg] = self.memory.byte(src_adr) as u32;
            }
            Opcode::MoveAbsRegShort => {
                let src_adr = self.next_long() as usize;
                self.registers[dst_reg] = self.memory.short(src_adr) as u32;
            }
            Opcode::MoveAbsRegLong => {
                let src_adr = self.next_long() as usize;
                self.registers[dst_reg] = self.memory.long(src_adr);
            }

            // from indirect memory
            Opcode::MoveIndirectRegByte => {
                let src_reg = self.next_byte() as usize;
                let src_adr = self.registers[src_reg] as usize;
                self.registers[dst_reg] = self.memory.byte(src_adr) as u32;
            }
            Opcode::MoveIndirectRegShort => {
                let src_reg = self.next_byte() as usize;
                let src_adr = self.registers[src_reg] as usize;
                self.registers[dst_reg] = self.memory.short(src_adr) as u32;
            }
            Opcode::MoveIndirectRegLong => {
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
        let dst_adr = self.next_long() as usize;
        match opcode {
            // from immediate
            Opcode::MoveImmMemByte => {
                let src_val = self.next_byte();
                self.memory.set_byte(dst_adr + self.ip(), src_val);
            }
            Opcode::MoveImmMemShort => {
                let src_val = self.next_short();
                self.memory.set_short(dst_adr + self.ip(), src_val);
            }
            Opcode::MoveImmMemLong => {
                let src_val = self.next_long();
                self.memory.set_long(dst_adr + self.ip(), src_val);
            }

            // from register
            Opcode::MoveRegMemByte => {
                let src_reg = self.next_byte() as usize;
                self.memory
                    .set_byte(dst_adr + self.ip(), self.registers[src_reg] as u8);
            }
            Opcode::MoveRegMemShort => {
                let src_reg = self.next_byte() as usize;
                self.memory
                    .set_short(dst_adr + self.ip(), self.registers[src_reg] as u16);
            }
            Opcode::MoveRegMemLong => {
                let src_reg = self.next_byte() as usize;
                self.memory
                    .set_long(dst_adr + self.ip(), self.registers[src_reg]);
            }

            // from relative memory
            Opcode::MoveMemMemByte => {
                let src_adr = self.next_long() as usize;
                let src_val = self.memory.byte(src_adr + self.ip());
                self.memory.set_byte(dst_adr + self.ip(), src_val);
            }
            Opcode::MoveMemMemShort => {
                let src_adr = self.next_long() as usize;
                let src_val = self.memory.short(src_adr + self.ip());
                self.memory.set_short(dst_adr + self.ip(), src_val);
            }
            Opcode::MoveMemMemLong => {
                let src_adr = self.next_long() as usize;
                let src_val = self.memory.long(src_adr + self.ip());
                self.memory.set_long(dst_adr + self.ip(), src_val);
            }

            // from absolute memory
            Opcode::MoveAbsMemByte => {
                let src_adr = self.next_long() as usize;
                let src_val = self.memory.byte(src_adr);
                self.memory.set_byte(dst_adr + self.ip(), src_val);
            }
            Opcode::MoveAbsMemShort => {
                let src_adr = self.next_long() as usize;
                let src_val = self.memory.short(src_adr);
                self.memory.set_short(dst_adr + self.ip(), src_val);
            }
            Opcode::MoveAbsMemLong => {
                let src_adr = self.next_long() as usize;
                let src_val = self.memory.long(src_adr);
                self.memory.set_long(dst_adr + self.ip(), src_val);
            }

            // from Indirect memory
            Opcode::MoveIndirectMemByte => {
                let src_reg = self.next_byte() as usize;
                let src_val = self.memory.byte(self.registers[src_reg] as usize);
                self.memory.set_byte(dst_adr + self.ip(), src_val);
            }
            Opcode::MoveIndirectMemShort => {
                let src_reg = self.next_byte() as usize;
                let src_val = self.memory.short(self.registers[src_reg] as usize);
                self.memory.set_short(dst_adr + self.ip(), src_val);
            }
            Opcode::MoveIndirectMemLong => {
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
        let dst_adr = self.next_long() as usize;
        match opcode {
            // from immediate
            Opcode::MoveImmAbsByte => {
                let src_val = self.next_byte();
                self.memory.set_byte(dst_adr, src_val);
            }
            Opcode::MoveImmAbsShort => {
                let src_val = self.next_short();
                self.memory.set_short(dst_adr, src_val);
            }
            Opcode::MoveImmAbsLong => {
                let src_val = self.next_long();
                self.memory.set_long(dst_adr, src_val);
            }

            // from register
            Opcode::MoveRegAbsByte => {
                let src_reg = self.next_byte() as usize;
                self.memory.set_byte(dst_adr, self.registers[src_reg] as u8);
            }
            Opcode::MoveRegAbsShort => {
                let src_reg = self.next_byte() as usize;
                self.memory
                    .set_short(dst_adr, self.registers[src_reg] as u16);
            }
            Opcode::MoveRegAbsLong => {
                let src_reg = self.next_byte() as usize;
                self.memory.set_long(dst_adr, self.registers[src_reg]);
            }

            // from relative memory
            Opcode::MoveMemAbsByte => {
                let src_adr = self.next_long() as usize;
                let src_val = self.memory.byte(src_adr + self.ip());
                self.memory.set_byte(dst_adr, src_val);
            }
            Opcode::MoveMemAbsShort => {
                let src_adr = self.next_long() as usize;
                let src_val = self.memory.short(src_adr + self.ip());
                self.memory.set_short(dst_adr, src_val);
            }
            Opcode::MoveMemAbsLong => {
                let src_adr = self.next_long() as usize;
                let src_val = self.memory.long(src_adr + self.ip());
                self.memory.set_long(dst_adr, src_val);
            }

            // from absolute memory
            Opcode::MoveAbsAbsByte => {
                let src_adr = self.next_long() as usize;
                let src_val = self.memory.byte(src_adr);
                self.memory.set_byte(dst_adr, src_val);
            }
            Opcode::MoveAbsAbsShort => {
                let src_adr = self.next_long() as usize;
                let src_val = self.memory.short(src_adr);
                self.memory.set_short(dst_adr, src_val);
            }
            Opcode::MoveAbsAbsLong => {
                let src_adr = self.next_long() as usize;
                let src_val = self.memory.long(src_adr);
                self.memory.set_long(dst_adr, src_val);
            }

            // from Indirect memory
            Opcode::MoveIndirectAbsByte => {
                let src_reg = self.next_byte() as usize;
                let src_val = self.memory.byte(self.registers[src_reg] as usize);
                self.memory.set_byte(dst_adr, src_val);
            }
            Opcode::MoveIndirectAbsShort => {
                let src_reg = self.next_byte() as usize;
                let src_val = self.memory.short(self.registers[src_reg] as usize);
                self.memory.set_short(dst_adr, src_val);
            }
            Opcode::MoveIndirectAbsLong => {
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
        let dst_reg = self.next_byte() as usize;
        match opcode {
            // from immediate
            Opcode::MoveImmIndirectByte => {
                let src_val = self.next_byte();
                let dst_adr = self.registers[dst_reg] as usize;
                self.memory.set_byte(dst_adr, src_val);
            }
            Opcode::MoveImmIndirectShort => {
                let src_val = self.next_short();
                let dst_adr = self.registers[dst_reg] as usize;
                self.memory.set_short(dst_adr, src_val);
            }
            Opcode::MoveImmIndirectLong => {
                let src_val = self.next_long();
                let dst_adr = self.registers[dst_reg] as usize;
                self.memory.set_long(dst_adr, src_val);
            }

            // from register
            Opcode::MoveRegIndirectByte => {
                let src_reg = self.next_byte() as usize;
                let dst_adr = self.registers[dst_reg] as usize;
                self.memory.set_byte(dst_adr, self.registers[src_reg] as u8);
            }
            Opcode::MoveRegIndirectShort => {
                let src_reg = self.next_byte() as usize;
                let dst_adr = self.registers[dst_reg] as usize;
                self.memory
                    .set_short(dst_adr, self.registers[src_reg] as u16);
            }
            Opcode::MoveRegIndirectLong => {
                let src_reg = self.next_byte() as usize;
                let dst_adr = self.registers[dst_reg] as usize;
                self.memory.set_long(dst_adr, self.registers[src_reg]);
            }

            // from relative memory
            Opcode::MoveMemIndirectByte => {
                let src_adr = self.next_long() as usize;
                let dst_adr = self.registers[dst_reg] as usize;
                let src_val = self.memory.byte(src_adr + self.ip());
                self.memory.set_byte(dst_adr, src_val);
            }
            Opcode::MoveMemIndirectShort => {
                let src_adr = self.next_long() as usize;
                let dst_adr = self.registers[dst_reg] as usize;
                let src_val = self.memory.short(src_adr + self.ip());
                self.memory.set_short(dst_adr, src_val);
            }
            Opcode::MoveMemIndirectLong => {
                let src_adr = self.next_long() as usize;
                let dst_adr = self.registers[dst_reg] as usize;
                let src_val = self.memory.long(src_adr + self.ip());
                self.memory.set_long(dst_adr, src_val);
            }

            // from absolute memory
            Opcode::MoveAbsIndirectByte => {
                let src_adr = self.next_long() as usize;
                let dst_adr = self.registers[dst_reg] as usize;
                let src_val = self.memory.byte(src_adr);
                self.memory.set_byte(dst_adr, src_val);
            }
            Opcode::MoveAbsIndirectShort => {
                let src_adr = self.next_long() as usize;
                let dst_adr = self.registers[dst_reg] as usize;
                let src_val = self.memory.short(src_adr);
                self.memory.set_short(dst_adr, src_val);
            }
            Opcode::MoveAbsIndirectLong => {
                let src_adr = self.next_long() as usize;
                let dst_adr = self.registers[dst_reg] as usize;
                let src_val = self.memory.long(src_adr);
                self.memory.set_long(dst_adr, src_val);
            }

            // from indirect memory
            Opcode::MoveIndirectIndirectByte => {
                let src_reg = self.next_byte() as usize;
                let dst_adr = self.registers[dst_reg] as usize;
                let src_adr = self.registers[src_reg] as usize;
                let src_val = self.memory.byte(src_adr);
                self.memory.set_byte(dst_adr, src_val);
            }
            Opcode::MoveIndirectIndirectShort => {
                let src_reg = self.next_byte() as usize;
                let dst_adr = self.registers[dst_reg] as usize;
                let src_adr = self.registers[src_reg] as usize;
                let src_val = self.memory.short(src_adr);
                self.memory.set_short(dst_adr, src_val);
            }
            Opcode::MoveIndirectIndirectLong => {
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
        let flags = self.registers[FLAGS];

        // Extract the function pointer to avoid borrowing issues
        if flags & Cpu::INTERRUPT_FLAG == Cpu::INTERRUPT_FLAG {
            if let Some(hw_interrupt) = self.hardware_interrupt_routine.take() {
                hw_interrupt(self);
                self.hardware_interrupt_routine = None;
            }
        }

        let instruction = self.next_byte();
        let opcode = Opcode::from(instruction);

        if false {
            log_opcode(&opcode);
        }

        match opcode {
            Opcode::Interrupt => {
                if self.registers[FLAGS] & Cpu::INTERRUPT_FLAG as u32 != Cpu::INTERRUPT_FLAG as u32
                {
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
                self.registers[IP] = self.memory.long(isr_addr as usize);
            }
            Opcode::InterruptReturn => {
                let ret_addr = self.memory.long(self.sp());
                self.inc_sp(4);
                self.registers[IP] = ret_addr;
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

            Opcode::JumpEqual
            | Opcode::JumpNotEqual
            | Opcode::JumpGreater
            | Opcode::JumpGreaterEqual
            | Opcode::JumpLess
            | Opcode::JumpLessEqual
            | Opcode::JumpSignedGreater
            | Opcode::JumpSignedGreaterEqual
            | Opcode::JumpSignedLess
            | Opcode::JumpSignedLessEqual => {
                self.jump_cmp(&opcode);
            }

            Opcode::JumpReg => {
                self.jmp_reg();
            }
            Opcode::JumpImm => {
                self.jmp_imm();
            }

            Opcode::CompareByteImm
            | Opcode::CompareShortImm
            | Opcode::CompareLongImm
            | Opcode::CompareByteReg
            | Opcode::CompareShortReg
            | Opcode::CompareLongReg => {
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
            | Opcode::AndLongImm
            | Opcode::AndLongReg
            | Opcode::AndByteImm
            | Opcode::AndByteReg => self.and(&opcode),

            Opcode::AddByteImm
            | Opcode::AddByteReg
            | Opcode::DivByteImm
            | Opcode::DivByteReg
            | Opcode::MulByteImm
            | Opcode::MulByteReg
            | Opcode::SubByteImm
            | Opcode::SubByteReg
            | Opcode::AddCarryByteImm
            | Opcode::AddCarryByteReg
            | Opcode::SignedDivByteImm
            | Opcode::SignedDivByteReg
            | Opcode::SignedMulByteImm
            | Opcode::SignedMulByteReg
            | Opcode::SubBorrowByteImm
            | Opcode::SubBorrowByteReg => {
                self.arith_byte(&opcode);
            }
            Opcode::AddShortImm
            | Opcode::AddShortReg
            | Opcode::DivShortImm
            | Opcode::DivShortReg
            | Opcode::MulShortImm
            | Opcode::MulShortReg
            | Opcode::SubShortImm
            | Opcode::SubShortReg
            | Opcode::AddCarryShortImm
            | Opcode::AddCarryShortReg
            | Opcode::SignedDivShortImm
            | Opcode::SignedDivShortReg
            | Opcode::SignedMulShortImm
            | Opcode::SignedMulShortReg
            | Opcode::SubBorrowShortImm
            | Opcode::SubBorrowShortReg => {
                self.arith_short(&opcode);
            }
            Opcode::AddLongImm
            | Opcode::AddLongReg
            | Opcode::DivLongImm
            | Opcode::DivLongReg
            | Opcode::MulLongImm
            | Opcode::MulLongReg
            | Opcode::SubLongImm
            | Opcode::SubLongReg
            | Opcode::AddCarryLongImm
            | Opcode::AddCarryLongReg
            | Opcode::SubBorrowLongImm
            | Opcode::SubBorrowLongReg
            | Opcode::SignedDivLongImm
            | Opcode::SignedDivLongReg
            | Opcode::SignedMulLongImm
            | Opcode::SignedMulLongReg => {
                self.arith_long(&opcode);
            }

            Opcode::PushByteReg
            | Opcode::PushShortReg
            | Opcode::PushLongReg
            | Opcode::PushByteImm
            | Opcode::PushLongImm
            | Opcode::PushShortImm => {
                self.push(&opcode);
            }

            Opcode::PopByte | Opcode::PopShort | Opcode::PopLong => {
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
                self.set_flag(Cpu::HALT_FLAG, true);
            }

            Opcode::Nop => {}
            Opcode::OrByteImm => {
                let val = self.next_byte();
                let res = self.registers[0] as u8 | val;
                self.registers[0] = res as u32;
            }
            Opcode::XorByteImm => {
                let val = self.next_byte();
                let res = self.registers[0] as u8 ^ val;
                self.registers[0] = res as u32;
            }
            Opcode::OrShortImm => {
                let val = self.next_short();
                let res = self.registers[0] as u16 | val;
                self.registers[0] = res as u32;
            }
            Opcode::XorShortImm => {
                let val = self.next_short();
                let res = self.registers[0] as u16 ^ val;
                self.registers[0] = res as u32;
            }
            Opcode::OrLongImm => {
                let val = self.next_long();
                let res = self.registers[0] as u32 | val;
                self.registers[0] = res as u32;
            }
            Opcode::XorLongImm => {
                let val = self.next_long();
                let res = self.registers[0] as u32 ^ val;
                self.registers[0] = res as u32;
            }
            Opcode::OrByteReg => {
                let reg = self.next_byte() as usize;
                let val = self.registers[reg] as u8;
                let res = self.registers[0] as u8 | val;
                self.registers[0] = res as u32;
            }
            Opcode::XorByteReg => {
                let reg = self.next_byte() as usize;
                let val = self.registers[reg] as u8;
                let res = self.registers[0] as u8 ^ val;
                self.registers[0] = res as u32;
            }
            Opcode::OrShortReg => {
                let reg = self.next_short() as usize;
                let val = self.registers[reg] as u16;
                let res = self.registers[0] as u16 | val;
                self.registers[0] = res as u32;
            }
            Opcode::XorShortReg => {
                let reg = self.next_short() as usize;
                let val = self.registers[reg] as u16;
                let res = self.registers[0] as u16 ^ val;
                self.registers[0] = res as u32;
            }
            Opcode::OrLongReg => {
                let reg = self.next_long() as usize;
                let val = self.registers[reg] as u32;
                let res = self.registers[0] as u32 | val;
                self.registers[0] = res as u32;
            }
            Opcode::XorLongReg => {
                let reg = self.next_long() as usize;
                let val = self.registers[reg] as u32;
                let res = self.registers[0] as u32 ^ val;
                self.registers[0] = res as u32;
            }
            Opcode::LogShiftLeftImm => {
                let val = self.next_byte();
                self.registers[0] = (self.registers[0] as u32).shl(val);
            }
            Opcode::LogShiftRightImm => {
                let val = self.next_byte();
                self.registers[0] = (self.registers[0] as u32).shr(val);
            }
            Opcode::ArithShiftLeftImm => {
                let val = self.next_byte();
                self.registers[0] = (self.registers[0] as u32).shl(val);
            }
            Opcode::ArithShiftRightImm => {
                let val = self.next_byte();
                self.registers[0] = (self.registers[0] as u32).shr(val);
            }
            Opcode::RotateLeftImm => {
                let val = self.next_byte();
                self.registers[0] = self.registers[0].rotate_left(val as u32);
            }
            Opcode::RotateRightImm => {
                let val = self.next_byte();
                self.registers[0] = self.registers[0].rotate_right(val as u32);
            }
            Opcode::LogShiftLeftReg => {
                let reg = self.next_byte() as usize;
                let val = self.registers[reg];
                self.registers[0] = (self.registers[0] as u32).shl(val);
            }
            Opcode::LogShiftRightReg => {
                let reg = self.next_byte() as usize;
                let val = self.registers[reg];
                self.registers[0] = (self.registers[0] as u32).shr(val);
            }
            Opcode::ArithShiftLeftReg => {
                let reg = self.next_byte() as usize;
                let val = self.registers[reg];
                self.registers[0] = (self.registers[0] as u32).shl(val);
            }
            Opcode::ArithShiftRightReg => {
                let reg = self.next_byte() as usize;
                let val = self.registers[reg];
                self.registers[0] = (self.registers[0] as u32).shr(val);
            }
            Opcode::RotateLeftReg => {
                let reg = self.next_byte() as usize;
                let val = self.registers[reg];
                self.registers[0] = self.registers[0].rotate_left(val as u32);
            }
            Opcode::RotateRightReg => {
                let reg = self.next_byte() as usize;
                let val = self.registers[reg];
                self.registers[0] = self.registers[0].rotate_right(val as u32);
            }
            Opcode::NotByte => {
                let reg = self.next_byte() as usize;
                let val = (self.registers[reg] as i8).not();
                self.registers[reg] = val as u32;
            }
            Opcode::NotShort => {
                let reg = self.next_byte() as usize;
                let val = (self.registers[reg] as i16).not();
                self.registers[reg] = val as u32;
            }
            Opcode::NotLong => {
                let reg = self.next_byte() as usize;
                let val = (self.registers[reg] as i32).not();
                self.registers[reg] = val as u32;
            }
            Opcode::NegateByte => {
                let reg = self.next_byte() as usize;
                let val = (self.registers[reg] as i8).neg();
                self.registers[reg] = val as u32;
            }
            Opcode::NegateShort => {
                let reg = self.next_byte() as usize;
                let val = (self.registers[reg] as i16).neg();
                self.registers[reg] = val as u32;
            }
            Opcode::NegateLong => {
                let reg = self.next_byte() as usize;
                let val = (self.registers[reg] as i32).neg();
                self.registers[reg] = val as u32;
            }
            Opcode::IncrementByte => {
                let reg = self.next_byte() as usize;
                let val = (self.registers[reg] as u8).wrapping_add(1);
                self.registers[reg] = val as u32;
            }
            Opcode::IncrementShort => {
                let reg = self.next_byte() as usize;
                let val = (self.registers[reg] as u16).wrapping_add(1);
                self.registers[reg] = val as u32;
            }
            Opcode::IncrementLong => {
                let reg = self.next_byte() as usize;
                let val = (self.registers[reg] as u32).wrapping_add(1);
                self.registers[reg] = val as u32;
            }
            Opcode::DecrementByte => {
                let reg = self.next_byte() as usize;
                let val = (self.registers[reg] as u8).wrapping_sub(1);
                self.registers[reg] = val as u32;
            }
            Opcode::DecrementShort => {
                let reg = self.next_byte() as usize;
                let val = (self.registers[reg] as u16).wrapping_sub(1);
                self.registers[reg] = val as u32;
            }
            Opcode::DecrementLong => {
                let reg = self.next_byte() as usize;
                let val = (self.registers[reg] as u32).wrapping_sub(1);
                self.registers[reg] = val as u32;
            }
            Opcode::ClearCarry => {
                self.set_flag(Cpu::CARRY_FLAG, false);
            }
        }
    }
}

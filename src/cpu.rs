use crate::handlers::*;
use crate::opcodes::Opcode;
use core::fmt;
use std::fmt::Debug;
use std::io::Read;
use std::path::Path;
use std::str::Utf8Error;


pub type OpcodeHandler = fn(&mut Cpu);

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
    #[inline(always)]
    pub fn byte(&mut self, addr: usize) -> u8 {
        let b = self.buffer[addr];
        return b;
    }
    #[inline(always)]
    pub fn short(&mut self, addr: usize) -> u16 {
        let low = self.byte(addr) as u16;
        let high = self.byte(addr + 1) as u16;
        return (high << 8) | low;
    }
    #[inline(always)]
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

    #[inline(always)]
    pub fn set_long(&mut self, addr: usize, value: u32) {
        self.set_short(addr, value as u16);
        self.set_short(addr + 2, (value >> 16) as u16);
    }
    #[inline(always)]
    pub fn set_short(&mut self, addr: usize, value: u16) {
        self.set_byte(addr, value as u8);
        self.set_byte(addr + 1, (value >> 8) as u8);
    }
    #[inline(always)]
    pub fn set_byte(&mut self, addr: usize, value: u8) {
        if self.buffer.len() <= addr {
            panic!("memory access out of bounds {addr}");
        }
        self.buffer[addr] = value;
    }
}

pub type HardwareInterrupt = Box<dyn Fn(&mut Cpu) + Send + Sync>;

pub struct Cpu {
    pub registers: [u32; NUM_REGISTERS],
    pub memory: Memory,
    pub hardware_interrupt: Option<HardwareInterrupt>,
}

pub type OpcodeHandlerArray = [OpcodeHandler; 256];

pub const fn get_opcode_handlers() -> OpcodeHandlerArray {
    let mut handlers: OpcodeHandlerArray = [hlt; 256];

    handlers[Opcode::Hlt as usize] = hlt;
    handlers[Opcode::MoveImmRegByte as usize] = move_imm_reg_byte;
    handlers[Opcode::MoveImmRegShort as usize] = move_imm_reg_short;
    handlers[Opcode::MoveImmRegLong as usize] = move_imm_reg_long;

    handlers[Opcode::MoveRegRegByte as usize] = move_reg_reg_byte;
    handlers[Opcode::MoveRegRegShort as usize] = move_reg_reg_short;
    handlers[Opcode::MoveRegRegLong as usize] = move_reg_reg_long;

    handlers[Opcode::MoveAbsRegByte as usize] = move_abs_reg_byte;
    handlers[Opcode::MoveAbsRegShort as usize] = move_abs_reg_short;
    handlers[Opcode::MoveAbsRegLong as usize] = move_abs_reg_long;

    handlers[Opcode::MoveMemRegByte as usize] = move_mem_reg_byte;
    handlers[Opcode::MoveMemRegShort as usize] = move_mem_reg_short;
    handlers[Opcode::MoveMemRegLong as usize] = move_mem_reg_long;

    handlers[Opcode::MoveIndirectRegByte as usize] = move_indirect_reg_byte;
    handlers[Opcode::MoveIndirectRegShort as usize] = move_indirect_reg_short;
    handlers[Opcode::MoveIndirectRegLong as usize] = move_indirect_reg_long;

    handlers[Opcode::MoveImmAbsByte as usize] = move_imm_abs_byte;
    handlers[Opcode::MoveImmAbsShort as usize] = move_imm_abs_short;
    handlers[Opcode::MoveImmAbsLong as usize] = move_imm_abs_long;

    handlers[Opcode::MoveRegAbsByte as usize] = move_reg_abs_byte;
    handlers[Opcode::MoveRegAbsShort as usize] = move_reg_abs_short;
    handlers[Opcode::MoveRegAbsLong as usize] = move_reg_abs_long;

    handlers[Opcode::MoveAbsAbsByte as usize] = move_abs_abs_byte;
    handlers[Opcode::MoveAbsAbsShort as usize] = move_abs_abs_short;
    handlers[Opcode::MoveAbsAbsLong as usize] = move_abs_abs_long;

    handlers[Opcode::MoveMemAbsByte as usize] = move_mem_abs_byte;
    handlers[Opcode::MoveMemAbsShort as usize] = move_mem_abs_short;
    handlers[Opcode::MoveMemAbsLong as usize] = move_mem_abs_long;

    handlers[Opcode::MoveIndirectAbsByte as usize] = move_indirect_abs_byte;
    handlers[Opcode::MoveIndirectAbsShort as usize] = move_indirect_abs_short;
    handlers[Opcode::MoveIndirectAbsLong as usize] = move_indirect_abs_long;

    handlers[Opcode::MoveImmMemByte as usize] = move_imm_mem_byte;
    handlers[Opcode::MoveImmMemShort as usize] = move_imm_mem_short;
    handlers[Opcode::MoveImmMemLong as usize] = move_imm_mem_long;

    handlers[Opcode::MoveRegMemByte as usize] = move_reg_mem_byte;
    handlers[Opcode::MoveRegMemShort as usize] = move_reg_mem_short;
    handlers[Opcode::MoveRegMemLong as usize] = move_reg_mem_long;

    handlers[Opcode::MoveAbsMemByte as usize] = move_abs_mem_byte;
    handlers[Opcode::MoveAbsMemShort as usize] = move_abs_mem_short;
    handlers[Opcode::MoveAbsMemLong as usize] = move_abs_mem_long;

    handlers[Opcode::MoveMemMemByte as usize] = move_mem_mem_byte;
    handlers[Opcode::MoveMemMemShort as usize] = move_mem_mem_short;
    handlers[Opcode::MoveMemMemLong as usize] = move_mem_mem_long;

    handlers[Opcode::MoveIndirectMemByte as usize] = move_indirect_mem_byte;
    handlers[Opcode::MoveIndirectMemShort as usize] = move_indirect_mem_short;
    handlers[Opcode::MoveIndirectMemLong as usize] = move_indirect_mem_long;

    handlers[Opcode::MoveImmIndirectByte as usize] = move_imm_indirect_byte;
    handlers[Opcode::MoveImmIndirectShort as usize] = move_imm_indirect_short;
    handlers[Opcode::MoveImmIndirectLong as usize] = move_imm_indirect_long;

    handlers[Opcode::MoveRegIndirectByte as usize] = move_reg_indirect_byte;
    handlers[Opcode::MoveRegIndirectShort as usize] = move_reg_indirect_short;
    handlers[Opcode::MoveRegIndirectLong as usize] = move_reg_indirect_long;

    handlers[Opcode::MoveAbsIndirectByte as usize] = move_abs_indirect_byte;
    handlers[Opcode::MoveAbsIndirectShort as usize] = move_abs_indirect_short;
    handlers[Opcode::MoveAbsIndirectLong as usize] = move_abs_indirect_long;

    handlers[Opcode::MoveMemIndirectByte as usize] = move_mem_indirect_byte;
    handlers[Opcode::MoveMemIndirectShort as usize] = move_mem_indirect_short;
    handlers[Opcode::MoveMemIndirectLong as usize] = move_mem_indirect_long;

    handlers[Opcode::MoveIndirectIndirectByte as usize] = move_indirect_indirect_byte;
    handlers[Opcode::MoveIndirectIndirectShort as usize] = move_indirect_indirect_short;
    handlers[Opcode::MoveIndirectIndirectLong as usize] = move_indirect_indirect_long;

    handlers[Opcode::AddByteImm as usize] = add_byte_imm;
    handlers[Opcode::AddShortImm as usize] = add_short_imm;
    handlers[Opcode::AddLongImm as usize] = add_long_imm;

    handlers[Opcode::AddByteReg as usize] = add_byte_reg;
    handlers[Opcode::AddShortReg as usize] = add_short_reg;
    handlers[Opcode::AddLongReg as usize] = add_long_reg;

    handlers[Opcode::AddCarryByteImm as usize] = add_carry_byte_imm;
    handlers[Opcode::AddCarryShortImm as usize] = add_carry_short_imm;
    handlers[Opcode::AddCarryLongImm as usize] = add_carry_long_imm;

    handlers[Opcode::AddCarryByteReg as usize] = add_carry_byte_reg;
    handlers[Opcode::AddCarryShortReg as usize] = add_carry_short_reg;
    handlers[Opcode::AddCarryLongReg as usize] = add_carry_long_reg;

    handlers[Opcode::SubByteImm as usize] = sub_byte_imm;
    handlers[Opcode::SubShortImm as usize] = sub_short_imm;
    handlers[Opcode::SubLongImm as usize] = sub_long_imm;

    handlers[Opcode::SubByteReg as usize] = sub_byte_reg;
    handlers[Opcode::SubShortReg as usize] = sub_short_reg;
    handlers[Opcode::SubLongReg as usize] = sub_long_reg;

    handlers[Opcode::SubBorrowByteImm as usize] = sub_borrow_byte_imm;
    handlers[Opcode::SubBorrowShortImm as usize] = sub_borrow_short_imm;
    handlers[Opcode::SubBorrowLongImm as usize] = sub_borrow_long_imm;

    handlers[Opcode::SubBorrowByteReg as usize] = sub_borrow_byte_reg;
    handlers[Opcode::SubBorrowShortReg as usize] = sub_borrow_short_reg;
    handlers[Opcode::SubBorrowLongReg as usize] = sub_borrow_long_reg;

    handlers[Opcode::MulByteImm as usize] = mul_byte_imm;
    handlers[Opcode::MulShortImm as usize] = mul_short_imm;
    handlers[Opcode::MulLongImm as usize] = mul_long_imm;

    handlers[Opcode::MulByteReg as usize] = mul_byte_reg;
    handlers[Opcode::MulShortReg as usize] = mul_short_reg;
    handlers[Opcode::MulLongReg as usize] = mul_long_reg;

    handlers[Opcode::DivByteImm as usize] = div_byte_imm;
    handlers[Opcode::DivShortImm as usize] = div_short_imm;
    handlers[Opcode::DivLongImm as usize] = div_long_imm;

    handlers[Opcode::DivByteReg as usize] = div_byte_reg;
    handlers[Opcode::DivShortReg as usize] = div_short_reg;
    handlers[Opcode::DivLongReg as usize] = div_long_reg;

    handlers[Opcode::SignedMulByteImm as usize] = signed_mul_byte_imm;
    handlers[Opcode::SignedMulShortImm as usize] = signed_mul_short_imm;
    handlers[Opcode::SignedMulLongImm as usize] = signed_mul_long_imm;

    handlers[Opcode::SignedMulByteReg as usize] = signed_mul_byte_reg;
    handlers[Opcode::SignedMulShortReg as usize] = signed_mul_short_reg;
    handlers[Opcode::SignedMulLongReg as usize] = signed_mul_long_reg;

    handlers[Opcode::SignedDivByteImm as usize] = signed_div_byte_imm;
    handlers[Opcode::SignedDivShortImm as usize] = signed_div_short_imm;
    handlers[Opcode::SignedDivLongImm as usize] = signed_div_long_imm;

    handlers[Opcode::SignedDivByteReg as usize] = signed_div_byte_reg;
    handlers[Opcode::SignedDivShortReg as usize] = signed_div_short_reg;
    handlers[Opcode::SignedDivLongReg as usize] = signed_div_long_reg;

    handlers[Opcode::AndByteImm as usize] = and_byte_imm;
    handlers[Opcode::AndShortImm as usize] = and_short_imm;
    handlers[Opcode::AndLongImm as usize] = and_long_imm;

    handlers[Opcode::AndByteReg as usize] = and_byte_reg;
    handlers[Opcode::AndShortReg as usize] = and_short_reg;
    handlers[Opcode::AndLongReg as usize] = and_long_reg;

    handlers[Opcode::OrByteImm as usize] = or_byte_imm;
    handlers[Opcode::OrShortImm as usize] = or_short_imm;
    handlers[Opcode::OrLongImm as usize] = or_long_imm;

    handlers[Opcode::OrByteReg as usize] = or_byte_reg;
    handlers[Opcode::OrShortReg as usize] = or_short_reg;
    handlers[Opcode::OrLongReg as usize] = or_long_reg;

    handlers[Opcode::XorByteImm as usize] = xor_byte_imm;
    handlers[Opcode::XorShortImm as usize] = xor_short_imm;
    handlers[Opcode::XorLongImm as usize] = xor_long_imm;

    handlers[Opcode::XorByteReg as usize] = xor_byte_reg;
    handlers[Opcode::XorShortReg as usize] = xor_short_reg;
    handlers[Opcode::XorLongReg as usize] = xor_long_reg;

    handlers[Opcode::PushByteImm as usize] = push_byte_imm;
    handlers[Opcode::PushShortImm as usize] = push_short_imm;
    handlers[Opcode::PushLongImm as usize] = push_long_imm;

    handlers[Opcode::PushByteReg as usize] = push_byte_reg;
    handlers[Opcode::PushShortReg as usize] = push_short_reg;
    handlers[Opcode::PushLongReg as usize] = push_long_reg;

    handlers[Opcode::CompareByteImm as usize] = compare_byte_imm;
    handlers[Opcode::CompareShortImm as usize] = compare_short_imm;
    handlers[Opcode::CompareLongImm as usize] = compare_long_imm;

    handlers[Opcode::CompareByteReg as usize] = compare_byte_reg;
    handlers[Opcode::CompareShortReg as usize] = compare_short_reg;
    handlers[Opcode::CompareLongReg as usize] = compare_long_reg;

    handlers[Opcode::LogShiftLeftByteImm as usize] = log_shift_left_byte_imm;
    handlers[Opcode::LogShiftLeftShortImm as usize] = log_shift_left_short_imm;
    handlers[Opcode::LogShiftLeftLongImm as usize] = log_shift_left_long_imm;

    handlers[Opcode::LogShiftLeftByteReg as usize] = log_shift_left_byte_reg;
    handlers[Opcode::LogShiftLeftShortReg as usize] = log_shift_left_short_reg;
    handlers[Opcode::LogShiftLeftLongReg as usize] = log_shift_left_long_reg;

    handlers[Opcode::LogShiftRightByteImm as usize] = log_shift_right_byte_imm;
    handlers[Opcode::LogShiftRightShortImm as usize] = log_shift_right_short_imm;
    handlers[Opcode::LogShiftRightLongImm as usize] = log_shift_right_long_imm;

    handlers[Opcode::LogShiftRightByteReg as usize] = log_shift_right_byte_reg;
    handlers[Opcode::LogShiftRightShortReg as usize] = log_shift_right_short_reg;
    handlers[Opcode::LogShiftRightLongReg as usize] = log_shift_right_long_reg;

    handlers[Opcode::ArithShiftLeftByteImm as usize] = arith_shift_left_byte_imm;
    handlers[Opcode::ArithShiftLeftShortImm as usize] = arith_shift_left_short_imm;
    handlers[Opcode::ArithShiftLeftLongImm as usize] = arith_shift_left_long_imm;

    handlers[Opcode::ArithShiftLeftByteReg as usize] = arith_shift_left_byte_reg;
    handlers[Opcode::ArithShiftLeftShortReg as usize] = arith_shift_left_short_reg;
    handlers[Opcode::ArithShiftLeftLongReg as usize] = arith_shift_left_long_reg;

    handlers[Opcode::ArithShiftRightByteImm as usize] = arith_shift_right_byte_imm;
    handlers[Opcode::ArithShiftRightShortImm as usize] = arith_shift_right_short_imm;
    handlers[Opcode::ArithShiftRightLongImm as usize] = arith_shift_right_long_imm;

    handlers[Opcode::ArithShiftRightByteReg as usize] = arith_shift_right_byte_reg;
    handlers[Opcode::ArithShiftRightShortReg as usize] = arith_shift_right_short_reg;
    handlers[Opcode::ArithShiftRightLongReg as usize] = arith_shift_right_long_reg;

    handlers[Opcode::RotateLeftByteImm as usize] = rotate_left_byte_imm;
    handlers[Opcode::RotateLeftShortImm as usize] = rotate_left_short_imm;
    handlers[Opcode::RotateLeftLongImm as usize] = rotate_left_long_imm;

    handlers[Opcode::RotateLeftByteReg as usize] = rotate_left_byte_reg;
    handlers[Opcode::RotateLeftShortReg as usize] = rotate_left_short_reg;
    handlers[Opcode::RotateLeftLongReg as usize] = rotate_left_long_reg;

    handlers[Opcode::RotateRightByteImm as usize] = rotate_right_byte_imm;
    handlers[Opcode::RotateRightShortImm as usize] = rotate_right_short_imm;
    handlers[Opcode::RotateRightLongImm as usize] = rotate_right_long_imm;

    handlers[Opcode::RotateRightByteReg as usize] = rotate_right_byte_reg;
    handlers[Opcode::RotateRightShortReg as usize] = rotate_right_short_reg;
    handlers[Opcode::RotateRightLongReg as usize] = rotate_right_long_reg;

    handlers[Opcode::PopByte as usize] = pop_byte;
    handlers[Opcode::PopShort as usize] = pop_short;
    handlers[Opcode::PopLong as usize] = pop_long;

    handlers[Opcode::NegateByte as usize] = negate_byte;
    handlers[Opcode::NegateShort as usize] = negate_short;
    handlers[Opcode::NegateLong as usize] = negate_long;

    handlers[Opcode::NotByte as usize] = not_byte;
    handlers[Opcode::NotShort as usize] = not_short;
    handlers[Opcode::NotLong as usize] = not_long;

    handlers[Opcode::IncrementByte as usize] = increment_byte;
    handlers[Opcode::IncrementShort as usize] = increment_short;
    handlers[Opcode::IncrementLong as usize] = increment_long;

    handlers[Opcode::DecrementByte as usize] = decrement_byte;
    handlers[Opcode::DecrementShort as usize] = decrement_short;
    handlers[Opcode::DecrementLong as usize] = decrement_long;

    handlers[Opcode::JumpEqual as usize] = jump_equal;
    handlers[Opcode::JumpNotEqual as usize] = jump_not_equal;
    handlers[Opcode::JumpGreater as usize] = jump_greater;
    handlers[Opcode::JumpGreaterEqual as usize] = jump_greater_equal;
    handlers[Opcode::JumpLess as usize] = jump_less;
    handlers[Opcode::JumpLessEqual as usize] = jump_less_equal;
    handlers[Opcode::JumpSignedGreater as usize] = jump_signed_greater;
    handlers[Opcode::JumpSignedGreaterEqual as usize] = jump_signed_greater_equal;
    handlers[Opcode::JumpSignedLess as usize] = jump_signed_less;
    handlers[Opcode::JumpSignedLessEqual as usize] = jump_signed_less_equal;
    handlers[Opcode::JumpImm as usize] = jump_imm;
    handlers[Opcode::JumpReg as usize] = jump_reg;

    handlers[Opcode::Interrupt as usize] = interrupt;
    handlers[Opcode::InterruptReturn as usize] = interrupt_return;

    handlers[Opcode::Call as usize] = call;
    handlers[Opcode::Return as usize] = ret;
    handlers[Opcode::Syscall as usize] = syscall;

    handlers[Opcode::ClearCarry as usize] = clear_carry;
    handlers[Opcode::Nop as usize] = nop;

    assert!(handlers.len() == 256);

    handlers
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


// General, Cycle, Load Program[OpcodeHandler; 256];
impl Cpu {
    const OPCODE_HANDLERS: OpcodeHandlerArray = get_opcode_handlers();

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

    pub fn load_program_from_file<T: AsRef<Path> + ?Sized>(
        &mut self,
        file_path: &T,
    ) -> std::io::Result<()> {
        let mut file = std::fs::File::open(file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        self.load_program(&buffer);
        Ok(())
    }

    #[inline(always)]
    pub fn cycle(&mut self) {
        let flags = unsafe { self.registers.get_unchecked(FLAGS) };

        if flags & Cpu::INTERRUPT_FLAG == Cpu::INTERRUPT_FLAG {
            if let Some(hw_interrupt) = self.hardware_interrupt.take() {
                (hw_interrupt)(self);
                self.hardware_interrupt = None;
            }
        }
        let instruction = self.next_byte();
        unsafe { (Self::OPCODE_HANDLERS.get_unchecked(instruction as usize))(self) };
    }
}

// Memory utils
impl Cpu {
    #[inline(always)]
    pub fn next_byte(&mut self) -> u8 {
        let b = unsafe { *self.memory.buffer.get_unchecked(self.ip() as usize) };
        self.inc_ip(1);
        b
    }
    
    #[inline(always)]
    pub fn next_short(&mut self) -> u16 {
        let ip = self.ip() as usize;
        let low = unsafe { *self.memory.buffer.get_unchecked(ip) as u16 };
        let high = unsafe { *self.memory.buffer.get_unchecked(ip + 1) as u16 };
        self.inc_ip(2);
        (high << 8) | low
    }
    
    #[inline(always)]
    pub fn next_long(&mut self) -> u32 {
        let ip = self.ip() as usize;
        let low = unsafe { *self.memory.buffer.get_unchecked(ip) as u32 };
        let mid = unsafe { *self.memory.buffer.get_unchecked(ip + 1) as u32 };
        let high = unsafe { *self.memory.buffer.get_unchecked(ip + 2) as u32 };
        let top = unsafe { *self.memory.buffer.get_unchecked(ip + 3) as u32 };
        self.inc_ip(4);
        (top << 24) | (high << 16) | (mid << 8) | low
    }
    
    #[inline(always)]
    pub fn next_utf8(&mut self) -> String {
        let string = self.memory.utf8(self.ip()).unwrap();
        self.inc_ip(string.len() as u32);
        return string;
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
            hardware_interrupt: None,
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
    #[inline(always)]
    pub fn flags(&self) -> u32 {
        unsafe { *self.registers.get_unchecked(FLAGS) }
    }
    #[inline(always)]
    pub fn has_flag(&self, flag: u32) -> bool {
        (self.flags() & flag) == flag
    }
    #[inline(always)]
    pub fn set_flag(&mut self, flag: u32, set: bool) {
        unsafe {
            let flags = self.registers.get_unchecked_mut(FLAGS);
            *flags = (*flags & !flag) | (-(set as i32) as u32 & flag);
        }
    }
    #[inline(always)]
    pub fn sp(&self) -> usize {
        unsafe { *self.registers.get_unchecked(SP) as usize }
    }
    #[inline(always)]
    pub fn ip(&self) -> usize {
        unsafe { *self.registers.get_unchecked(IP) as usize }
    }
    #[inline(always)]
    pub fn bp(&self) -> usize {
        unsafe { *self.registers.get_unchecked(BP) as usize }
    }
    #[inline(always)]
    pub fn dec_sp(&mut self, value: u32) {
        unsafe { *self.registers.get_unchecked_mut(SP) -= value; }
    }
    #[inline(always)]
    pub fn inc_sp(&mut self, value: u32) {
        unsafe { *self.registers.get_unchecked_mut(SP) += value; } 
    }
    #[inline(always)]
    pub fn inc_ip(&mut self, value: u32) {
        unsafe { *self.registers.get_unchecked_mut(IP) += value; }
    }
}

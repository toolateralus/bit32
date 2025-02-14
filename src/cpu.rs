use crate::functions::{self, log_opcode};
use crate::handlers::*;
use crate::opcodes::Opcode;
use core::fmt;
use std::fmt::Debug;
use std::io::Read;
use std::ops::{Neg, Not, Shl, Shr};
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
    pub opcode_handlers: [OpcodeHandler; Opcode::Nop as usize],
}

pub fn get_opcode_handlers() -> [OpcodeHandler; Opcode::Nop as usize] {
    let handlers = [
        hlt,
        
        move_imm_reg_byte,
        move_imm_reg_short,
        move_imm_reg_long,

        move_reg_reg_byte,
        move_reg_reg_short,
        move_reg_reg_long,

        move_mem_reg_byte,
        move_mem_reg_short,
        move_mem_reg_long,

        move_abs_reg_byte,
        move_abs_reg_short,
        move_abs_reg_long,

        move_indirect_reg_byte,
        move_indirect_reg_short,
        move_indirect_reg_long,

        move_imm_abs_byte,
        move_imm_abs_short,
        move_imm_abs_long,

        move_reg_abs_byte,
        move_reg_abs_short,
        move_reg_abs_long,

        move_abs_abs_byte,
        move_abs_abs_short,
        move_abs_abs_long,
        
        move_mem_abs_byte,
        move_mem_abs_short,
        move_mem_abs_long,

        move_indirect_abs_byte,
        move_indirect_abs_short,
        move_indirect_abs_long,

        move_imm_mem_byte,
        move_imm_mem_short,
        move_imm_mem_long,

        move_reg_mem_byte,
        move_reg_mem_short,
        move_reg_mem_long,

        move_mem_mem_byte,
        move_mem_mem_short,
        move_mem_mem_long,

        move_abs_mem_byte,
        move_abs_mem_short,
        move_abs_mem_long,

        move_indirect_mem_byte,
        move_indirect_mem_short,
        move_indirect_mem_long,

        move_imm_indirect_byte,
        move_imm_indirect_short,
        move_imm_indirect_long,
        
        move_reg_indirect_byte,
        move_reg_indirect_short,
        move_reg_indirect_long,
        
        move_abs_indirect_byte,
        move_abs_indirect_short,
        move_abs_indirect_long,
        
        move_mem_indirect_byte,
        move_mem_indirect_short,
        move_mem_indirect_long,

        move_indirect_indirect_byte,
        move_indirect_indirect_short,
        move_indirect_indirect_long,

        add_byte_imm,
        add_short_imm,
        add_long_imm,

        add_byte_reg,
        add_short_reg,
        add_long_reg,

        add_carry_byte_imm,
        add_carry_short_imm,
        add_carry_long_imm,

        add_carry_byte_reg,
        add_carry_short_reg,
        add_carry_long_reg,

        sub_byte_imm,
        sub_short_imm,
        sub_long_imm,

        sub_byte_reg,
        sub_short_reg,
        sub_long_reg,

        sub_borrow_byte_imm,
        sub_borrow_short_imm,
        sub_borrow_long_imm,

        sub_borrow_byte_reg,
        sub_borrow_short_reg,
        sub_borrow_long_reg,
        
        mul_byte_imm,
        mul_short_imm,
        mul_long_imm,

        mul_byte_reg,
        mul_short_reg,
        mul_long_reg,

        div_byte_imm,
        div_short_imm,
        div_long_imm,

        div_byte_reg,
        div_short_reg,
        div_long_reg,

        signed_mul_byte_imm,
        signed_mul_short_imm,
        signed_mul_long_imm,

        signed_mul_byte_reg,
        signed_mul_short_reg,
        signed_mul_long_reg,

        signed_div_byte_imm,
        signed_div_short_imm,
        signed_div_long_imm,

        signed_div_byte_reg,
        signed_div_short_reg,
        signed_div_long_reg,

        and_byte_imm,
        and_short_imm,
        and_long_imm,

        and_byte_reg,
        and_short_reg,
        and_long_reg,

        or_byte_imm,
        or_short_imm,
        or_long_imm,

        or_byte_reg,
        or_short_reg,
        or_long_reg,

        xor_byte_imm,
        xor_short_imm,
        xor_long_imm,

        xor_byte_reg,
        xor_short_reg,
        xor_long_reg,

        push_byte_imm,
        push_short_imm,
        push_long_imm,

        push_byte_reg,
        push_short_reg,
        push_long_reg,

        compare_byte_imm,
        compare_short_imm,
        compare_long_imm,

        compare_byte_reg,
        compare_short_reg,
        compare_long_reg,

        log_shift_left_byte_imm,
        log_shift_left_short_imm,
        log_shift_left_long_imm,

        log_shift_left_byte_reg,
        log_shift_left_short_reg,
        log_shift_left_long_reg,

        log_shift_right_byte_imm,
        log_shift_right_short_imm,
        log_shift_right_long_imm,

        log_shift_right_byte_reg,
        log_shift_right_short_reg,
        log_shift_right_long_reg,

        arith_shift_left_byte_imm,
        arith_shift_left_short_imm,
        arith_shift_left_long_imm,

        arith_shift_left_byte_reg,
        arith_shift_left_short_reg,
        arith_shift_left_long_reg,

        arith_shift_right_byte_imm,
        arith_shift_right_short_imm,
        arith_shift_right_long_imm,

        arith_shift_right_byte_reg,
        arith_shift_right_short_reg,
        arith_shift_right_long_reg,

        rotate_left_byte_imm,
        rotate_left_short_imm,
        rotate_left_long_imm,

        rotate_left_byte_reg,
        rotate_left_short_reg,
        rotate_left_long_reg,

        rotate_right_byte_imm,
        rotate_right_short_imm,
        rotate_right_long_imm,

        rotate_right_byte_reg,
        rotate_right_short_reg,
        rotate_right_long_reg,

        pop_byte,
        pop_short,
        pop_long,

        negate_byte,
        negate_short,
        negate_long,

        not_byte,
        not_short,
        not_long,

        increment_byte,
        increment_short,
        increment_long,

        decrement_byte,
        decrement_short,
        decrement_long,

        jump_equal,
        jump_not_equal,
        jump_greater,
        jump_greater_equal,
        jump_less,
        jump_less_equal,
        jump_signed_greater,
        jump_signed_greaterequal,
        jump_signed_less,
        jump_signed_lessequal,
        jump_imm,
        jump_reg,

        interrupt,
        interrupt_return,

        call,
        ret,
        syscall,
    
        clearcarry,
        nop,
    ];

    todo!();
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

    pub fn cycle(&mut self) {
        let flags = self.registers[FLAGS];

        // Extract the function pointer to avoid borrowing issues
        if flags & Cpu::INTERRUPT_FLAG == Cpu::INTERRUPT_FLAG {
            if let Some(hw_interrupt) = self.hardware_interrupt_routine.take() {
                (hw_interrupt)(self);
                self.hardware_interrupt_routine = None;
            }
        }

        let instruction = self.next_byte();
        let opcode = Opcode::from(instruction);

        if true {
            log_opcode(&opcode);
        }

        match opcode {
            Opcode::Interrupt => {
                let busy_in_interrupt = (self.registers[FLAGS] & Cpu::INTERRUPT_FLAG as u32) != 0;

                // we block interrupts while handling an interrupt.
                // in a more complicated emulator, you wouldn't have this
                // loss of data, but it's complicated and we don't do insane
                // rewinding and reordering of instructions.
                if busy_in_interrupt {
                    return;
                }

                let irq = self.next_byte() as u32;

                // get the base of the idt
                let idt_base = self.registers[IDT] as u32;

                // idt entries are exactly 4 bytes long
                let isr_addr = idt_base + (irq * 4);

                // push return address
                let return_address = self.ip();
                self.dec_sp(4);
                self.memory.set_long(self.sp(), return_address as u32);

                // Debugging: Print the return address and stack pointer
                println!(
                    "Interrupt: Pushed return address {:08x} to stack pointer {:08x}",
                    return_address,
                    self.sp()
                );

                // set the interrupt flag
                self.registers[FLAGS] |= Cpu::INTERRUPT_FLAG as u32;

                // jump to the interrupt service routine
                self.registers[IP] = self.memory.long(isr_addr as usize);
            }
            Opcode::InterruptReturn => {
                // clear the interrupt flag
                self.registers[FLAGS] &= !(Cpu::INTERRUPT_FLAG as u32);

                // pop return address
                let ret_addr = self.memory.long(self.sp());

                // Debugging: Print the return address and stack pointer
                println!(
                    "InterruptReturn: Popped return address {:08x} from stack pointer {:08x}",
                    ret_addr,
                    self.sp()
                );

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

            Opcode::Syscall => {
                let idx = self.next_byte() as usize;

                match idx {
                    0 => functions::log_memory(self),
                    1 => functions::log(self),
                    2 => functions::print_string(self),
                    3 => functions::print_register(self),
                    _ => panic!("invalid rust function: {}", idx)
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

            Opcode::LogShiftLeftByteImm => {
                let val = self.next_byte();
                self.registers[0] = (self.registers[0] as u8).shl(val) as u32;
            }
            Opcode::LogShiftLeftShortImm => {
                let val = self.next_byte();
                self.registers[0] = (self.registers[0] as u16).shl(val) as u32;
            }
            Opcode::LogShiftLeftLongImm => {
                let val = self.next_byte();
                self.registers[0] = (self.registers[0] as u32).shl(val) as u32;
            }

            Opcode::LogShiftRightByteImm => {
                let val = self.next_byte();
                self.registers[0] = (self.registers[0] as u8).shr(val) as u32;
            }
            Opcode::LogShiftRightShortImm => {
                let val = self.next_byte();
                self.registers[0] = (self.registers[0] as u16).shr(val) as u32;
            }
            Opcode::LogShiftRightLongImm => {
                let val = self.next_byte();
                self.registers[0] = (self.registers[0] as u32).shr(val) as u32;
            }

            Opcode::ArithShiftLeftByteImm => {
                let val = self.next_byte();
                self.registers[0] = (self.registers[0] as u8).shl(val) as u32;
            }
            Opcode::ArithShiftLeftShortImm => {
                let val = self.next_byte();
                self.registers[0] = (self.registers[0] as u16).shl(val) as u32;
            }
            Opcode::ArithShiftLeftLongImm => {
                let val = self.next_byte();
                self.registers[0] = (self.registers[0] as u32).shl(val) as u32;
            }

            Opcode::ArithShiftRightByteImm => {
                let val = self.next_byte();
                self.registers[0] = (self.registers[0] as u8).shr(val) as u32;
            }
            Opcode::ArithShiftRightShortImm => {
                let val = self.next_byte();
                self.registers[0] = (self.registers[0] as u16).shr(val) as u32;
            }
            Opcode::ArithShiftRightLongImm => {
                let val = self.next_byte();
                self.registers[0] = (self.registers[0] as u32).shr(val) as u32;
            }

            Opcode::RotateLeftByteImm => {
                let val = self.next_byte();
                self.registers[0] = (self.registers[0] as u8).rotate_left(val as u32) as u32;
            }
            Opcode::RotateLeftShortImm => {
                let val = self.next_byte();
                self.registers[0] = (self.registers[0] as u16).rotate_left(val as u32) as u32;
            }
            Opcode::RotateLeftLongImm => {
                let val = self.next_byte();
                self.registers[0] = (self.registers[0] as u32).rotate_left(val as u32) as u32;
            }

            Opcode::RotateRightByteImm => {
                let val = self.next_byte();
                self.registers[0] = (self.registers[0] as u8).rotate_right(val as u32) as u32;
            }
            Opcode::RotateRightShortImm => {
                let val = self.next_byte();
                self.registers[0] = (self.registers[0] as u16).rotate_right(val as u32) as u32;
            }
            Opcode::RotateRightLongImm => {
                let val = self.next_byte();
                self.registers[0] = (self.registers[0] as u32).rotate_right(val as u32) as u32;
            }

            Opcode::LogShiftLeftByteReg => {
                let reg = self.next_byte() as usize;
                let val = self.registers[reg] as u8;
                self.registers[0] = (self.registers[0] as u8).shl(val) as u32;
            }
            Opcode::LogShiftLeftShortReg => {
                let reg = self.next_byte() as usize;
                let val = self.registers[reg] as u16;
                self.registers[0] = (self.registers[0] as u16).shl(val) as u32;
            }
            Opcode::LogShiftLeftLongReg => {
                let reg = self.next_byte() as usize;
                let val = self.registers[reg] as u32;
                self.registers[0] = (self.registers[0] as u32).shl(val) as u32;
            }

            Opcode::LogShiftRightByteReg => {
                let reg = self.next_byte() as usize;
                let val = self.registers[reg] as u8;
                self.registers[0] = (self.registers[0] as u8).shr(val) as u32;
            }
            Opcode::LogShiftRightShortReg => {
                let reg = self.next_byte() as usize;
                let val = self.registers[reg] as u16;
                self.registers[0] = (self.registers[0] as u16).shr(val) as u32;
            }
            Opcode::LogShiftRightLongReg => {
                let reg = self.next_byte() as usize;
                let val = self.registers[reg] as u32;
                self.registers[0] = (self.registers[0] as u32).shr(val) as u32;
            }

            Opcode::ArithShiftLeftByteReg => {
                let reg = self.next_byte() as usize;
                let val = self.registers[reg] as u8;
                self.registers[0] = (self.registers[0] as i8).shl(val) as u32;
            }
            Opcode::ArithShiftLeftShortReg => {
                let reg = self.next_byte() as usize;
                let val = self.registers[reg] as u16;
                self.registers[0] = (self.registers[0] as i16).shl(val) as u32;
            }
            Opcode::ArithShiftLeftLongReg => {
                let reg = self.next_byte() as usize;
                let val = self.registers[reg] as u32;
                self.registers[0] = (self.registers[0] as i32).shl(val) as u32;
            }

            Opcode::ArithShiftRightByteReg => {
                let reg = self.next_byte() as usize;
                let val = self.registers[reg] as u8;
                self.registers[0] = (self.registers[0] as i8).shr(val) as u32;
            }
            Opcode::ArithShiftRightShortReg => {
                let reg = self.next_byte() as usize;
                let val = self.registers[reg] as u16;
                self.registers[0] = (self.registers[0] as i16).shr(val) as u32;
            }
            Opcode::ArithShiftRightLongReg => {
                let reg = self.next_byte() as usize;
                let val = self.registers[reg] as u32;
                self.registers[0] = (self.registers[0] as i32).shr(val) as u32;
            }

            Opcode::RotateLeftByteReg => {
                let reg = self.next_byte() as usize;
                let val = self.registers[reg] as u32;
                self.registers[0] = (self.registers[0] as u8).rotate_left(val) as u32;
            }
            Opcode::RotateLeftShortReg => {
                let reg = self.next_byte() as usize;
                let val = self.registers[reg] as u32;
                self.registers[0] = (self.registers[0] as u16).rotate_left(val) as u32;
            }
            Opcode::RotateLeftLongReg => {
                let reg = self.next_byte() as usize;
                let val = self.registers[reg] as u32;
                self.registers[0] = (self.registers[0] as u32).rotate_left(val) as u32;
            }

            Opcode::RotateRightByteReg => {
                let reg = self.next_byte() as usize;
                let val = self.registers[reg] as u32;
                self.registers[0] = (self.registers[0] as u8).rotate_right(val) as u32;
            }
            Opcode::RotateRightShortReg => {
                let reg = self.next_byte() as usize;
                let val = self.registers[reg] as u32;
                self.registers[0] = (self.registers[0] as u16).rotate_right(val) as u32;
            }
            Opcode::RotateRightLongReg => {
                let reg = self.next_byte() as usize;
                let val = self.registers[reg] as u32;
                self.registers[0] = (self.registers[0] as u32).rotate_right(val) as u32;
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
            opcode_handlers: get_opcode_handlers(),
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
        self.registers[FLAGS] as u32
    }
    #[inline(always)]
    pub fn has_flag(&self, flag: u32) -> bool {
        (self.flags() & flag) == flag
    }
    #[inline(always)]
    pub fn set_flag(&mut self, flag: u32, set: bool) {
        self.registers[FLAGS] = (self.registers[FLAGS] & !flag) | (-(set as i32) as u32 & flag);
    }
    #[inline(always)]
    pub fn sp(&self) -> usize {
        self.registers[SP] as usize
    }
    #[inline(always)]
    pub fn ip(&self) -> usize {
        self.registers[IP] as usize
    }
    #[inline(always)]
    pub fn bp(&self) -> usize {
        self.registers[BP] as usize
    }
    #[inline(always)]
    fn dec_sp(&mut self, value: u32) {
        if let Some(result) = self.registers[SP].checked_sub(value) {
            self.registers[SP] = result;
        } else {
            println!("{:?}", self);
            panic!("stack underflow.");
        }
    }
    #[inline(always)]
    fn inc_sp(&mut self, value: u32) {
        if let Some(result) = self.registers[SP].checked_add(value) {
            self.registers[SP] = result;
        } else {
            println!("{:?}", self);
            panic!("stack overflow.");
        }
    }
    #[inline(always)]
    fn inc_ip(&mut self, value: u32) {
        if let Some(result) = self.registers[IP].checked_add(value) {
            self.registers[IP] = result;
        } else {
            println!("{:?}", self);
            panic!("instruction pointer overflow.");
        }
    }
}
// Registers
impl Cpu {
    // TODO: probably remove these, it's a dang cpu emulator why would we have bounds checking.
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


// Comparisons
impl Cpu {
    fn cmp(&mut cpu, opcode: &Opcode) {
        match opcode {
            Opcode::CompareByteImm => {
                let lhs = cpu.registers[0];
                let rhs = cpu.next_byte();
                cpu.registers[0] = if lhs as u8 == rhs { 1 } else { 0 };
            }
            Opcode::CompareShortImm => {
                let lhs = cpu.registers[0];
                let rhs = cpu.next_short();
                cpu.registers[0] = if lhs as u16 == rhs { 1 } else { 0 };
            }
            Opcode::CompareLongImm => {
                let lhs = cpu.registers[0];
                let rhs = cpu.next_long();
                cpu.registers[0] = if lhs == rhs { 1 } else { 0 };
            }
            Opcode::CompareByteReg => {
                let lhs = cpu.registers[0];
                let index = cpu.next_byte() as usize;
                let rhs = cpu.registers[index] as u8;
                cpu.registers[0] = if lhs as u8 == rhs { 1 } else { 0 };
            }
            Opcode::CompareShortReg => {
                let lhs = cpu.registers[0];
                let index = cpu.next_byte() as usize;
                let rhs = cpu.registers[index] as u16;
                cpu.registers[0] = if lhs as u16 == rhs { 1 } else { 0 };
            }
            Opcode::CompareLongReg => {
                let lhs = cpu.registers[0];
                let index = cpu.next_byte() as usize;
                let rhs = cpu.registers[index];
                cpu.registers[0] = if lhs == rhs { 1 } else { 0 };
            }
            _ => panic!("invalid compare"),
        }
    }
    pub fn and(&mut cpu, op: &Opcode) {
        match op {
            Opcode::AndByteImm => {
                let lhs = (cpu.registers[0] & 0xFF) as u8;
                let rhs = cpu.next_byte();
                cpu.registers[0] = (lhs & rhs) as u32;
            }
            Opcode::AndShortImm => {
                let lhs = (cpu.registers[0] & 0xFFFF) as u16;
                let rhs = cpu.next_short();
                cpu.registers[0] = (lhs & rhs) as u32;
            }
            Opcode::AndLongImm => {
                let lhs = cpu.registers[0];
                let rhs = cpu.next_long();
                cpu.registers[0] = lhs & rhs;
            }
            Opcode::AndByteReg => {
                let lhs = (cpu.registers[0] & 0xFF) as u8;
                let index = cpu.next_byte() as usize;
                let rhs = (cpu.registers[index] & 0xFF) as u8;
                cpu.registers[0] = (lhs & rhs) as u32;
            }
            Opcode::AndShortReg => {
                let lhs = (cpu.registers[0] & 0xFFFF) as u16;
                let index = cpu.next_byte() as usize;
                let rhs = (cpu.registers[index] & 0xFFFF) as u16;
                cpu.registers[0] = (lhs & rhs) as u32;
            }
            Opcode::AndLongReg => {
                let lhs = cpu.registers[0];
                let index = cpu.next_byte() as usize;
                let rhs = cpu.registers[index];
                cpu.registers[0] = lhs & rhs;
            }
            _ => {
                panic!("invalid and instruction");
            }
        }
    }
}

// Jumps
impl Cpu {
    fn jump_cmp(&mut cpu, op: &Opcode) {
        let lhs = cpu.registers[0];
        let rhs = cpu.registers[1];
        let addr = cpu.next_long();
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
            cpu.registers[IP] = addr;
        }
    }
    fn jmp_imm(&mut cpu) {
        let addr = cpu.next_long();
        cpu.registers[IP] = addr;
    }
    fn jmp_reg(&mut cpu) {
        let index = cpu.next_byte() as usize;
        let addr = cpu.registers[index];
        cpu.registers[IP] = addr;
    }
}

// Stack
impl Cpu {
    fn push(&mut cpu, op: &Opcode) {
        match op {
            Opcode::PushByteReg => {
                cpu.dec_sp(1);
                let index = cpu.next_byte() as usize;
                let value = (cpu.registers[index] & 0xFF) as u8;
                cpu.memory.set_byte(cpu.sp(), value);
            }
            Opcode::PushByteImm => {
                cpu.dec_sp(1);
                let value = cpu.next_byte();
                cpu.memory.set_byte(cpu.sp(), value);
            }
            Opcode::PushShortImm => {
                cpu.dec_sp(2);
                let value = cpu.next_short();
                cpu.memory.set_short(cpu.sp(), value);
            }
            Opcode::PushShortReg => {
                cpu.dec_sp(2);
                let index = cpu.next_byte() as usize;
                let value = (cpu.registers[index] & 0xFFFF) as u16;
                cpu.memory.set_short(cpu.sp(), value);
            }
            Opcode::PushLongReg => {
                cpu.dec_sp(4);
                let index = cpu.next_byte() as usize;
                let value = cpu.registers[index];
                cpu.memory.set_long(cpu.sp(), value);
            }
            Opcode::PushLongImm => {
                cpu.dec_sp(4);
                let value = cpu.next_long();
                cpu.memory.set_long(cpu.sp(), value);
            }
            _ => {
                panic!("invalid push");
            }
        }
    }

    fn pop(&mut cpu, op: &Opcode) {
        match op {
            Opcode::PopByte => {
                let dest = cpu.next_byte() as usize;
                let value = cpu.memory.byte(cpu.sp());
                cpu.registers[dest] = value as u32;
                cpu.inc_sp(1);
            }
            Opcode::PopShort => {
                let dest = cpu.next_byte() as usize;
                let value = cpu.memory.short(cpu.sp());
                cpu.registers[dest] = value as u32;
                cpu.inc_sp(2);
            }
            Opcode::PopLong => {
                let dest = cpu.next_byte() as usize;
                let value = cpu.memory.long(cpu.sp());
                cpu.registers[dest] = value as u32;
                cpu.inc_sp(4);
            }
            _ => {
                panic!("invalid pop");
            }
        }
    }
}
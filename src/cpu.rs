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

pub const fn get_opcode_handlers() -> [OpcodeHandler; Opcode::Nop as usize + 1] {
    [
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
        jump_signed_greater_equal,
        jump_signed_less,
        jump_signed_less_equal,
        jump_imm,
        jump_reg,

        interrupt,
        interrupt_return,

        call,
        ret,
        syscall,
    
        clear_carry,
        nop,
    ]
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
    const OPCODE_HANDLERS: [OpcodeHandler; Opcode::Nop as usize + 1] = get_opcode_handlers();
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
            if let Some(hw_interrupt) = self.hardware_interrupt_routine.take() {
                (hw_interrupt)(self);
                self.hardware_interrupt_routine = None;
            }
        }
        let instruction = self.next_byte();
        Self::OPCODE_HANDLERS[instruction as usize](self);
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
        if let Some(result) = self.registers[SP].checked_sub(value) {
            self.registers[SP] = result;
        } else {
            println!("{:?}", self);
            panic!("stack underflow.");
        }
    }
    #[inline(always)]
    pub fn inc_sp(&mut self, value: u32) {
        if let Some(result) = self.registers[SP].checked_add(value) {
            self.registers[SP] = result;
        } else {
            println!("{:?}", self);
            panic!("stack overflow.");
        }
    }
    #[inline(always)]
    pub fn inc_ip(&mut self, value: u32) {
        if let Some(result) = self.registers[IP].checked_add(value) {
            self.registers[IP] = result;
        } else {
            println!("{:?}", self);
            panic!("instruction pointer overflow.");
        }
    }
}

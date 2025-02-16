use std::ops::{Neg, Not, Shl, Shr};

use crate::{
    cpu::{Cpu, FLAGS, IDT, IP},
    functions,
};

// pub fn hlt(cpu: &mut Cpu);

pub fn hlt(cpu: &mut Cpu) {
    cpu.set_flag(Cpu::HALT_FLAG, true);
    
    for hardware in cpu.hardware.iter() {
        let hw = hardware.clone();
        hw.borrow_mut().deinit();
    }
}

pub fn move_imm_reg_byte(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_val = cpu.next_byte();
    unsafe {
        *cpu.registers.get_unchecked_mut(dst_reg) = src_val as u32;
    }
}
pub fn move_imm_reg_short(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_val = cpu.next_short();
    unsafe {
        *cpu.registers.get_unchecked_mut(dst_reg) = src_val as u32;
    }
}
pub fn move_imm_reg_long(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_val = cpu.next_long();
    unsafe {
        *cpu.registers.get_unchecked_mut(dst_reg) = src_val;
    }
}

pub fn move_reg_reg_byte(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_reg = cpu.next_byte() as usize;
    unsafe {
        *cpu.registers.get_unchecked_mut(dst_reg) = cpu.registers.get_unchecked(src_reg) & 0xFF;
    }
}

pub fn move_reg_reg_short(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_reg = cpu.next_byte() as usize;
    unsafe {
        *cpu.registers.get_unchecked_mut(dst_reg) = cpu.registers.get_unchecked(src_reg) & 0xFFFF;
    }
}

pub fn move_reg_reg_long(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_reg = cpu.next_byte() as usize;
    unsafe {
        *cpu.registers.get_unchecked_mut(dst_reg) = *cpu.registers.get_unchecked(src_reg);
    }
}

pub fn move_mem_reg_byte(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_adr = cpu.next_long() as usize;
    let src_val = cpu.memory.byte(src_adr + cpu.ip()) as u32;
    cpu.registers[dst_reg] = src_val;
}
pub fn move_mem_reg_short(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_adr = cpu.next_long() as usize;
    let src_val = cpu.memory.short(src_adr + cpu.ip()) as u32;
    cpu.registers[dst_reg] = src_val;
}
pub fn move_mem_reg_long(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_adr = cpu.next_long() as usize;
    let src_val = cpu.memory.long(src_adr + cpu.ip());
    cpu.registers[dst_reg] = src_val;
}

pub fn move_abs_reg_byte(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_adr = cpu.next_long() as usize;
    cpu.registers[dst_reg] = cpu.memory.byte(src_adr) as u32;
}
pub fn move_abs_reg_short(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_adr = cpu.next_long() as usize;
    cpu.registers[dst_reg] = cpu.memory.short(src_adr) as u32;
}
pub fn move_abs_reg_long(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_adr = cpu.next_long() as usize;
    cpu.registers[dst_reg] = cpu.memory.long(src_adr);
}

pub fn move_indirect_reg_byte(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_reg = cpu.next_byte() as usize;
    let src_adr = cpu.registers[src_reg] as usize;
    cpu.registers[dst_reg] = cpu.memory.byte(src_adr) as u32;
}
pub fn move_indirect_reg_short(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_reg = cpu.next_byte() as usize;
    let src_adr = cpu.registers[src_reg] as usize;
    cpu.registers[dst_reg] = cpu.memory.short(src_adr) as u32;
}
pub fn move_indirect_reg_long(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_reg = cpu.next_byte() as usize;
    let src_adr = cpu.registers[src_reg] as usize;
    cpu.registers[dst_reg] = cpu.memory.long(src_adr);
}

pub fn move_imm_abs_byte(cpu: &mut Cpu) {
    let dst_adr = cpu.next_long() as usize;
    let src_val = cpu.next_byte();
    cpu.memory.set_byte(dst_adr, src_val);
}
pub fn move_imm_abs_short(cpu: &mut Cpu) {
    let dst_adr = cpu.next_long() as usize;
    let src_val = cpu.next_short();
    cpu.memory.set_short(dst_adr, src_val);
}
pub fn move_imm_abs_long(cpu: &mut Cpu) {
    let dst_adr = cpu.next_long() as usize;
    let src_val = cpu.next_long();
    cpu.memory.set_long(dst_adr, src_val);
}

pub fn move_reg_abs_byte(cpu: &mut Cpu) {
    let dst_adr = cpu.next_long() as usize;
    let src_reg = cpu.next_byte() as usize;
    cpu.memory.set_byte(dst_adr, cpu.registers[src_reg] as u8);
}
pub fn move_reg_abs_short(cpu: &mut Cpu) {
    let dst_adr = cpu.next_long() as usize;

    let src_reg = cpu.next_byte() as usize;
    cpu.memory.set_short(dst_adr, cpu.registers[src_reg] as u16);
}
pub fn move_reg_abs_long(cpu: &mut Cpu) {
    let dst_adr = cpu.next_long() as usize;
    let src_reg = cpu.next_byte() as usize;
    cpu.memory.set_long(dst_adr, cpu.registers[src_reg]);
}

pub fn move_abs_abs_byte(cpu: &mut Cpu) {
    let dst_adr = cpu.next_long() as usize;
    let src_adr = cpu.next_long() as usize;
    let src_val = cpu.memory.byte(src_adr);
    cpu.memory.set_byte(dst_adr, src_val);
}
pub fn move_abs_abs_short(cpu: &mut Cpu) {
    let dst_adr = cpu.next_long() as usize;
    let src_adr = cpu.next_long() as usize;
    let src_val = cpu.memory.short(src_adr);
    cpu.memory.set_short(dst_adr, src_val);
}
pub fn move_abs_abs_long(cpu: &mut Cpu) {
    let dst_adr = cpu.next_long() as usize;
    let src_adr = cpu.next_long() as usize;
    let src_val = cpu.memory.long(src_adr);
    cpu.memory.set_long(dst_adr, src_val);
}

pub fn move_mem_abs_byte(cpu: &mut Cpu) {
    let dst_adr = cpu.next_long() as usize;
    let src_adr = cpu.next_long() as usize;
    let src_val = cpu.memory.byte(src_adr + cpu.ip());
    cpu.memory.set_byte(dst_adr, src_val);
}
pub fn move_mem_abs_short(cpu: &mut Cpu) {
    let dst_adr = cpu.next_long() as usize;
    let src_adr = cpu.next_long() as usize;
    let src_val = cpu.memory.short(src_adr + cpu.ip());
    cpu.memory.set_short(dst_adr, src_val);
}
pub fn move_mem_abs_long(cpu: &mut Cpu) {
    let dst_adr = cpu.next_long() as usize;
    let src_adr = cpu.next_long() as usize;
    let src_val = cpu.memory.long(src_adr + cpu.ip());
    cpu.memory.set_long(dst_adr, src_val);
}

pub fn move_indirect_abs_byte(cpu: &mut Cpu) {
    let dst_adr = cpu.next_long() as usize;
    let src_reg = cpu.next_byte() as usize;
    let src_val = cpu.memory.byte(cpu.registers[src_reg] as usize);
    cpu.memory.set_byte(dst_adr, src_val);
}
pub fn move_indirect_abs_short(cpu: &mut Cpu) {
    let dst_adr = cpu.next_long() as usize;
    let src_reg = cpu.next_byte() as usize;
    let src_val = cpu.memory.short(cpu.registers[src_reg] as usize);
    cpu.memory.set_short(dst_adr, src_val);
}
pub fn move_indirect_abs_long(cpu: &mut Cpu) {
    let dst_adr = cpu.next_long() as usize;
    let src_reg = cpu.next_byte() as usize;
    let src_val = cpu.memory.long(cpu.registers[src_reg] as usize);
    cpu.memory.set_long(dst_adr, src_val);
}

pub fn move_imm_mem_byte(cpu: &mut Cpu) {
    let dst_adr = cpu.next_long() as usize;
    let src_val = cpu.next_byte();
    cpu.memory.set_byte(dst_adr + cpu.ip(), src_val);
}
pub fn move_imm_mem_short(cpu: &mut Cpu) {
    let dst_adr = cpu.next_long() as usize;
    let src_val = cpu.next_short();
    cpu.memory.set_short(dst_adr + cpu.ip(), src_val);
}
pub fn move_imm_mem_long(cpu: &mut Cpu) {
    let dst_adr = cpu.next_long() as usize;
    let src_val = cpu.next_long();
    cpu.memory.set_long(dst_adr + cpu.ip(), src_val);
}

pub fn move_reg_mem_byte(cpu: &mut Cpu) {
    let dst_adr = cpu.next_long() as usize;
    let src_reg = cpu.next_byte() as usize;
    cpu.memory
        .set_byte(dst_adr + cpu.ip(), cpu.registers[src_reg] as u8);
}
pub fn move_reg_mem_short(cpu: &mut Cpu) {
    let dst_adr = cpu.next_long() as usize;
    let src_reg = cpu.next_byte() as usize;
    cpu.memory
        .set_short(dst_adr + cpu.ip(), cpu.registers[src_reg] as u16);
}
pub fn move_reg_mem_long(cpu: &mut Cpu) {
    let dst_adr = cpu.next_long() as usize;
    let src_reg = cpu.next_byte() as usize;
    cpu.memory
        .set_long(dst_adr + cpu.ip(), cpu.registers[src_reg]);
}

pub fn move_mem_mem_byte(cpu: &mut Cpu) {
    let dst_adr = cpu.next_long() as usize;
    let src_adr = cpu.next_long() as usize;
    let src_val = cpu.memory.byte(src_adr + cpu.ip());
    cpu.memory.set_byte(dst_adr + cpu.ip(), src_val);
}
pub fn move_mem_mem_short(cpu: &mut Cpu) {
    let dst_adr = cpu.next_long() as usize;
    let src_adr = cpu.next_long() as usize;
    let src_val = cpu.memory.short(src_adr + cpu.ip());
    cpu.memory.set_short(dst_adr + cpu.ip(), src_val);
}
pub fn move_mem_mem_long(cpu: &mut Cpu) {
    let dst_adr = cpu.next_long() as usize;
    let src_adr = cpu.next_long() as usize;
    let src_val = cpu.memory.long(src_adr + cpu.ip());
    cpu.memory.set_long(dst_adr + cpu.ip(), src_val);
}

pub fn move_abs_mem_byte(cpu: &mut Cpu) {
    let dst_adr = cpu.next_long() as usize;
    let src_adr = cpu.next_long() as usize;
    let src_val = cpu.memory.byte(src_adr);
    cpu.memory.set_byte(dst_adr + cpu.ip(), src_val);
}
pub fn move_abs_mem_short(cpu: &mut Cpu) {
    let dst_adr = cpu.next_long() as usize;
    let src_adr = cpu.next_long() as usize;
    let src_val = cpu.memory.short(src_adr);
    cpu.memory.set_short(dst_adr + cpu.ip(), src_val);
}
pub fn move_abs_mem_long(cpu: &mut Cpu) {
    let dst_adr = cpu.next_long() as usize;
    let src_adr = cpu.next_long() as usize;
    let src_val = cpu.memory.long(src_adr);
    cpu.memory.set_long(dst_adr + cpu.ip(), src_val);
}

pub fn move_indirect_mem_byte(cpu: &mut Cpu) {
    let dst_adr = cpu.next_long() as usize;
    let src_reg = cpu.next_byte() as usize;
    let src_val = cpu.memory.byte(cpu.registers[src_reg] as usize);
    cpu.memory.set_byte(dst_adr + cpu.ip(), src_val);
}
pub fn move_indirect_mem_short(cpu: &mut Cpu) {
    let dst_adr = cpu.next_long() as usize;
    let src_reg = cpu.next_byte() as usize;
    let src_val = cpu.memory.short(cpu.registers[src_reg] as usize);
    cpu.memory.set_short(dst_adr + cpu.ip(), src_val);
}
pub fn move_indirect_mem_long(cpu: &mut Cpu) {
    let dst_adr = cpu.next_long() as usize;
    let src_reg = cpu.next_byte() as usize;
    let src_val = cpu.memory.long(cpu.registers[src_reg] as usize);
    cpu.memory.set_long(dst_adr + cpu.ip(), src_val);
}

pub fn move_imm_indirect_byte(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_val = cpu.next_byte();
    let dst_adr = cpu.registers[dst_reg] as usize;
    cpu.memory.set_byte(dst_adr, src_val);
}
pub fn move_imm_indirect_short(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_val = cpu.next_short();
    let dst_adr = cpu.registers[dst_reg] as usize;
    cpu.memory.set_short(dst_adr, src_val);
}
pub fn move_imm_indirect_long(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_val = cpu.next_long();
    let dst_adr = cpu.registers[dst_reg] as usize;
    cpu.memory.set_long(dst_adr, src_val);
}

pub fn move_reg_indirect_byte(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_reg = cpu.next_byte() as usize;
    let dst_adr = cpu.registers[dst_reg] as usize;
    cpu.memory.set_byte(dst_adr, cpu.registers[src_reg] as u8);
}
pub fn move_reg_indirect_short(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_reg = cpu.next_byte() as usize;
    let dst_adr = cpu.registers[dst_reg] as usize;
    cpu.memory.set_short(dst_adr, cpu.registers[src_reg] as u16);
}
pub fn move_reg_indirect_long(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_reg = cpu.next_byte() as usize;
    let dst_adr = cpu.registers[dst_reg] as usize;
    cpu.memory.set_long(dst_adr, cpu.registers[src_reg]);
}

pub fn move_abs_indirect_byte(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_adr = cpu.next_long() as usize;
    let dst_adr = cpu.registers[dst_reg] as usize;
    let src_val = cpu.memory.byte(src_adr);
    cpu.memory.set_byte(dst_adr, src_val);
}
pub fn move_abs_indirect_short(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_adr = cpu.next_long() as usize;
    let dst_adr = cpu.registers[dst_reg] as usize;
    let src_val = cpu.memory.short(src_adr);
    cpu.memory.set_short(dst_adr, src_val);
}
pub fn move_abs_indirect_long(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_adr = cpu.next_long() as usize;
    let dst_adr = cpu.registers[dst_reg] as usize;
    let src_val = cpu.memory.long(src_adr);
    cpu.memory.set_long(dst_adr, src_val);
}

pub fn move_mem_indirect_byte(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_adr = cpu.next_long() as usize;
    let dst_adr = cpu.registers[dst_reg] as usize;
    let src_val = cpu.memory.byte(src_adr + cpu.ip());
    cpu.memory.set_byte(dst_adr, src_val);
}
pub fn move_mem_indirect_short(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_adr = cpu.next_long() as usize;
    let dst_adr = cpu.registers[dst_reg] as usize;
    let src_val = cpu.memory.short(src_adr + cpu.ip());
    cpu.memory.set_short(dst_adr, src_val);
}
pub fn move_mem_indirect_long(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_adr = cpu.next_long() as usize;
    let dst_adr = cpu.registers[dst_reg] as usize;
    let src_val = cpu.memory.long(src_adr + cpu.ip());
    cpu.memory.set_long(dst_adr, src_val);
}

pub fn move_indirect_indirect_byte(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_reg = cpu.next_byte() as usize;
    let dst_adr = cpu.registers[dst_reg] as usize;
    let src_adr = cpu.registers[src_reg] as usize;
    let src_val = cpu.memory.byte(src_adr);
    cpu.memory.set_byte(dst_adr, src_val);
}
pub fn move_indirect_indirect_short(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_reg = cpu.next_byte() as usize;
    let dst_adr = cpu.registers[dst_reg] as usize;
    let src_adr = cpu.registers[src_reg] as usize;
    let src_val = cpu.memory.short(src_adr);
    cpu.memory.set_short(dst_adr, src_val);
}
pub fn move_indirect_indirect_long(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_reg = cpu.next_byte() as usize;
    let dst_adr = cpu.registers[dst_reg] as usize;
    let src_adr = cpu.registers[src_reg] as usize;
    let src_val = cpu.memory.long(src_adr);
    cpu.memory.set_long(dst_adr, src_val);
}

pub fn add_byte_imm(cpu: &mut Cpu) {
    let lhs = (unsafe { cpu.registers.get_unchecked(0) } & 0xFF) as u8;
    let rhs = cpu.next_byte();
    let (result, carry) = lhs.overflowing_add(rhs);
    unsafe {
        *cpu.registers.get_unchecked_mut(0) = result as u32;
    }
    cpu.set_flag(Cpu::CARRY_FLAG, carry);
}
pub fn add_short_imm(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFFFF) as u16;
    let rhs = cpu.next_short();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.registers[0] = result as u32;
    cpu.set_flag(Cpu::CARRY_FLAG, carry);
}
pub fn add_long_imm(cpu: &mut Cpu) {
    let lhs = cpu.registers[0];
    let rhs = cpu.next_long();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.registers[0] = result;
    cpu.set_flag(Cpu::CARRY_FLAG, carry);
}

pub fn add_byte_reg(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFF) as u8;
    let index = cpu.next_byte() as usize;
    let rhs = (cpu.registers[index] & 0xFF) as u8;
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.registers[0] = result as u32;
    cpu.set_flag(Cpu::CARRY_FLAG, carry);
}
pub fn add_short_reg(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFFFF) as u16;
    let index = cpu.next_byte() as usize;
    let rhs = (cpu.registers[index] & 0xFFFF) as u16;
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.registers[0] = result as u32;
    cpu.set_flag(Cpu::CARRY_FLAG, carry);
}
pub fn add_long_reg(cpu: &mut Cpu) {
    let lhs = cpu.registers[0];
    let index = cpu.next_byte() as usize;
    let rhs = cpu.registers[index];
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.registers[0] = result;
    cpu.set_flag(Cpu::CARRY_FLAG, carry);
}

pub fn add_carry_byte_imm(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFF) as u8;
    let rhs = cpu.next_byte();
    let carry = cpu.has_flag(Cpu::CARRY_FLAG) as u8;
    let (result, carry0) = lhs.overflowing_add(carry);
    let (result, carry1) = result.overflowing_add(rhs);
    cpu.registers[0] = result as u32;
    cpu.set_flag(Cpu::CARRY_FLAG, carry0 | carry1);
}
pub fn add_carry_short_imm(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFFFF) as u16;
    let rhs = cpu.next_short();
    let carry = cpu.has_flag(Cpu::CARRY_FLAG) as u16;
    let (result, carry0) = lhs.overflowing_add(carry);
    let (result, carry1) = result.overflowing_add(rhs);
    cpu.registers[0] = result as u32;
    cpu.set_flag(Cpu::CARRY_FLAG, carry0 | carry1);
}
pub fn add_carry_long_imm(cpu: &mut Cpu) {
    let lhs = cpu.registers[0];
    let rhs = cpu.next_long();
    let carry = cpu.has_flag(Cpu::CARRY_FLAG) as u32;
    let (result, carry0) = lhs.overflowing_add(carry);
    let (result, carry1) = result.overflowing_add(rhs);
    cpu.registers[0] = result;
    cpu.set_flag(Cpu::CARRY_FLAG, carry0 | carry1);
}

pub fn add_carry_byte_reg(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFF) as u8;
    let index = cpu.next_byte() as usize;
    let rhs = (cpu.registers[index] & 0xFF) as u8;
    let carry = cpu.has_flag(Cpu::CARRY_FLAG) as u8;
    let (result, carry0) = lhs.overflowing_add(carry);
    let (result, carry1) = result.overflowing_add(rhs);
    cpu.registers[0] = result as u32;
    cpu.set_flag(Cpu::CARRY_FLAG, carry0 | carry1);
}
pub fn add_carry_short_reg(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFFFF) as u16;
    let index = cpu.next_byte() as usize;
    let rhs = (cpu.registers[index] & 0xFFFF) as u16;
    let carry = cpu.has_flag(Cpu::CARRY_FLAG) as u16;
    let (result, carry0) = lhs.overflowing_add(carry);
    let (result, carry1) = result.overflowing_add(rhs);
    cpu.registers[0] = result as u32;
    cpu.set_flag(Cpu::CARRY_FLAG, carry0 | carry1);
}
pub fn add_carry_long_reg(cpu: &mut Cpu) {
    let lhs = cpu.registers[0];
    let index = cpu.next_byte() as usize;
    let rhs = cpu.registers[index];
    let carry = cpu.has_flag(Cpu::CARRY_FLAG) as u32;
    let (result, carry0) = lhs.overflowing_add(carry);
    let (result, carry1) = result.overflowing_add(rhs);
    cpu.registers[0] = result;
    cpu.set_flag(Cpu::CARRY_FLAG, carry0 | carry1);
}

pub fn sub_byte_imm(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFF) as u8;
    let rhs = cpu.next_byte();
    let (result, carry) = lhs.overflowing_sub(rhs);
    cpu.registers[0] = result as u32;
    cpu.set_flag(Cpu::CARRY_FLAG, carry);
}
pub fn sub_short_imm(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFFFF) as u16;
    let rhs = cpu.next_short();
    let (result, carry) = lhs.overflowing_sub(rhs);
    cpu.registers[0] = result as u32;
    cpu.set_flag(Cpu::CARRY_FLAG, carry);
}
pub fn sub_long_imm(cpu: &mut Cpu) {
    let lhs = cpu.registers[0];
    let rhs = cpu.next_long();
    let (result, carry) = lhs.overflowing_sub(rhs);
    cpu.registers[0] = result;
    cpu.set_flag(Cpu::CARRY_FLAG, carry);
}

pub fn sub_byte_reg(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFF) as u8;
    let index = cpu.next_byte() as usize;
    let rhs = (cpu.registers[index] & 0xFF) as u8;
    let (result, carry) = lhs.overflowing_sub(rhs);
    cpu.registers[0] = result as u32;
    cpu.set_flag(Cpu::CARRY_FLAG, carry);
}
pub fn sub_short_reg(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFFFF) as u16;
    let index = cpu.next_byte() as usize;
    let rhs = (cpu.registers[index] & 0xFFFF) as u16;
    let (result, carry) = lhs.overflowing_sub(rhs);
    cpu.registers[0] = result as u32;
    cpu.set_flag(Cpu::CARRY_FLAG, carry);
}
pub fn sub_long_reg(cpu: &mut Cpu) {
    let lhs = cpu.registers[0];
    let index = cpu.next_byte() as usize;
    let rhs = cpu.registers[index];
    let (result, carry) = lhs.overflowing_sub(rhs);
    cpu.registers[0] = result;
    cpu.set_flag(Cpu::CARRY_FLAG, carry);
}

pub fn sub_borrow_byte_imm(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFF) as u8;
    let rhs = cpu.next_byte();
    let carry = cpu.has_flag(Cpu::CARRY_FLAG) as u8;
    let (result, carry0) = lhs.overflowing_sub(carry);
    let (result, carry1) = result.overflowing_sub(rhs);
    cpu.registers[0] = result as u32;
    cpu.set_flag(Cpu::CARRY_FLAG, carry0 | carry1);
}
pub fn sub_borrow_short_imm(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFFFF) as u16;
    let rhs = cpu.next_short();
    let carry = cpu.has_flag(Cpu::CARRY_FLAG) as u16;
    let (result, carry0) = lhs.overflowing_sub(carry);
    let (result, carry1) = result.overflowing_sub(rhs);
    cpu.registers[0] = result as u32;
    cpu.set_flag(Cpu::CARRY_FLAG, carry0 | carry1);
}
pub fn sub_borrow_long_imm(cpu: &mut Cpu) {
    let lhs = cpu.registers[0];
    let rhs = cpu.next_long();
    let carry = cpu.has_flag(Cpu::CARRY_FLAG) as u32;
    let (result, carry0) = lhs.overflowing_sub(carry);
    let (result, carry1) = result.overflowing_sub(rhs);
    cpu.registers[0] = result;
    cpu.set_flag(Cpu::CARRY_FLAG, carry0 | carry1);
}

pub fn sub_borrow_byte_reg(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFF) as u8;
    let index = cpu.next_byte() as usize;
    let rhs = (cpu.registers[index] & 0xFF) as u8;
    let carry = cpu.has_flag(Cpu::CARRY_FLAG) as u8;
    let (result, carry0) = lhs.overflowing_sub(carry);
    let (result, carry1) = result.overflowing_sub(rhs);
    cpu.registers[0] = result as u32;
    cpu.set_flag(Cpu::CARRY_FLAG, carry0 | carry1);
}
pub fn sub_borrow_short_reg(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFFFF) as u16;
    let index = cpu.next_byte() as usize;
    let rhs = (cpu.registers[index] & 0xFFFF) as u16;
    let carry = cpu.has_flag(Cpu::CARRY_FLAG) as u16;
    let (result, carry0) = lhs.overflowing_sub(carry);
    let (result, carry1) = result.overflowing_sub(rhs);
    cpu.registers[0] = result as u32;
    cpu.set_flag(Cpu::CARRY_FLAG, carry0 | carry1);
}
pub fn sub_borrow_long_reg(cpu: &mut Cpu) {
    let lhs = cpu.registers[0];
    let index = cpu.next_byte() as usize;
    let rhs = cpu.registers[index];
    let carry = cpu.has_flag(Cpu::CARRY_FLAG) as u32;
    let (result, carry0) = lhs.overflowing_sub(carry);
    let (result, carry1) = result.overflowing_sub(rhs);
    cpu.registers[0] = result;
    cpu.set_flag(Cpu::CARRY_FLAG, carry0 | carry1);
}

pub fn mul_byte_imm(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFF) as u8;
    let rhs = cpu.next_byte();
    let result = lhs.wrapping_mul(rhs);
    cpu.registers[0] = (result & 0xFF) as u32;
}
pub fn mul_short_imm(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFFFF) as u16;
    let rhs = cpu.next_short();
    let result = lhs.wrapping_mul(rhs);
    cpu.registers[0] = (result & 0xFFFF) as u32;
}
pub fn mul_long_imm(cpu: &mut Cpu) {
    let lhs = cpu.registers[0];
    let rhs = cpu.next_long();
    let result = lhs.wrapping_mul(rhs);
    cpu.registers[0] = result;
}

pub fn mul_byte_reg(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFF) as u8;
    let index = cpu.next_byte() as usize;
    let rhs = (cpu.registers[index] & 0xFF) as u8;
    let result = lhs.wrapping_mul(rhs);
    cpu.registers[0] = (result & 0xFF) as u32;
}
pub fn mul_short_reg(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFFFF) as u16;
    let index = cpu.next_byte() as usize;
    let rhs = (cpu.registers[index] & 0xFFFF) as u16;
    let result = lhs.wrapping_mul(rhs);
    cpu.registers[0] = (result & 0xFFFF) as u32;
}
pub fn mul_long_reg(cpu: &mut Cpu) {
    let lhs = cpu.registers[0];
    let index = cpu.next_byte() as usize;
    let rhs = cpu.registers[index];
    let result = lhs.wrapping_mul(rhs);
    cpu.registers[0] = result;
}

pub fn div_byte_imm(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFF) as u8;
    let rhs = cpu.next_byte();
    let quotient = lhs / rhs;
    let remainder = lhs % rhs;
    cpu.registers[0] = quotient as u32;
    cpu.registers[1] = remainder as u32;
}
pub fn div_short_imm(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFFFF) as u16;
    let rhs = cpu.next_short();
    let quotient = lhs / rhs;
    let remainder = lhs % rhs;
    cpu.registers[0] = quotient as u32;
    cpu.registers[1] = remainder as u32;
}
pub fn div_long_imm(cpu: &mut Cpu) {
    let lhs = cpu.registers[0];
    let rhs = cpu.next_long();
    let quotient = lhs / rhs;
    let remainder = lhs % rhs;
    cpu.registers[0] = quotient;
    cpu.registers[1] = remainder;
}

pub fn div_byte_reg(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFF) as u8;
    let index = cpu.next_byte() as usize;
    let rhs = (cpu.registers[index] & 0xFF) as u8;
    let quotient = lhs / rhs;
    let remainder = lhs % rhs;
    cpu.registers[0] = quotient as u32;
    cpu.registers[1] = remainder as u32;
}
pub fn div_short_reg(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFFFF) as u16;
    let index = cpu.next_byte() as usize;
    let rhs = (cpu.registers[index] & 0xFFFF) as u16;
    let quotient = lhs / rhs;
    let remainder = lhs % rhs;
    cpu.registers[0] = quotient as u32;
    cpu.registers[1] = remainder as u32;
}
pub fn div_long_reg(cpu: &mut Cpu) {
    let lhs = cpu.registers[0];
    let index = cpu.next_byte() as usize;
    let rhs = cpu.registers[index];
    let quotient = lhs / rhs;
    let remainder = lhs % rhs;
    cpu.registers[0] = quotient;
    cpu.registers[1] = remainder;
}

pub fn signed_mul_byte_imm(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFF) as u8;
    let rhs = cpu.next_byte() as i8;
    let result = (lhs as i8).wrapping_mul(rhs);
    cpu.registers[0] = result as u32;
}
pub fn signed_mul_short_imm(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFFFF) as u16;
    let rhs = cpu.next_short() as i16;
    let result = (lhs as i16).wrapping_mul(rhs);
    cpu.registers[0] = result as u32;
}
pub fn signed_mul_long_imm(cpu: &mut Cpu) {
    let lhs = cpu.registers[0];
    let rhs = cpu.next_long() as i32;
    let result = (lhs as i32).wrapping_mul(rhs);
    cpu.registers[0] = result as u32;
}

pub fn signed_mul_byte_reg(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFF) as u8;
    let index = cpu.next_byte() as usize;
    let rhs = cpu.registers[index] as i8;
    let result = (lhs as i8).wrapping_mul(rhs);
    cpu.registers[0] = result as u32;
}
pub fn signed_mul_short_reg(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFFFF) as u16;
    let index = cpu.next_byte() as usize;
    let rhs = (cpu.registers[index] & 0xFFFF) as i16;
    let result = (lhs as i16).wrapping_mul(rhs);
    cpu.registers[0] = result as u32;
}
pub fn signed_mul_long_reg(cpu: &mut Cpu) {
    let lhs = cpu.registers[0];
    let index = cpu.next_byte() as usize;
    let rhs = cpu.registers[index] as i32;
    let result = (lhs as i32).wrapping_mul(rhs);
    cpu.registers[0] = result as u32;
}

pub fn signed_div_byte_imm(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFF) as u8;
    let rhs = cpu.next_byte() as i8;
    let quotient = (lhs as i8) / rhs;
    let remainder = (lhs as i8) % rhs;
    cpu.registers[0] = quotient as u32;
    cpu.registers[1] = remainder as u32;
}
pub fn signed_div_short_imm(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFFFF) as u16;
    let rhs = cpu.next_short() as i16;
    let quotient = (lhs as i16) / rhs;
    let remainder = (lhs as i16) % rhs;
    cpu.registers[0] = quotient as u32;
    cpu.registers[1] = remainder as u32;
}
pub fn signed_div_long_imm(cpu: &mut Cpu) {
    let lhs = cpu.registers[0];
    let rhs = cpu.next_long() as i32;
    let quotient = lhs as i32 / rhs;
    let remainder = lhs as i32 % rhs;
    cpu.registers[0] = quotient as u32;
    cpu.registers[1] = remainder as u32;
}

pub fn signed_div_byte_reg(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFF) as u8;
    let index = cpu.next_byte() as usize;
    let rhs = cpu.registers[index] as i8;
    let quotient = (lhs as i8) / rhs;
    let remainder = (lhs as i8) % rhs;
    cpu.registers[0] = quotient as u32;
    cpu.registers[1] = remainder as u32;
}
pub fn signed_div_short_reg(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFFFF) as u16;
    let index = cpu.next_byte() as usize;
    let rhs = (cpu.registers[index] & 0xFFFF) as i16;
    let quotient = (lhs as i16) / rhs;
    let remainder = (lhs as i16) % rhs;
    cpu.registers[0] = quotient as u32;
    cpu.registers[1] = remainder as u32;
}
pub fn signed_div_long_reg(cpu: &mut Cpu) {
    let lhs = cpu.registers[0];
    let index = cpu.next_byte() as usize;
    let rhs = cpu.registers[index] as i32;
    let quotient = lhs as i32 / rhs;
    let remainder = lhs as i32 % rhs;
    cpu.registers[0] = quotient as u32;
    cpu.registers[1] = remainder as u32;
}

pub fn and_byte_imm(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFF) as u8;
    let rhs = cpu.next_byte();
    cpu.registers[0] = (lhs & rhs) as u32;
}
pub fn and_short_imm(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFFFF) as u16;
    let rhs = cpu.next_short();
    cpu.registers[0] = (lhs & rhs) as u32;
}
pub fn and_long_imm(cpu: &mut Cpu) {
    let lhs = cpu.registers[0];
    let rhs = cpu.next_long();
    cpu.registers[0] = lhs & rhs;
}

pub fn and_byte_reg(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFF) as u8;
    let index = cpu.next_byte() as usize;
    let rhs = (cpu.registers[index] & 0xFF) as u8;
    cpu.registers[0] = (lhs & rhs) as u32;
}
pub fn and_short_reg(cpu: &mut Cpu) {
    let lhs = (cpu.registers[0] & 0xFFFF) as u16;
    let index = cpu.next_byte() as usize;
    let rhs = (cpu.registers[index] & 0xFFFF) as u16;
    cpu.registers[0] = (lhs & rhs) as u32;
}
pub fn and_long_reg(cpu: &mut Cpu) {
    let lhs = cpu.registers[0];
    let index = cpu.next_byte() as usize;
    let rhs = cpu.registers[index];
    cpu.registers[0] = lhs & rhs;
}

pub fn or_byte_imm(cpu: &mut Cpu) {
    let val = cpu.next_short();
    let res = cpu.registers[0] as u16 | val;
    cpu.registers[0] = res as u32;
}
pub fn or_short_imm(cpu: &mut Cpu) {
    let val = cpu.next_short();
    let res = cpu.registers[0] as u16 | val;
    cpu.registers[0] = res as u32;
}
pub fn or_long_imm(cpu: &mut Cpu) {
    let val = cpu.next_long();
    let res = cpu.registers[0] as u32 | val;
    cpu.registers[0] = res as u32;
}

pub fn or_byte_reg(cpu: &mut Cpu) {
    let reg = cpu.next_byte() as usize;
    let val = cpu.registers[reg] as u8;
    let res = cpu.registers[0] as u8 | val;
    cpu.registers[0] = res as u32;
}
pub fn or_short_reg(cpu: &mut Cpu) {
    let reg = cpu.next_short() as usize;
    let val = cpu.registers[reg] as u16;
    let res = cpu.registers[0] as u16 | val;
    cpu.registers[0] = res as u32;
}
pub fn or_long_reg(cpu: &mut Cpu) {
    let reg = cpu.next_long() as usize;
    let val = cpu.registers[reg] as u32;
    let res = cpu.registers[0] as u32 | val;
    cpu.registers[0] = res as u32;
}

pub fn xor_byte_imm(cpu: &mut Cpu) {
    let val = cpu.next_byte();
    let res = cpu.registers[0] as u8 ^ val;
    cpu.registers[0] = res as u32;
}
pub fn xor_short_imm(cpu: &mut Cpu) {
    let val = cpu.next_short();
    let res = cpu.registers[0] as u16 ^ val;
    cpu.registers[0] = res as u32;
}
pub fn xor_long_imm(cpu: &mut Cpu) {
    let val = cpu.next_long();
    let res = cpu.registers[0] as u32 ^ val;
    cpu.registers[0] = res as u32;
}

pub fn xor_byte_reg(cpu: &mut Cpu) {
    let reg = cpu.next_byte() as usize;
    let val = cpu.registers[reg] as u8;
    let res = cpu.registers[0] as u8 ^ val;
    cpu.registers[0] = res as u32;
}
pub fn xor_short_reg(cpu: &mut Cpu) {
    let reg = cpu.next_short() as usize;
    let val = cpu.registers[reg] as u16;
    let res = cpu.registers[0] as u16 ^ val;
    cpu.registers[0] = res as u32;
}
pub fn xor_long_reg(cpu: &mut Cpu) {
    let reg = cpu.next_long() as usize;
    let val = cpu.registers[reg] as u32;
    let res = cpu.registers[0] as u32 ^ val;
    cpu.registers[0] = res as u32;
}

pub fn push_byte_imm(cpu: &mut Cpu) {
    cpu.dec_sp(1);
    let value = cpu.next_byte();
    cpu.memory.set_byte(cpu.sp(), value);
}
pub fn push_short_imm(cpu: &mut Cpu) {
    cpu.dec_sp(2);
    let value = cpu.next_short();
    cpu.memory.set_short(cpu.sp(), value);
}
pub fn push_long_imm(cpu: &mut Cpu) {
    cpu.dec_sp(4);
    let value = cpu.next_long();
    cpu.memory.set_long(cpu.sp(), value);
}

pub fn push_byte_reg(cpu: &mut Cpu) {
    cpu.dec_sp(1);
    let index = cpu.next_byte() as usize;
    let value = (cpu.registers[index] & 0xFF) as u8;
    cpu.memory.set_byte(cpu.sp(), value);
}
pub fn push_short_reg(cpu: &mut Cpu) {
    cpu.dec_sp(2);
    let index = cpu.next_byte() as usize;
    let value = (cpu.registers[index] & 0xFFFF) as u16;
    cpu.memory.set_short(cpu.sp(), value);
}
pub fn push_long_reg(cpu: &mut Cpu) {
    cpu.dec_sp(4);
    let index = cpu.next_byte() as usize;
    let value = cpu.registers[index];
    cpu.memory.set_long(cpu.sp(), value);
}

pub fn compare_byte_imm(cpu: &mut Cpu) {
    let lhs = cpu.registers[0];
    let rhs = cpu.next_byte();
    cpu.registers[0] = if lhs as u8 == rhs { 1 } else { 0 };
}
pub fn compare_short_imm(cpu: &mut Cpu) {
    let lhs = cpu.registers[0];
    let rhs = cpu.next_short();
    cpu.registers[0] = if lhs as u16 == rhs { 1 } else { 0 };
}
pub fn compare_long_imm(cpu: &mut Cpu) {
    let lhs = cpu.registers[0];
    let rhs = cpu.next_long();
    cpu.registers[0] = if lhs == rhs { 1 } else { 0 };
}

pub fn compare_byte_reg(cpu: &mut Cpu) {
    let lhs = cpu.registers[0];
    let index = cpu.next_byte() as usize;
    let rhs = cpu.registers[index] as u8;
    cpu.registers[0] = if lhs as u8 == rhs { 1 } else { 0 };
}
pub fn compare_short_reg(cpu: &mut Cpu) {
    let lhs = cpu.registers[0];
    let index = cpu.next_byte() as usize;
    let rhs = cpu.registers[index] as u16;
    cpu.registers[0] = if lhs as u16 == rhs { 1 } else { 0 };
}
pub fn compare_long_reg(cpu: &mut Cpu) {
    let lhs = cpu.registers[0];
    let index = cpu.next_byte() as usize;
    let rhs = cpu.registers[index];
    cpu.registers[0] = if lhs == rhs { 1 } else { 0 };
}

pub fn log_shift_left_byte_imm(cpu: &mut Cpu) {
    let val = cpu.next_byte();
    cpu.registers[0] = (cpu.registers[0] as u8).shl(val) as u32;
}
pub fn log_shift_left_short_imm(cpu: &mut Cpu) {
    let val = cpu.next_byte();
    cpu.registers[0] = (cpu.registers[0] as u16).shl(val) as u32;
}
pub fn log_shift_left_long_imm(cpu: &mut Cpu) {
    let val = cpu.next_byte();
    cpu.registers[0] = (cpu.registers[0] as u32).shl(val) as u32;
}

pub fn log_shift_left_byte_reg(cpu: &mut Cpu) {
    let reg = cpu.next_byte() as usize;
    let val = cpu.registers[reg] as u8;
    cpu.registers[0] = (cpu.registers[0] as u8).shl(val) as u32;
}
pub fn log_shift_left_short_reg(cpu: &mut Cpu) {
    let reg = cpu.next_byte() as usize;
    let val = cpu.registers[reg] as u16;
    cpu.registers[0] = (cpu.registers[0] as u16).shl(val) as u32;
}
pub fn log_shift_left_long_reg(cpu: &mut Cpu) {
    let reg = cpu.next_byte() as usize;
    let val = cpu.registers[reg] as u32;
    cpu.registers[0] = (cpu.registers[0] as u32).shl(val) as u32;
}

pub fn log_shift_right_byte_imm(cpu: &mut Cpu) {
    let val = cpu.next_byte();
    cpu.registers[0] = (cpu.registers[0] as u8).shr(val) as u32;
}
pub fn log_shift_right_short_imm(cpu: &mut Cpu) {
    let val = cpu.next_byte();
    cpu.registers[0] = (cpu.registers[0] as u16).shr(val) as u32;
}
pub fn log_shift_right_long_imm(cpu: &mut Cpu) {
    let val = cpu.next_byte();
    cpu.registers[0] = (cpu.registers[0] as u32).shr(val) as u32;
}
pub fn log_shift_right_byte_reg(cpu: &mut Cpu) {
    let reg = cpu.next_byte() as usize;
    let val = cpu.registers[reg] as u8;
    cpu.registers[0] = (cpu.registers[0] as u8).shr(val) as u32;
}
pub fn log_shift_right_short_reg(cpu: &mut Cpu) {
    let reg = cpu.next_byte() as usize;
    let val = cpu.registers[reg] as u16;
    cpu.registers[0] = (cpu.registers[0] as u16).shr(val) as u32;
}
pub fn log_shift_right_long_reg(cpu: &mut Cpu) {
    let reg = cpu.next_byte() as usize;
    let val = cpu.registers[reg] as u32;
    cpu.registers[0] = (cpu.registers[0] as u32).shr(val) as u32;
}

pub fn arith_shift_left_byte_imm(cpu: &mut Cpu) {
    let val = cpu.next_byte();
    cpu.registers[0] = (cpu.registers[0] as u8).shl(val) as u32;
}
pub fn arith_shift_left_short_imm(cpu: &mut Cpu) {
    let val = cpu.next_byte();
    cpu.registers[0] = (cpu.registers[0] as u16).shl(val) as u32;
}
pub fn arith_shift_left_long_imm(cpu: &mut Cpu) {
    let val = cpu.next_byte();
    cpu.registers[0] = (cpu.registers[0] as u32).shl(val) as u32;
}

pub fn arith_shift_left_byte_reg(cpu: &mut Cpu) {
    let reg = cpu.next_byte() as usize;
    let val = cpu.registers[reg] as u8;
    cpu.registers[0] = (cpu.registers[0] as i8).shl(val) as u32;
}
pub fn arith_shift_left_short_reg(cpu: &mut Cpu) {
    let reg = cpu.next_byte() as usize;
    let val = cpu.registers[reg] as u16;
    cpu.registers[0] = (cpu.registers[0] as i16).shl(val) as u32;
}
pub fn arith_shift_left_long_reg(cpu: &mut Cpu) {
    let reg = cpu.next_byte() as usize;
    let val = cpu.registers[reg] as u32;
    cpu.registers[0] = (cpu.registers[0] as i32).shl(val) as u32;
}

pub fn arith_shift_right_byte_imm(cpu: &mut Cpu) {
    let val = cpu.next_byte();
    cpu.registers[0] = (cpu.registers[0] as u8).shr(val) as u32;
}
pub fn arith_shift_right_short_imm(cpu: &mut Cpu) {
    let val = cpu.next_byte();
    cpu.registers[0] = (cpu.registers[0] as u16).shr(val) as u32;
}
pub fn arith_shift_right_long_imm(cpu: &mut Cpu) {
    let val = cpu.next_byte();
    cpu.registers[0] = (cpu.registers[0] as u32).shr(val) as u32;
}

pub fn arith_shift_right_byte_reg(cpu: &mut Cpu) {
    let reg = cpu.next_byte() as usize;
    let val = cpu.registers[reg] as u8;
    cpu.registers[0] = (cpu.registers[0] as i8).shr(val) as u32;
}
pub fn arith_shift_right_short_reg(cpu: &mut Cpu) {
    let reg = cpu.next_byte() as usize;
    let val = cpu.registers[reg] as u16;
    cpu.registers[0] = (cpu.registers[0] as i16).shr(val) as u32;
}
pub fn arith_shift_right_long_reg(cpu: &mut Cpu) {
    let reg = cpu.next_byte() as usize;
    let val = cpu.registers[reg] as u32;
    cpu.registers[0] = (cpu.registers[0] as i32).shr(val) as u32;
}

pub fn rotate_left_byte_imm(cpu: &mut Cpu) {
    let val = cpu.next_byte();
    cpu.registers[0] = (cpu.registers[0] as u8).rotate_left(val as u32) as u32;
}
pub fn rotate_left_short_imm(cpu: &mut Cpu) {
    let val = cpu.next_byte();
    cpu.registers[0] = (cpu.registers[0] as u16).rotate_left(val as u32) as u32;
}
pub fn rotate_left_long_imm(cpu: &mut Cpu) {
    let val = cpu.next_byte();
    cpu.registers[0] = (cpu.registers[0] as u32).rotate_left(val as u32) as u32;
}

pub fn rotate_left_byte_reg(cpu: &mut Cpu) {
    let reg = cpu.next_byte() as usize;
    let val = cpu.registers[reg] as u32;
    cpu.registers[0] = (cpu.registers[0] as u8).rotate_left(val) as u32;
}
pub fn rotate_left_short_reg(cpu: &mut Cpu) {
    let reg = cpu.next_byte() as usize;
    let val = cpu.registers[reg] as u32;
    cpu.registers[0] = (cpu.registers[0] as u16).rotate_left(val) as u32;
}
pub fn rotate_left_long_reg(cpu: &mut Cpu) {
    let reg = cpu.next_byte() as usize;
    let val = cpu.registers[reg] as u32;
    cpu.registers[0] = (cpu.registers[0] as u32).rotate_left(val) as u32;
}

pub fn rotate_right_byte_imm(cpu: &mut Cpu) {
    let val = cpu.next_byte();
    cpu.registers[0] = (cpu.registers[0] as u8).rotate_right(val as u32) as u32;
}
pub fn rotate_right_short_imm(cpu: &mut Cpu) {
    let val = cpu.next_byte();
    cpu.registers[0] = (cpu.registers[0] as u16).rotate_right(val as u32) as u32;
}
pub fn rotate_right_long_imm(cpu: &mut Cpu) {
    let val = cpu.next_byte();
    cpu.registers[0] = (cpu.registers[0] as u32).rotate_right(val as u32) as u32;
}

pub fn rotate_right_byte_reg(cpu: &mut Cpu) {
    let reg = cpu.next_byte() as usize;
    let val = cpu.registers[reg] as u32;
    cpu.registers[0] = (cpu.registers[0] as u8).rotate_right(val) as u32;
}
pub fn rotate_right_short_reg(cpu: &mut Cpu) {
    let reg = cpu.next_byte() as usize;
    let val = cpu.registers[reg] as u32;
    cpu.registers[0] = (cpu.registers[0] as u16).rotate_right(val) as u32;
}
pub fn rotate_right_long_reg(cpu: &mut Cpu) {
    let reg = cpu.next_byte() as usize;
    let val = cpu.registers[reg] as u32;
    cpu.registers[0] = (cpu.registers[0] as u32).rotate_right(val) as u32;
}

pub fn pop_byte(cpu: &mut Cpu) {
    let dest = cpu.next_byte() as usize;
    let value = cpu.memory.byte(cpu.sp());
    unsafe {
        *cpu.registers.get_unchecked_mut(dest) = value as u32;
    };
    cpu.inc_sp(1);
}
pub fn pop_short(cpu: &mut Cpu) {
    let dest = cpu.next_byte() as usize;
    let value = cpu.memory.short(cpu.sp());
    unsafe {
        *cpu.registers.get_unchecked_mut(dest) = value as u32;
    };
    cpu.inc_sp(2);
}
pub fn pop_long(cpu: &mut Cpu) {
    let dest = cpu.next_byte() as usize;
    let value = cpu.memory.long(cpu.sp());
    unsafe {
        *cpu.registers.get_unchecked_mut(dest) = value as u32;
    };
    cpu.inc_sp(4);
}

pub fn negate_byte(cpu: &mut Cpu) {
    let reg = cpu.next_byte() as usize;
    let val = (cpu.registers[reg] as i8).neg();
    cpu.registers[reg] = val as u32;
}
pub fn negate_short(cpu: &mut Cpu) {
    let reg = cpu.next_byte() as usize;
    let val = (cpu.registers[reg] as i16).neg();
    cpu.registers[reg] = val as u32;
}
pub fn negate_long(cpu: &mut Cpu) {
    let reg = cpu.next_byte() as usize;
    let val = (cpu.registers[reg] as i32).neg();
    cpu.registers[reg] = val as u32;
}

pub fn not_byte(cpu: &mut Cpu) {
    let reg = cpu.next_byte() as usize;
    let val = (cpu.registers[reg] as i8).not();
    cpu.registers[reg] = val as u32;
}
pub fn not_short(cpu: &mut Cpu) {
    let reg = cpu.next_byte() as usize;
    let val = (cpu.registers[reg] as i16).not();
    cpu.registers[reg] = val as u32;
}
pub fn not_long(cpu: &mut Cpu) {
    let reg = cpu.next_byte() as usize;
    let val = (cpu.registers[reg] as i32).not();
    cpu.registers[reg] = val as u32;
}

pub fn increment_byte(cpu: &mut Cpu) {
    let reg = cpu.next_byte() as usize;
    let val = (cpu.registers[reg] as u8).wrapping_add(1);
    cpu.registers[reg] = val as u32;
}
pub fn increment_short(cpu: &mut Cpu) {
    let reg = cpu.next_byte() as usize;
    let val = (cpu.registers[reg] as u16).wrapping_add(1);
    cpu.registers[reg] = val as u32;
}
pub fn increment_long(cpu: &mut Cpu) {
    let reg = cpu.next_byte() as usize;
    let val = (cpu.registers[reg] as u32).wrapping_add(1);
    cpu.registers[reg] = val as u32;
}

pub fn decrement_byte(cpu: &mut Cpu) {
    let reg = cpu.next_byte() as usize;
    let val = (cpu.registers[reg] as u8).wrapping_sub(1);
    cpu.registers[reg] = val as u32;
}
pub fn decrement_short(cpu: &mut Cpu) {
    let reg = cpu.next_byte() as usize;
    let val = (cpu.registers[reg] as u16).wrapping_sub(1);
    cpu.registers[reg] = val as u32;
}
pub fn decrement_long(cpu: &mut Cpu) {
    let reg = cpu.next_byte() as usize;
    let val = (cpu.registers[reg] as u32).wrapping_sub(1);
    cpu.registers[reg] = val as u32;
}

pub fn read_byte(cpu: &mut Cpu) {
    let port = cpu.next_byte() as usize;
    let reg = cpu.next_byte() as usize;
    unsafe {
        let hardware_clone = cpu.hardware[port].clone();
        let hardware = hardware_clone.borrow_mut();
        *cpu.registers.get_unchecked_mut(reg) = hardware.read() as u32;
    }
}
pub fn read_short(cpu: &mut Cpu) {
    let port = cpu.next_byte() as usize;
    let reg = cpu.next_byte() as usize;
    unsafe {
        let hardware_clone = cpu.hardware[port].clone();
        let hardware = hardware_clone.borrow_mut();
        let mut val = hardware.read() as u32;
        val += (hardware.read() as u32) << 8;
        *cpu.registers.get_unchecked_mut(reg) = val;
    }
}
pub fn read_long(cpu: &mut Cpu) {
    let port = cpu.next_byte() as usize;
    let reg = cpu.next_byte() as usize;
    unsafe {
        let hardware_clone = cpu.hardware[port].clone();
        let hardware = hardware_clone.borrow_mut();
        let mut val = hardware.read() as u32;
        val += (hardware.read() as u32) << 8;
        val += (hardware.read() as u32) << 16;
        val += (hardware.read() as u32) << 24;
        *cpu.registers.get_unchecked_mut(reg) = val;
    }
}

pub fn write_byte_imm(cpu: &mut Cpu) {
    let port = cpu.next_byte() as usize;
    let hardware_clone = cpu.hardware[port].clone();
    let mut hardware = hardware_clone.borrow_mut();
    hardware.write(cpu.next_byte());
}
pub fn write_short_imm(cpu: &mut Cpu) {
    let port = cpu.next_byte() as usize;

    let hardware_clone = cpu.hardware[port].clone();
    let mut hardware = hardware_clone.borrow_mut();
    hardware.write(cpu.next_byte());
    hardware.write(cpu.next_byte());
}
pub fn write_long_imm(cpu: &mut Cpu) {
    let port = cpu.next_byte() as usize;

    let hardware_clone = cpu.hardware[port].clone();
    let mut hardware = hardware_clone.borrow_mut();
    hardware.write(cpu.next_byte());
    hardware.write(cpu.next_byte());
    hardware.write(cpu.next_byte());
    hardware.write(cpu.next_byte());
}

pub fn write_byte_reg(cpu: &mut Cpu) {
    let port = cpu.next_byte() as usize;
    let reg = cpu.next_byte() as usize;
    let val = cpu.registers[reg];

    let hardware_clone = cpu.hardware[port].clone();
    let mut hardware = hardware_clone.borrow_mut();
    hardware.write(val as u8);
}
pub fn write_short_reg(cpu: &mut Cpu) {
    let port = cpu.next_byte() as usize;
    let reg = cpu.next_byte() as usize;
    let val = cpu.registers[reg];

    let hardware_clone = cpu.hardware[port].clone();
    let mut hardware = hardware_clone.borrow_mut();
    hardware.write(val as u8);
    hardware.write((val >> 8) as u8);
}
pub fn write_long_reg(cpu: &mut Cpu) {
    let port = cpu.next_byte() as usize;
    let reg = cpu.next_byte() as usize;
    let val = cpu.registers[reg];

    let hardware_clone = cpu.hardware[port].clone();
    let mut hardware = hardware_clone.borrow_mut();
    hardware.write(val as u8);
    hardware.write((val >> 8) as u8);
    hardware.write((val >> 16) as u8);
    hardware.write((val >> 24) as u8);
}

pub fn jump_equal(cpu: &mut Cpu) {
    let addr = cpu.next_long();
    let lhs = cpu.registers[0];
    let rhs = cpu.registers[1];
    if lhs == rhs {
        cpu.registers[IP] = addr;
    }
}
pub fn jump_not_equal(cpu: &mut Cpu) {
    let addr = cpu.next_long();
    let lhs = cpu.registers[0];
    let rhs = cpu.registers[1];
    if lhs != rhs {
        cpu.registers[IP] = addr;
    }
}
pub fn jump_greater(cpu: &mut Cpu) {
    let addr = cpu.next_long();
    unsafe {
        let lhs = cpu.registers.get_unchecked(0);
        let rhs = cpu.registers.get_unchecked(1);
        if lhs > rhs {
            *cpu.registers.get_unchecked_mut(IP) = addr;
        }
    };
}
pub fn jump_greater_equal(cpu: &mut Cpu) {
    let addr = cpu.next_long();
    let lhs = cpu.registers[0];
    let rhs = cpu.registers[1];
    if lhs >= rhs {
        cpu.registers[IP] = addr;
    }
}
pub fn jump_less(cpu: &mut Cpu) {
    let addr = cpu.next_long();
    let lhs = cpu.registers[0];
    let rhs = cpu.registers[1];
    if lhs < rhs {
        cpu.registers[IP] = addr;
    }
}
pub fn jump_less_equal(cpu: &mut Cpu) {
    let addr = cpu.next_long();
    let lhs = cpu.registers[0];
    let rhs = cpu.registers[1];
    if lhs <= rhs {
        cpu.registers[IP] = addr;
    }
}
pub fn jump_signed_greater(cpu: &mut Cpu) {
    let addr = cpu.next_long();
    let lhs = cpu.registers[0];
    let rhs = cpu.registers[1];
    if lhs as i32 > rhs as i32 {
        cpu.registers[IP] = addr;
    }
}
pub fn jump_signed_greater_equal(cpu: &mut Cpu) {
    let addr = cpu.next_long();
    let lhs = cpu.registers[0];
    let rhs = cpu.registers[1];
    if lhs as i32 >= rhs as i32 {
        cpu.registers[IP] = addr;
    }
}
pub fn jump_signed_less(cpu: &mut Cpu) {
    let addr = cpu.next_long();
    let lhs = cpu.registers[0];
    let rhs = cpu.registers[1];
    if (lhs as i32) < rhs as i32 {
        cpu.registers[IP] = addr;
    }
}
pub fn jump_signed_less_equal(cpu: &mut Cpu) {
    let addr = cpu.next_long();
    let lhs = cpu.registers[0];
    let rhs = cpu.registers[1];
    if lhs as i32 <= rhs as i32 {
        cpu.registers[IP] = addr;
    }
}

pub fn jump_imm(cpu: &mut Cpu) {
    let addr = cpu.next_long();
    cpu.registers[IP] = addr;
}
pub fn jump_reg(cpu: &mut Cpu) {
    let index = cpu.next_byte() as usize;
    let addr = cpu.registers[index];
    cpu.registers[IP] = addr;
}

pub fn interrupt(cpu: &mut Cpu) {
    let busy_in_interrupt = (cpu.registers[FLAGS] & Cpu::INTERRUPT_FLAG as u32) != 0;

    // we block interrupts while handling an interrupt.
    // in a more complicated emulator, you wouldn't have this
    // loss of data, but it's complicated and we don't do insane
    // rewinding and reordering of instructions.
    if busy_in_interrupt {
        return;
    }

    let irq = cpu.next_byte() as u32;

    // get the base of the idt
    let idt_base = cpu.registers[IDT] as u32;

    // idt entries are exactly 4 bytes long
    let isr_addr = idt_base + (irq * 4);

    // push return address
    let return_address = cpu.ip();

    cpu.dec_sp(4);
    cpu.memory.set_long(cpu.sp(), return_address as u32);

    // set the interrupt flag
    unsafe {
        *cpu.registers.get_unchecked_mut(FLAGS) |= Cpu::INTERRUPT_FLAG as u32;
    }
    unsafe {
        *cpu.registers.get_unchecked_mut(IP) = cpu.memory.long(isr_addr as usize);
    }
}
pub fn interrupt_return(cpu: &mut Cpu) {
    // clear the interrupt flag
    cpu.registers[FLAGS] &= !(Cpu::INTERRUPT_FLAG as u32);

    // pop return address
    let ret_addr = cpu.memory.long(cpu.sp());

    cpu.inc_sp(4);
    cpu.registers[IP] = ret_addr;
}

pub fn call(cpu: &mut Cpu) {
    cpu.dec_sp(4);
    let addr = cpu.next_long();
    cpu.memory.set_long(cpu.sp(), cpu.ip() as u32);
    cpu.registers[IP] = addr;
}
pub fn ret(cpu: &mut Cpu) {
    let addr = cpu.memory.long(cpu.sp());
    cpu.inc_sp(4);
    cpu.registers[IP] = addr;
}

pub fn syscall(cpu: &mut Cpu) {
    let idx = cpu.next_byte() as usize;
    match idx {
        0 => functions::log_memory(cpu),
        1 => functions::log(cpu),
        2 => functions::print_string(cpu),
        3 => functions::print_register(cpu),
        _ => panic!("invalid rust function: {}", idx),
    }
}
pub fn clear_carry(cpu: &mut Cpu) {
    cpu.set_flag(Cpu::CARRY_FLAG, false);
}

pub fn nop(_: &mut Cpu) {
    // do fricken nothin
}

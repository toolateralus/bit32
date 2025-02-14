use crate::cpu::Cpu;

// pub fn hlt(cpu: &mut Cpu);

pub fn hlt(cpu: &mut Cpu) {
    cpu.set_flag(Cpu::HALT_FLAG, true);
}

pub fn move_imm_reg_byte(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_val = cpu.next_byte();
    cpu.validate_register(dst_reg);
    cpu.registers[dst_reg] = src_val as u32;
}
pub fn move_imm_reg_short(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_val = cpu.next_short();
    cpu.validate_register(dst_reg);
    cpu.registers[dst_reg] = src_val as u32;
}
pub fn move_imm_reg_long(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_val = cpu.next_long();
    cpu.validate_register(dst_reg);
    cpu.registers[dst_reg] = src_val;
}

pub fn move_reg_reg_byte(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_reg = cpu.next_byte() as usize;
    cpu.validate_registers(&[dst_reg, src_reg]);
    cpu.registers[dst_reg] = cpu.registers[src_reg] & 0xFF;
}
pub fn move_reg_reg_short(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_reg = cpu.next_byte() as usize;
    cpu.validate_registers(&[dst_reg, src_reg]);
    cpu.registers[dst_reg] = cpu.registers[src_reg] & 0xFFFF;
}
pub fn move_reg_reg_long(cpu: &mut Cpu) {
    let dst_reg = cpu.next_byte() as usize;
    let src_reg = cpu.next_byte() as usize;
    cpu.validate_registers(&[dst_reg, src_reg]);
    cpu.registers[dst_reg] = cpu.registers[src_reg];
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
    let lhs = (cpu.registers[0] & 0xFF) as u8;
    let rhs = cpu.next_byte();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.registers[0] = result as u32;
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

use crate::cpu::Cpu;
use crate::opcodes::Opcode;
use std::fs::{File, OpenOptions};
use std::io::Write;

pub fn get_or_create_log_file() -> File {
    OpenOptions::new()
        .append(true)
        .create(true)
        .open("logs.txt")
        .unwrap()
}

pub fn log(cpu: &mut Cpu) {
    let addr = cpu.registers[0];
    let string = cpu.memory.utf8(addr as usize).unwrap();
    let mut file = get_or_create_log_file();
    writeln!(file, "{}", string).unwrap();
}

pub fn log_opcode(opcode: &Opcode) {
    let mut file = get_or_create_log_file();
    writeln!(file, "{:?}", opcode).unwrap();
}

pub fn log_memory(cpu: &mut Cpu) {
    let start_idx = cpu.registers[0] as usize;
    let end_idx = cpu.registers[1] as usize;
    let mut file = get_or_create_log_file();
    if start_idx > end_idx {
        writeln!(
            file,
            "log memory got invalid start and end index. {} to {}",
            start_idx, end_idx
        )
        .unwrap();
        return;
    }
    let range = &cpu.memory.buffer[start_idx..end_idx];
    writeln!(file, "memory at {} to {}, {:?}", start_idx, end_idx, range).unwrap();
}

pub fn print_string(cpu: &mut Cpu) {
    let addr = cpu.registers[0];
    let string = cpu.memory.utf8(addr as usize).unwrap();
    println!("{}", string);
}

pub fn print_register(cpu: &mut Cpu) {
    let reg = cpu.registers[0] as usize;
    cpu.validate_register(reg);
    println!("{}", cpu.registers[reg]);
}

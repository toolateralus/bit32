use crate::cpu::Cpu;
use std::fs::OpenOptions;
use std::io::Write;

pub fn log_memory(cpu: &mut Cpu) {
    let start_idx = cpu.registers[0] as usize;
    let end_idx = cpu.registers[1] as usize;
    let range = &cpu.memory.buffer[start_idx..end_idx];
    
    if let Ok(mut file) = OpenOptions::new().append(true).open("logs.txt") {
        if let Err(err) = writeln!(file, "memory at {} to {}, {:?}", start_idx, end_idx, range) {
            eprintln!("Failed to write to file: {}", err);
        }
    } else {
        eprintln!("Failed to open file");
    }
}

use crate::cpu::Cpu;
use std::fs::OpenOptions;
use std::io::Write;

pub fn print(cpu: &mut Cpu) {
    let string = cpu.next_utf8().unwrap();
    let mut file = OpenOptions::new().append(true).open("logs.txt").unwrap();
    writeln!(file, "{}", string).unwrap();
}
pub fn log_memory(cpu: &mut Cpu) {
    let start_idx = cpu.registers[0] as usize;
    let end_idx = cpu.registers[1] as usize;
    
    let mut file = OpenOptions::new().append(true).open("logs.txt").unwrap();
    if start_idx > end_idx {
        writeln!(file, "log memory got invalid start and end index. {} to {}", start_idx, end_idx).unwrap();    
        return;
    }
    
    let range = &cpu.memory.buffer[start_idx..end_idx];
    writeln!(file, "memory at {} to {}, {:?}", start_idx, end_idx, range).unwrap();
}

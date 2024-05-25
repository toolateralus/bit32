use crate::cpu::Cpu;

pub mod cpu;
pub mod opcodes;
pub mod test;

fn main() {
    let mut cpu = Cpu::new();
    cpu.load_program_from_file("../asm32/test.o").unwrap();
    
    cpu.run();
    println!("Cpu halted: \n{:?}", cpu);
}

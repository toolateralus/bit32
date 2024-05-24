use crate::cpu::Cpu;

pub mod cpu;
pub mod opcodes;
pub mod test;

fn main() {
    let mut cpu = Cpu::new();
    cpu.run();
    println!("Cpu halted: \n{:?}", cpu);
}

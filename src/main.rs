use std::env;

use cpu::Cpu;
use debug::Debugger;

pub mod cpu;
pub mod debug;
pub mod opcodes;
pub mod test;
pub mod functions;

fn main() {
    let file = "../asm32/test.o";
    
    if env::args().len() > 1 && env::args().nth(1).unwrap() == "g" {
        let mut debugger = Debugger {
            file: String::new(),
        };
        debugger.run(file);
    } else {
        let mut cpu = Cpu::new();
        cpu.load_program_from_file(file).unwrap();
        cpu.run();
    }
    
}

use debug::Debugger;

use crate::cpu::Cpu;

pub mod cpu;
pub mod opcodes;
pub mod test;
pub mod debug;

fn main() {
    let file = "../asm32/test.o";
    let mut debugger = Debugger{
        file: String::new(),
    };
    debugger.run(file);
}

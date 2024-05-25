use debug::Debugger;

pub mod cpu;
pub mod debug;
pub mod opcodes;
pub mod test;

fn main() {
    let file = "../asm32/test.o";
    let mut debugger = Debugger {
        file: String::new(),
    };
    debugger.run(file);
}

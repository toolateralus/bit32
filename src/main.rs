use cpu::Cpu;
use hardware::Hardware;
use std::env;
use std::io::stdout;
use std::time::Instant;

use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{cursor, execute};
use debug::Debugger;

pub mod cpu;
pub mod debug;
pub mod functions;
pub mod hardware;
pub mod opcodes;
pub mod test;
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
        
        execute!(stdout(), EnterAlternateScreen).unwrap();
        execute!(stdout(), cursor::Hide).unwrap();
        
        let mut cycles = 0usize;
        let start = Instant::now();
        
        const CYCLES_PER_FRAME: usize = 15_000_000 / 60;
        
        while (cpu.flags() & Cpu::HALT_FLAG) != Cpu::HALT_FLAG {
            cpu.cycle();
            
            if cycles >= CYCLES_PER_FRAME * 2 {
                Hardware::draw_vga_buffer(&cpu);
                //Hardware::handle_input(&mut cpu);
                cycles = 0;
            }
            
            cycles += 1;
        }
        
        let elapsed = start.elapsed();
        let seconds = elapsed.as_secs_f64();
        let clock_speed_hz = cycles as f64 / seconds;
        
        execute!(stdout(), LeaveAlternateScreen).unwrap();
        println!("Average CPU clock speed: {:.2} Hz", clock_speed_hz);
    }
}
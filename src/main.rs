use cpu::Cpu;
use hardware::Hardware;
use std::env;
use std::io::stdout;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

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
        let cpu = Arc::new(RwLock::new(Cpu::new()));
        let running = Arc::new(AtomicBool::new(true));
        
        {
            let mut cpu = cpu.write().unwrap();
            cpu.load_program_from_file(file).unwrap();
        }
        
        let cpu_clone = Arc::clone(&cpu);
        let running_clone = Arc::clone(&running);
        execute!(stdout(), EnterAlternateScreen).unwrap();
        execute!(stdout(), cursor::Hide).unwrap();
        
        let draw_handle = thread::spawn(move || {
            while running_clone.load(Ordering::SeqCst) {
                thread::sleep(Duration::from_millis(16));
                Hardware::draw_vga_buffer(cpu_clone.clone());
                Hardware::handle_input(cpu_clone.clone());
            }
        });
        
        let cpu_self_clone = Arc::clone(&cpu);
        let running_self_clone = Arc::clone(&running);
        
        let cpu_handle = thread::spawn(move || loop {
            {
                let cpu = cpu_self_clone.read().unwrap();
                if (cpu.flags() & Cpu::HALT_FLAG) == Cpu::HALT_FLAG {
                    running_self_clone.store(false, Ordering::SeqCst);
                    break;
                }
            }
            cpu_self_clone.write().unwrap().cycle();
        });
        
        cpu_handle.join().unwrap();
        draw_handle.join().unwrap();

        execute!(stdout(), LeaveAlternateScreen).unwrap();
    }
}
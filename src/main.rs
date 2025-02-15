use cpu::Cpu;
use std::env::{self};
use std::io::stdout;
use std::time::Instant;

use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{cursor, execute};
use debug::Debugger;

pub mod handlers;
pub mod cpu;
pub mod debug;
pub mod functions;
pub mod hardware;
pub mod opcodes;
pub mod test;
pub mod graphical;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let file = args[1].clone();


    if args.contains(&String::from("debug")) {
        let mut debugger = Debugger {
            file: file.clone(),
        };
        std::panic::set_hook(Box::new( |info| {
            execute!(stdout(), LeaveAlternateScreen).unwrap();
            execute!(stdout(), cursor::Show).unwrap();
            println!("{}", info);
        }));
        debugger.run(&file);
    } else if args.contains(&String::from("graphical")) {
       
    } else {
        let mut cpu = Cpu::new();
        cpu.load_program_from_file(&file).unwrap();

        let start = Instant::now();
        let mut cycles = 0;
        while !cpu.has_flag(Cpu::HALT_FLAG) {
            cpu.cycle();
            cycles += 1;
        }

        let elapsed = start.elapsed();
        let seconds = elapsed.as_secs_f64();
        let clock_speed_hz = cycles as f64 / seconds;
        println!("Average CPU clock speed: {:.2} Mhz", clock_speed_hz / 1_000_000.0);
    }
}

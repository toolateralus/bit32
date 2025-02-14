use cpu::Cpu;
use hardware::Hardware;
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
fn main() {
    let args = env::args().collect::<Vec<String>>();
    let file = args[1].clone();


    if args.contains(&String::from("debug")) {
        let mut debugger = Debugger {
            file: file.clone(),
        };
        std::panic::set_hook(Box::new(move |info| {
            execute!(stdout(), LeaveAlternateScreen).unwrap();
            execute!(stdout(), cursor::Show).unwrap();
            println!("{}", info);
        }));
        debugger.run(&file);
    } else if args.contains(&String::from("graphical")) {
        std::panic::set_hook(Box::new(move |info| {
            execute!(stdout(), LeaveAlternateScreen).unwrap();
            execute!(stdout(), cursor::Show).unwrap();
            println!("{}", info);
        }));

        let mut cpu = Cpu::new();
        cpu.load_program_from_file(&file).unwrap();

        execute!(stdout(), EnterAlternateScreen).unwrap();
        execute!(stdout(), cursor::Hide).unwrap();

        let mut cycles = 0usize;
        let start = Instant::now();

        const CYCLES_PER_FRAME: usize = 15_000_000 / 60;

        while !cpu.has_flag(Cpu::HALT_FLAG) {
            cpu.cycle();

            if cycles >= CYCLES_PER_FRAME * 2 {
                Hardware::draw_vga_buffer(&cpu);
                Hardware::handle_input(&mut cpu);
                cycles = 0;
            }

            cycles += 1;
        }

        let elapsed = start.elapsed();
        let seconds = elapsed.as_secs_f64();
        let clock_speed_hz = cycles as f64 / seconds;

        execute!(stdout(), LeaveAlternateScreen).unwrap();
        execute!(stdout(), cursor::Show).unwrap();
        println!("Average CPU clock speed: {:.2} Hz", clock_speed_hz);
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
        println!("Average CPU clock speed: {:.2} Hz", clock_speed_hz);
    }
}

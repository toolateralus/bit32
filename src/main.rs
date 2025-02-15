use cpu::Cpu;
use hardware::{Config, Hardware};
use std::cell::RefCell;
use std::env::{self};
use std::io::stdout;
use std::rc::Rc;
use std::time::Instant;

use crossterm::terminal::LeaveAlternateScreen;
use crossterm::{cursor, execute};
use debug::Debugger;

pub mod cpu;
pub mod debug;
pub mod functions;
pub mod graphical;
pub mod handlers;
pub mod hardware;
pub mod opcodes;
pub mod test;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let file = args[1].clone();

    if args.contains(&String::from("debug")) {
        let mut debugger = Debugger { file: file.clone() };
        std::panic::set_hook(Box::new(|info| {
            execute!(stdout(), LeaveAlternateScreen).unwrap();
            execute!(stdout(), cursor::Show).unwrap();
            println!("{}", info);
        }));
        debugger.run(&file);
    } else if args.contains(&String::from("graphical")) {
        let cpu = Rc::new(RefCell::new(Cpu::new()));
        let gpu = Rc::new(RefCell::new(graphical::GPU::new()));
        let cfg = Config {
            cpu: cpu.clone(),
            id: 0,
        };
        gpu.clone().borrow_mut().init(cfg);
        let cpu = cpu.clone();
        cpu.borrow_mut().hardware.push(gpu.clone());
        cpu.borrow_mut().load_program_from_file(&file).unwrap();
        let start = Instant::now();
        let mut cycles = 0;
        while !cpu.borrow_mut().has_flag(Cpu::HALT_FLAG) {
            cpu.borrow_mut().cycle();
            cycles += 1;
        }

        let elapsed = start.elapsed();
        let seconds = elapsed.as_secs_f64();
        let clock_speed_hz = cycles as f64 / seconds;
        if clock_speed_hz >= 1_000_000.0 {
            println!(
            "Average CPU clock speed: {:.2} Mhz",
            clock_speed_hz / 1_000_000.0
            );
        } else if clock_speed_hz >= 1_000.0 {
            println!(
            "Average CPU clock speed: {:.2} Khz",
            clock_speed_hz / 1_000.0
            );
        } else {
            println!(
            "Average CPU clock speed: {:.2} Hz",
            clock_speed_hz
            );
        }
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
        println!(
            "Average CPU clock speed: {:.2} Mhz",
            clock_speed_hz / 1_000_000.0
        );
    }
}

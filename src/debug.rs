use crossterm::{
    cursor,
    event::{self, KeyModifiers},
    execute, queue,
    style::Print,
    terminal,
};
use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

use crate::cpu::{Cpu, Memory};
use crossterm::event::{Event, KeyCode};

pub enum DebugState {
    Executing,
    Pause,
    Step,
    Continue,
    Reset,
    Abort,
}

pub struct Debugger {
    pub file: String,
}
impl Debugger {
    pub fn input(&self, state: &mut DebugState) {
        if event::poll(Duration::from_secs(0)).unwrap() {
            let event = event::read().unwrap();

            if let Event::Resize(_, _) = event {
                execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
            } else if let Event::Key(key_event) = event {
                *state = match key_event.code {
                    KeyCode::Char('1') => DebugState::Executing,
                    KeyCode::Char('2') => DebugState::Pause,
                    KeyCode::Char('3') => DebugState::Step,
                    KeyCode::Char('4') => DebugState::Continue,
                    KeyCode::Char('5') => DebugState::Reset,
                    KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => {
                        DebugState::Abort
                    }
                    _ => return,
                };
            }
        }
    }

    pub fn run(&mut self, file: &str) {
        let mut cpu = Cpu::new();
        self.file = file.to_string();

        cpu.load_program_from_file(file).unwrap();
        let mut stdout = stdout();
        let _raw = terminal::enable_raw_mode().unwrap();
        execute!(stdout, cursor::Hide).unwrap();
        execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

        let mut state: DebugState = DebugState::Executing;
        let start_time = std::time::Instant::now();
        let mut cycle_count = 0;

        while (cpu.flags() & Cpu::HALT_FLAG) != Cpu::HALT_FLAG {
            cpu.cycle();
            cycle_count += 1;
            self.input(&mut state);
            match state {
                DebugState::Executing | DebugState::Step | DebugState::Continue => {}
                DebugState::Pause => {
                    self.pause(&mut cpu, &mut state);
                }

                DebugState::Reset => {}
                DebugState::Abort => break,
            }
            let elapsed_time = start_time.elapsed();
            let elapsed_seconds = elapsed_time.as_secs_f64();
            let mhz = cycle_count as f64 / elapsed_seconds / 1_000_000.0; // Changed to MHz
            self.display_registers(&cpu, mhz); // Now displaying MHz
       
        }

        execute!(stdout, cursor::Show).unwrap();
        terminal::disable_raw_mode().unwrap();
        println!("\n\x1b[;032]RESULT::\n\n\t{:?}", cpu);
    }

    pub fn pause(&self, cpu: &mut Cpu, state: &mut DebugState) {
        loop {
            self.input(state);
            match state {
                DebugState::Step => {
                    cpu.cycle();
                    self.display_registers(cpu, 0.0);
                    *state = DebugState::Pause;
                    continue;
                }
                DebugState::Continue => {
                    return;
                }
                DebugState::Reset => {
                    cpu.registers = [0; 21];
                    cpu.memory = Memory::new();
                    cpu.load_program_from_file(self.file.as_str()).unwrap();
                    *state = DebugState::Pause;
                    return;
                }
                DebugState::Abort => {
                    return;
                }
                DebugState::Executing => {
                    return;
                }
                DebugState::Pause => {}
            }
            thread::sleep(Duration::from_millis(16));
        }
    }

    pub fn display_registers(&self, cpu: &Cpu, ghz : f64) {
        let mut stdout = stdout();
        for (i, register) in cpu.registers.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, i as u16)).unwrap();
            queue!(
                stdout,
                Print(format!(
                    "\x1b[1;96m{}\x1b[1;97m: {}\r",
                    Cpu::reg_index_to_str(&i),
                    register
                ))
            ).unwrap();
            
        }
        queue!(
            stdout, 
            Print(
                format!("CPU Speed: {:.2} MHz", ghz)
            )
        ).unwrap();
        
        stdout.flush().unwrap();
    }
}

use crossterm::{
    cursor,
    event::{self, KeyModifiers},
    execute, queue,
    style::{self, Print},
    terminal::{self},
};
use std::{
    io::{stdout, Stdout, Write},
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

        let mut state: DebugState = DebugState::Pause;
        let start_time = std::time::Instant::now();
        let mut cycle_count = 0;

        while (cpu.flags() & Cpu::HALT_FLAG) != Cpu::HALT_FLAG {
            execute!(stdout, cursor::MoveTo(0,25)).unwrap();
            self.display_legend();
            execute!(stdout, cursor::MoveTo(0,0)).unwrap();
            cpu.cycle();
            cycle_count += 1;
            self.input(&mut state);
            match state {
                DebugState::Executing | DebugState::Step | DebugState::Continue => {}
                DebugState::Pause => {
                    self.pause(&mut cpu, &mut state, &mut stdout);
                }

                DebugState::Reset => {}
                DebugState::Abort => break,
            }
            let elapsed_time = start_time.elapsed();
            let elapsed_seconds = elapsed_time.as_secs_f64();
            let mhz = cycle_count as f64 / elapsed_seconds / 1_000_000.0;
            self.display_registers(&cpu, mhz);
        }

        execute!(stdout, cursor::Show).unwrap();
        terminal::disable_raw_mode().unwrap();
        println!("\n\x1b[;032]RESULT::\n\n\t{:?}", cpu);
    }

    pub fn pause(&self, cpu: &mut Cpu, state: &mut DebugState, stdout: &mut Stdout) {
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
                    cpu.registers = [0; 22];
                    cpu.memory = Memory::new();
                    cpu.load_program_from_file(self.file.as_str()).unwrap();
                    *state = DebugState::Pause;
                    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
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

    pub fn display_registers(&self, cpu: &Cpu, clock_speed_mhz: f64) {
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
            )
            .unwrap();
        }
        queue!(
            stdout,
            Print(format!("CPU Speed: {:.2} MHz", clock_speed_mhz))
        )
        .unwrap();

        stdout.flush().unwrap();
    }
    pub fn display_legend(&self) {
        let mut stdout = stdout();
        execute!(
            stdout,
            style::SetForegroundColor(style::Color::White),
            style::Print("Controls:\n"),
            style::SetForegroundColor(style::Color::White),
            style::Print("["),
            style::SetForegroundColor(style::Color::Cyan),
            style::SetAttribute(style::Attribute::Bold),
            style::Print("1"),
            style::SetAttribute(style::Attribute::Reset),
            style::SetForegroundColor(style::Color::White),
            style::Print("]"),
            style::SetForegroundColor(style::Color::Cyan),
            style::Print(" Execute "),
            style::SetForegroundColor(style::Color::White),
            style::Print("["),
            style::SetForegroundColor(style::Color::Cyan),
            style::SetAttribute(style::Attribute::Bold),
            style::Print("2"),
            style::SetAttribute(style::Attribute::Reset),
            style::SetForegroundColor(style::Color::White),
            style::Print("]"),
            style::SetForegroundColor(style::Color::Cyan),
            style::Print(" Pause "),
            style::SetForegroundColor(style::Color::White),
            style::Print("["),
            style::SetForegroundColor(style::Color::Cyan),
            style::SetAttribute(style::Attribute::Bold),
            style::Print("3"),
            style::SetAttribute(style::Attribute::Reset),
            style::SetForegroundColor(style::Color::White),
            style::Print("]"),
            style::SetForegroundColor(style::Color::Cyan),
            style::Print(" Step "),
            style::SetForegroundColor(style::Color::White),
            style::Print("["),
            style::SetForegroundColor(style::Color::Cyan),
            style::SetAttribute(style::Attribute::Bold),
            style::Print("4"),
            style::SetAttribute(style::Attribute::Reset),
            style::SetForegroundColor(style::Color::White),
            style::Print("]"),
            style::SetForegroundColor(style::Color::Cyan),
            style::Print(" Continue "),
            style::SetForegroundColor(style::Color::White),
            style::Print("["),
            style::SetForegroundColor(style::Color::Cyan),
            style::SetAttribute(style::Attribute::Bold),
            style::Print("5"),
            style::SetAttribute(style::Attribute::Reset),
            style::SetForegroundColor(style::Color::White),
            style::Print("]"),
            style::SetForegroundColor(style::Color::Cyan),
            style::Print(" Reset "),
            style::SetForegroundColor(style::Color::White),
            style::Print("["),
            style::SetForegroundColor(style::Color::Cyan),
            style::SetAttribute(style::Attribute::Bold),
            style::Print("Ctrl+C"),
            style::SetAttribute(style::Attribute::Reset),
            style::SetForegroundColor(style::Color::White),
            style::Print("]"),
            style::SetForegroundColor(style::Color::Cyan),
            style::Print(" Abort\n"),
            style::ResetColor
        )
        .unwrap();

        stdout.flush().unwrap();
    }
}

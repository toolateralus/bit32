use crossterm::{
    cursor,
    event::{self, KeyModifiers},
    execute, queue,
    style::{self, Print},
    terminal::{self},
};
use std::{
    io::{stdout, Write}, time::Duration
};

use crate::{cpu::{Cpu, IP}, opcodes::Opcode};
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
        
        let mut state: DebugState = DebugState::Pause;
        execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
        
        while !cpu.has_flag(Cpu::HALT_FLAG) {

            execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
            self.display_registers(&cpu);
            execute!(stdout, cursor::MoveTo(0, 25)).unwrap();
            self.display_legend();

            self.input(&mut state);
            match state {
                DebugState::Step => {
                    state = DebugState::Pause;
                }
                DebugState::Reset => {
                    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
                    cpu = Cpu::new();
                    cpu.load_program_from_file(self.file.as_str()).unwrap();
                    state = DebugState::Pause;
                    continue;
                }
                DebugState::Abort => {
                    break;
                }
                DebugState::Continue |
                DebugState::Executing => {}
                DebugState::Pause => {
                    continue;
                }
            }

            cpu.cycle();
        }

        execute!(stdout, cursor::Show).unwrap();
        terminal::disable_raw_mode().unwrap();
        println!("\n\x1b[;032]RESULT::\n\n\t{:?}", cpu);
    }

    pub fn display_registers(&self, cpu: &Cpu) {
        let mut stdout = stdout();
        for (i, register) in cpu.registers.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, i as u16)).unwrap();
            queue!(
                stdout,
                Print(format!(
                    "\x1b[1;96m{}\x1b[1;97m: {} (0x{:X}){}\r",
                    Cpu::reg_index_to_str(&i),
                    register,
                    register,
                    "           "
                ))
            )
            .unwrap();
        }
        let next_i = cpu.memory.buffer[cpu.registers[IP] as usize];
        let next_i_str = if next_i < Opcode::Nop as u8 {
            format!("{:?}", Opcode::from(next_i))
        } else {
            format!("Invalid Opcode: {}", next_i)
        };
        queue!(
            stdout,
            Print(format!(
                "\x1b[1;96m{}\x1b[1;97m: {}{}\r",
                "Next Instruction:",
                next_i_str,
                "           "
            ))
        ).unwrap();

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

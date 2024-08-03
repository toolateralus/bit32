use std::env;
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;
use tokio::time::{self, Duration};

use cpu::Cpu;
use crossterm::event::{self, Event, KeyCode};
use crossterm::{
    queue,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear},
};
use debug::Debugger;
use std::io::{stdout, Write};

pub mod cpu;
pub mod debug;
pub mod functions;
pub mod opcodes;
pub mod test;

struct Hardware {}

impl Hardware {
    fn vga_color_to_crossterm_color(vga_color: u8) -> Color {
        match vga_color {
            0x0 => Color::Black,
            0x1 => Color::Blue,
            0x2 => Color::Green,
            0x3 => Color::Cyan,
            0x4 => Color::Red,
            0x5 => Color::Magenta,
            0x6 => Color::Yellow,
            0x7 => Color::White,
            0x8 => Color::DarkGrey,
            0x9 => Color::DarkBlue,
            0xA => Color::DarkGreen,
            0xB => Color::DarkCyan,
            0xC => Color::DarkRed,
            0xD => Color::DarkMagenta,
            0xE => Color::DarkYellow,
            0xF => Color::Grey,
            _ => Color::Black,
        }
    }

    pub async fn draw_vga_buffer(cpu: Arc<TokioMutex<Cpu>>) {
        let cpu = cpu.lock().await;
        queue!(stdout(), Clear(terminal::ClearType::All)).unwrap();
        let slice = &cpu.memory.buffer
            [Cpu::VGA_BUFFER_ADDRESS..Cpu::VGA_BUFFER_ADDRESS + Cpu::VGA_BUFFER_LEN];
        for chunk in slice.chunks(2) {
            if chunk.len() == 2 {
                let ch = chunk[0] as char;
                let color = chunk[1];
                let fg_color = Self::vga_color_to_crossterm_color(color & 0x0F);
                let bg_color = Self::vga_color_to_crossterm_color((color >> 4) & 0x0F);
                queue!(
                    stdout(),
                    SetForegroundColor(fg_color),
                    SetBackgroundColor(bg_color),
                    Print(ch)
                )
                .unwrap();
            }
        }
        stdout().flush().unwrap();
    }

    pub async fn handle_input(cpu: Arc<TokioMutex<Cpu>>) {
        if event::poll(Duration::from_millis(10)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Char(c) => {
                        cpu.lock().await.hardware_interrupt_routine =
                            Some(Box::new(move |cpu: &mut Cpu| {
                                cpu.registers[0] = c.into();
                            }));
                    }
                    _ => {}
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let file = "../asm32/test.o";

    if env::args().len() > 1 && env::args().nth(1).unwrap() == "g" {
        let mut debugger = Debugger {
            file: String::new(),
        };
        debugger.run(file);
    } else {
        let cpu = Arc::new(TokioMutex::new(Cpu::new()));

        {
            let mut cpu = cpu.lock().await;
            cpu.load_program_from_file(file).unwrap();
        }

        let cpu_clone = Arc::clone(&cpu);
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_millis(16));
            loop {
                interval.tick().await;
                Hardware::draw_vga_buffer(cpu_clone.clone()).await;
                Hardware::handle_input(cpu_clone.clone()).await;
            }
        });

        cpu.lock().await.run();
    }
}

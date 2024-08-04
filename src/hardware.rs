use std::{io::{stdout, Write}, sync::{Arc, RwLock}, time::Duration};

use crossterm::{cursor, event::{self, Event, KeyCode}, execute, style::{Color, Print, SetBackgroundColor, SetForegroundColor}, terminal::{self, Clear}};

use crate::cpu::Cpu;

pub struct Hardware;

impl Hardware {
  pub fn vga_color_to_crossterm_color(vga_color: u8) -> Color {
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

  pub fn draw_vga_buffer(cpu: Arc<RwLock<Cpu>>) {
      let cpu = cpu.read().unwrap();
      execute!(stdout(), Clear(terminal::ClearType::All)).unwrap();
      let slice = &cpu.memory.buffer[Cpu::VGA_BUFFER_ADDRESS..Cpu::VGA_BUFFER_ADDRESS + Cpu::VGA_BUFFER_LEN];
      let mut x = 0;
      let mut y = 0;
      for chunk in slice.chunks(2) {
          if chunk.len() == 2 {
              let ch = chunk[0] as char;
              let color = chunk[1];
              let fg_color = Self::vga_color_to_crossterm_color(color & 0x0F);
              let bg_color = Self::vga_color_to_crossterm_color((color >> 4) & 0x0F);
              execute!(
                  stdout(),
                  cursor::MoveTo(x, y),
                  SetForegroundColor(fg_color),
                  SetBackgroundColor(bg_color),
                  Print(ch)
              )
              .unwrap();
              x += 1;
              if x >= 80 { // Assuming 80 columns per row
                  x = 0;
                  y += 1;
              }
          }
      }
      stdout().flush().unwrap();
  }
  
  pub fn handle_input(cpu: Arc<RwLock<Cpu>>) {
      if event::poll(Duration::from_millis(0)).unwrap() {
          if let Event::Key(key_event) = event::read().unwrap() {
              match key_event.code {
                  KeyCode::Char(c) => {
                      cpu.write().unwrap().hardware_interrupt_routine =
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

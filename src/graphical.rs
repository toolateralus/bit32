use raylib::{color::Color, ffi::TraceLogLevel, init, prelude::RaylibDraw, RaylibHandle, RaylibThread};

use crate::{
    cpu::Cpu,
    hardware::{Config, Hardware, Numeric},
};

pub struct GfxContext {
    pub raylib: (RaylibHandle, RaylibThread),
    pub vga_buffer: [u8; Cpu::VGA_BUFFER_LEN],
    pub cfg: Option<Config>,
}

impl GfxContext {
    pub fn vga_to_raylib_color(vga_color: u8) -> Color {
        match vga_color {
            0x0 => Color::BLACK,
            0x1 => Color::BLUE,
            0x2 => Color::GREEN,
            0x3 => Color::CYAN,
            0x4 => Color::RED,
            0x5 => Color::MAGENTA,
            0x6 => Color::YELLOW,
            0x7 => Color::WHITE,
            0x8 => Color::DARKGRAY,
            0x9 => Color::DARKBLUE,
            0xA => Color::DARKGREEN,
            0xB => Color::DARKCYAN,
            0xC => Color::DARKRED,
            0xD => Color::DARKMAGENTA,
            0xE => Color::YELLOWGREEN,
            0xF => Color::GRAY,
            _ => Color::BLACK,
        }
    }

    pub fn new() -> Self {
        Self {
            vga_buffer: [0; Cpu::VGA_BUFFER_LEN],
            cfg: None,
            raylib: init().log_level(TraceLogLevel::LOG_NONE).build(),
        }
    }
}

impl Hardware for GfxContext {
    fn init(&mut self, cfg: Config) {
        self.cfg = Some(cfg);
    }

    fn read<T: Numeric>(&self) -> T {
        todo!()
    }

    fn write<T: Numeric>(&mut self, _b: T) {
        todo!()
    }
}

impl GfxContext {
    pub fn draw(&mut self) {
        let (ref mut window, ref thread) = self.raylib;
        let mut handle = window.begin_drawing(thread);
        let mut x = 0;
        let mut y = 0;
        let buffer = &self.vga_buffer;
        for chunk in buffer.chunks(2) {
            if chunk.len() == 2 {
                let ch = chunk[0] as char;
                let color = chunk[1];
                let fg_color = GfxContext::vga_to_raylib_color(color & 0x0F);
                let bg_color = GfxContext::vga_to_raylib_color((color >> 4) & 0x0F);
                handle.draw_rectangle(x * 8, y * 16, 8, 16, bg_color);
                handle.draw_text(&ch.to_string(), x * 8, y * 16, 16, fg_color);
                x += 1;
                if x >= 80 {
                    x = 0;
                    y += 1;
                }
            }
        }
    }
}

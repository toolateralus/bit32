use raylib::{
    color::Color,
    ffi::{CloseWindow, TraceLogLevel, WindowShouldClose},
    init,
    prelude::RaylibDraw,
    window::WindowState,
    RaylibHandle, RaylibThread,
};

use crate::hardware::{Config, Hardware};

pub struct GPU {
    pub raylib: (RaylibHandle, RaylibThread),
    pub vram: [u8; GPU::VRAM_SIZE],
    pub cfg: Option<Config>,
    instruction_size: usize,
    instruction_buffer_ptr: usize,
    instruction_buffer: [u8; 7],
}

impl GPU {
    const VRAM_SIZE: usize = 0x10000;
    const HLT: u8 = 0;
    const DRAW_VGA: u8 = GPU::HLT + 1;
    const WRITE_BYTE: u8 = GPU::DRAW_VGA + 1;
    const WRITE_SHORT: u8 = GPU::WRITE_BYTE + 1;
    const WRITE_LONG: u8 = GPU::WRITE_SHORT + 1;
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
            vram: [0; GPU::VRAM_SIZE],
            cfg: None,
            raylib: init().log_level(TraceLogLevel::LOG_NONE).build(),
            instruction_size: 0,
            instruction_buffer_ptr: 0,
            instruction_buffer: [0; 7],
        }
    }
}

impl Hardware for GPU {
    fn init(&mut self, cfg: Config) {
        self.cfg = Some(cfg);
        let mut i = 0;
        for byte in self.vram.iter_mut() {
            *byte = if i % 2 == 0 { b' ' } else { 0x70 };
            i += 1;
        }
    }

    fn read(&self) -> u8 {
        return 0;
    }

    fn write(&mut self, data: u8) {
        self.instruction_buffer[self.instruction_buffer_ptr] = data;
        self.instruction_buffer_ptr += 1;
        if self.instruction_size == 0 {
            match data {
                GPU::HLT => self.instruction_size = 1,
                GPU::DRAW_VGA => self.instruction_size = 1,
                GPU::WRITE_BYTE => self.instruction_size = 4,
                GPU::WRITE_SHORT => self.instruction_size = 5,
                GPU::WRITE_LONG => self.instruction_size = 7,
                _ => panic!("Unknown gpu instruction: {}", data),
            }
        }
        if self.instruction_buffer_ptr == self.instruction_size {
            match self.instruction_buffer[0] {
                GPU::HLT => self.deinit(),
                GPU::DRAW_VGA => self.draw(),
                GPU::WRITE_BYTE => {
                    let dest = self.instruction_buffer[1] as usize
                        + ((self.instruction_buffer[2] as usize) << 8);
                    self.vram[dest] = self.instruction_buffer[3];
                }
                GPU::WRITE_SHORT => {
                    let dest = self.instruction_buffer[1] as usize
                        + ((self.instruction_buffer[2] as usize) << 8);
                    self.vram[dest + 0] = self.instruction_buffer[3];
                    self.vram[dest + 1] = self.instruction_buffer[4];
                }
                GPU::WRITE_LONG => {
                    let dest = self.instruction_buffer[1] as usize
                        + ((self.instruction_buffer[2] as usize) << 8);
                    self.vram[dest + 0] = self.instruction_buffer[3];
                    self.vram[dest + 1] = self.instruction_buffer[4];
                    self.vram[dest + 2] = self.instruction_buffer[5];
                    self.vram[dest + 3] = self.instruction_buffer[6];
                }
                _ => panic!("Unknown gpu instruction: {}", self.instruction_buffer[0]),
            }
            self.instruction_size = 0;
            self.instruction_buffer_ptr = 0;
        }
    }

    fn deinit(&mut self) {
        unsafe {
            CloseWindow();
        }
    }
}

impl GPU {
    pub fn draw(&mut self) {
        let (ref mut window, ref thread) = self.raylib;
        let mut handle = window.begin_drawing(thread);
        let mut x = 0;
        let mut y = 0;
        let buffer = &self.vram;
        for chunk in buffer.chunks(2) {
            if chunk.len() == 2 {
                let ch = chunk[0] as char;
                let color = chunk[1];
                let fg_color = GPU::vga_to_raylib_color(color & 0x0F);
                let bg_color = GPU::vga_to_raylib_color((color >> 4) & 0x0F);
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

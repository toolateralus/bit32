use std::{
    sync::mpsc::{channel, Sender},
    thread::{self, JoinHandle},
};

use raylib::{
    color::Color,
    ffi::{CloseWindow, TraceLogLevel, Vector2},
    init,
    prelude::RaylibDraw,
    text::WeakFont,
    RaylibHandle, RaylibThread,
};

use crate::hardware::{Config, Hardware};

struct Core {
    vram: [u8; GPU::VRAM_SIZE],
    font: WeakFont,
    window: RaylibHandle,
    thread: RaylibThread,
}

pub struct GPU {
    thread_handle: Option<JoinHandle<()>>,
    sender: Sender<[u8; 9]>,
    pub cfg: Option<Config>,
    instruction_size: usize,
    instruction_buffer_ptr: usize,
    instruction_buffer: [u8; 9],
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
        let (sender, receiver) = channel::<[u8; 9]>();
        let thread_handle = thread::spawn(move || {
            let raylib = init()
                .log_level(TraceLogLevel::LOG_NONE)
                .size(0, 0)
                .resizable()
                .title("bit32")
                .vsync()
                .build();
            let (mut window, thread) = raylib;
            let font = unsafe {
                match window.load_font(
                    &thread,
                    "/usr/share/fonts/truetype/ModernDOS/ModernDOS8x16.ttf",
                ) {
                    Ok(font) => WeakFont::from_raw(font.to_raw()),
                    Err(_) => window.get_font_default(),
                }
            };
            let mut core = Core {
                vram: [0; GPU::VRAM_SIZE],
                font,
                window,
                thread,
            };
            loop {
                let msg = receiver
                    .recv()
                    .expect("Could not receive message in gpu thread");
                match msg[0] {
                    GPU::HLT => {
                        core.deinit();
                        return;
                    }
                    GPU::DRAW_VGA => core.draw(),
                    GPU::WRITE_BYTE => {
                        let dest = msg[1] as usize + ((msg[2] as usize) << 8);
                        core.vram[dest] = msg[3];
                    }
                    GPU::WRITE_SHORT => {
                        let dest = msg[1] as usize + ((msg[2] as usize) << 8);
                        core.vram[dest + 0] = msg[3];
                        core.vram[dest + 1] = msg[4];
                    }
                    GPU::WRITE_LONG => {
                        let dest = msg[1] as usize + ((msg[2] as usize) << 8);
                        core.vram[dest + 0] = msg[3];
                        core.vram[dest + 1] = msg[4];
                        core.vram[dest + 2] = msg[5];
                        core.vram[dest + 3] = msg[6];
                    }
                    _ => panic!("Unknown gpu instruction: {}", msg[0]),
                }
            }
        });
        Self {
            thread_handle: Some(thread_handle),
            sender,
            cfg: None,
            instruction_size: 0,
            instruction_buffer_ptr: 0,
            instruction_buffer: [0; 9],
        }
    }
}

impl Hardware for GPU {
    fn init(&mut self, cfg: Config) {
        self.cfg = Some(cfg);
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
            self.sender
                .send(self.instruction_buffer)
                .expect("Could not send message to gpu thread");
            self.instruction_size = 0;
            self.instruction_buffer_ptr = 0;
        }
    }

    fn deinit(&mut self) {
        self.sender
            .send([0; 9])
            .expect("Could not send message to gpu thread");
        match self.thread_handle.take() {
            Some(handle) => handle.join().expect("Failed to join GPU thread"),
            None => panic!("Thread handle already taken"),
        }
    }
}

impl Core {
    pub fn draw(&mut self) {
        let mut x = 0;
        let mut y = 0;

        let buffer = &self.vram;
        let width = self.window.get_screen_width() / 80;
        let height = i32::min(self.window.get_screen_height() / 25, width * 2);
        let font_size = height as f32;
        let spacing = 0.0;

        let mut handle = self.window.begin_drawing(&self.thread);
        for chunk in buffer.chunks(2) {
            if chunk.len() == 2 {
                let mut ch = chunk[0] as char;
                if !ch.is_ascii_graphic() {
                    ch = ' ';
                }
                let color = chunk[1];
                let fg_color = GPU::vga_to_raylib_color(color & 0x0F);
                let bg_color = GPU::vga_to_raylib_color((color >> 4) & 0x0F);

                handle.draw_rectangle(x * width, y * height, width, height, bg_color);

                let position = Vector2 {
                    x: (x * width) as f32,
                    y: (y * height) as f32,
                };

                handle.draw_text_ex(
                    &self.font,
                    &ch.to_string(),
                    position,
                    font_size,
                    spacing,
                    fg_color,
                );

                x += 1;
                if x >= 80 {
                    x = 0;
                    y += 1;
                }
            }
        }
    }
    pub fn deinit(&mut self) {
        unsafe {
            CloseWindow();
        }
    }
}

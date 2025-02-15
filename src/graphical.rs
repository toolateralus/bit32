use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use raylib::{color::Color, init, prelude::RaylibDraw};

use crate::{
    cpu::Cpu,
    hardware::{Config, Hardware, Numeric},
};

pub struct GfxContext {
    pub join_handle: Option<JoinHandle<i32>>,
    pub vga_buffer: Option<[u8; Cpu::VGA_BUFFER_LEN]>,
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

    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            join_handle: None,
            vga_buffer: None,
            cfg: None,
        }))
    }
}

impl Hardware for GfxContext {
    fn init(this: Arc<Mutex<Self>>, cfg: Config) {
        {
            let mut self_locked = this.lock().unwrap();
            self_locked.cfg = Some(cfg);
        }

        let self_arc_clone = Arc::clone(&this);

        let join_handle = Some(thread::spawn(move || {
            let (mut window, thread) = init().build();
            while !window.window_should_close() {
                let mut handle = window.begin_drawing(&thread);
                let mut x = 0;
                let mut y = 0;
                let self_ = self_arc_clone.lock().unwrap();
                let buffer = self_.vga_buffer.unwrap();

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
            0
        }));

        this.lock().unwrap().join_handle = join_handle;
    }

    fn read<T: Numeric>(&self) -> T {
        todo!()
    }

    fn write<T: Numeric>(&mut self, b: T) {
        todo!()
    }
}

impl Drop for GfxContext {
    fn drop(&mut self) {
        if let Some(handle) = self.join_handle.take() {
            match handle.join() {
                Ok(_) => {}
                Err(err) => {
                    panic!("error while joining raylib graphics thread {err:?}");
                }
            }
        }
    }
}

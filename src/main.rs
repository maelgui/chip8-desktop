use std::{env, fs, process};

use chip8core;
use minifb::{Scale, WindowOptions, Window, Key};

fn key_mapping(key: chip8core::Key) -> Key {
    match key {
        chip8core::Key::Key0 => Key::X,
        chip8core::Key::Key1 => Key::Key1,
        chip8core::Key::Key2 => Key::Key2,
        chip8core::Key::Key3 => Key::Key3,
        chip8core::Key::Key4 => Key::Q,
        chip8core::Key::Key5 => Key::W,
        chip8core::Key::Key6 => Key::E,
        chip8core::Key::Key7 => Key::A,
        chip8core::Key::Key8 => Key::S,
        chip8core::Key::Key9 => Key::D,
        chip8core::Key::KeyA => Key::Z,
        chip8core::Key::KeyB => Key::C,
        chip8core::Key::KeyC => Key::Key4,
        chip8core::Key::KeyD => Key::R,
        chip8core::Key::KeyE => Key::F,
        chip8core::Key::KeyF => Key::V,
    }
}

struct MyWindow(minifb::Window);

impl MyWindow {
    fn new() -> Self {
        let mut options = WindowOptions::default();
        options.scale = Scale::X8;
        MyWindow(Window::new(
            "Test - ESC to exit",
            chip8core::WIDTH,
            chip8core::HEIGHT,
            options,
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        }))
    }
}

impl chip8core::Window for MyWindow {
    fn update_with_buffer(&mut self, buffer: &[u8], width: usize, height: usize) {
        let colored_buffer: Vec<u32> = buffer.into_iter().map(|x| if *x == 1u8 { 0xF3E9C1 } else { 0x438190 }).collect();
        self.0
            .update_with_buffer(&colored_buffer, width, height)
            .unwrap();
    }

    fn is_running(&mut self) -> bool {
        self.0.is_open() && !self.0.is_key_down(Key::Escape)
    }
}


impl chip8core::Keyboard for MyWindow {
    fn is_key_down(&self, key: chip8core::Key) -> bool {
        self.0.is_key_down(key_mapping(key))
    }

    fn wait_key_down(&self) -> chip8core::Key {
        todo!()
    }
}

fn main() {

    env_logger::init();

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Missing rom file");
        eprintln!("Usage: {} <rom_file>", args.first().unwrap());
        process::exit(1);
    }

    let rom_file = std::env::args().nth(1).unwrap();
    
    let mut my_window = MyWindow::new();
    let mut my_chip8 = chip8core::Chip8::new(&mut my_window);

    let rom_vec = fs::read(rom_file).expect("Rom file not found.");

    my_chip8.init(&rom_vec);

    my_chip8.start();
}

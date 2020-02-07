extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

mod utils;

use wasm_bindgen::prelude::*;

use nes_core::console::NesConsole;
use nes_core::controller::Controller;
use nes_core::controller::ControllerDataLine;
use nes_core::rom::rom_file::RomFile;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct NesWebContext {
    nes: NesConsole,
    output_buffer: Vec<u32>,
    output_buffer_scaled: Vec<u32>,
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn init() -> NesWebContext {
    utils::set_panic_hook();

    // println!("Loading ROM from {}", rom_path.display());
    // let mut rom = RomFile::from_file(rom_path);
    let mut rom = RomFile::from_bytes(include_bytes!("../../roms/Super Mario Bros (E).nes"));
    // let mut rom = RomFile::from_bytes(include_bytes!("../../roms/Donkey Kong (World) (Rev A).nes"));
    let mut nes = NesConsole::new();

    nes.bus.borrow_mut().connect_cartridge(&mut rom);

    {
        let mut bus = nes.bus.borrow_mut();
        let controller = Controller::new();
        bus.controller0 = Some(controller);
    }

    nes.reset();

    NesWebContext {
        nes,
        output_buffer: vec![0; 256 * 240],
        output_buffer_scaled: vec![0; 512 * 480],
    }
}

#[wasm_bindgen]
impl NesWebContext {
    pub fn nes_frame(&mut self) {
        self.nes.render_full_frame();
    }

    pub fn set_image_array(&self, to_fill: &mut [u8]) {
        self.nes.get_output_rgba_u8(to_fill);
    }

    pub fn set_image_array_upscale(&mut self, to_fill: &mut [u8]) {
        self.nes.get_output_rgb_u32(&mut self.output_buffer[..]);
        nes_core::xbr::apply(
            &mut self.output_buffer_scaled,
            &self.output_buffer,
            256,
            240,
        );

        for i in 0..self.output_buffer_scaled.len() {
            to_fill[i * 4 + 0] = ((self.output_buffer_scaled[i] >> 16) & 0xFF) as u8;
            to_fill[i * 4 + 1] = ((self.output_buffer_scaled[i] >> 8) & 0xFF) as u8;
            to_fill[i * 4 + 2] = (self.output_buffer_scaled[i] & 0xFF) as u8;
            to_fill[i * 4 + 3] = 0xFF;
        }
    }

    pub fn key_down(&mut self, key: u8) {
        if let Some(controller) = self.nes.bus.borrow_mut().controller0.as_mut() {
            controller
                .data
                .insert(ControllerDataLine::from_bits(key).unwrap());
        }
    }

    pub fn key_up(&mut self, key: u8) {
        if let Some(controller) = self.nes.bus.borrow_mut().controller0.as_mut() {
            controller
                .data
                .remove(ControllerDataLine::from_bits(key).unwrap());
        }
    }
}

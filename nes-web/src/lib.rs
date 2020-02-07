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
use nes_core::palette;
use nes_core::rom::rom_file::RomFile;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct NesWebContext {
    nes: NesConsole,
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
    // let mut rom = RomFile::from_bytes(include_bytes!("../../roms/Super Mario Bros (E).nes"));
    let mut rom = RomFile::from_bytes(include_bytes!("../../roms/Donkey Kong (World) (Rev A).nes"));
    let mut nes = NesConsole::new();

    nes.bus.borrow_mut().connect_cartridge(&mut rom);

    {
        let mut bus = nes.bus.borrow_mut();
        let controller = Controller::new();
        bus.controller0 = Some(controller);
    }

    nes.reset();

    NesWebContext { nes: nes }
}

#[wasm_bindgen]
impl NesWebContext {
    pub fn nes_frame(&mut self) {
        self.nes.render_full_frame();
    }

    pub fn set_image_array(&self, to_fill: &mut [u8], width: usize, height: usize) {
        self.nes.get_output_rgba_u8(to_fill);
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

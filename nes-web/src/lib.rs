extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;

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
pub enum ControllerKeys {
    A = 1,
    B = 2,
    SELECT = 4,
    START = 8,
    UP = 16,
    DOWN = 32,
    LEFT = 64,
    RIGHT = 128,
}

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

    NesWebContext { nes }
}

#[wasm_bindgen]
impl NesWebContext {
    pub fn nes_frame(&mut self) {
        self.nes.render_full_frame();
    }

    pub fn get_background_color(&self) -> String {
        let ppu = self.nes.ppu.borrow();
        let color_idx = ppu.palette_vram[0];

        let (r, g, b) = palette::get_rgb_color_split(color_idx);

        format!("#{:02X}{:02X}{:02X}", r, g, b)
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

    pub fn setup_canvas(&mut self, canvas: &web_sys::HtmlCanvasElement) {
        canvas.set_width(256);
        canvas.set_height(240);
    }

    pub fn update_canvas(&mut self, canvas: &web_sys::HtmlCanvasElement) -> Result<(), JsValue> {
        let context = canvas
            .get_context("2d")?
            .expect("context 2d to be available")
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

        let width = 256;
        let height = 240;

        self.nes.render_full_frame();

        let mut output_buffer = vec![0; (256 * 240 * 4) as usize];
        self.nes.get_output_rgba_u8(&mut output_buffer);

        let image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&output_buffer),
            width,
            height,
        )?;

        context.put_image_data(&image_data, 0.0, 0.0)?;

        Ok(())
    }
}

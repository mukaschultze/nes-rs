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
use nes_core::input::joypad::Joypad;
use nes_core::input::joypad::JoypadDataLine;
use nes_core::input::zapper_gun::ZapperGun;
use nes_core::input::InputType;
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
pub enum InputTypeValue {
    Joypad = "joypad",
    ZapperGun = "zapper",
    Disconnected = "disconnected",
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
    // let mut rom = RomFile::from_bytes(include_bytes!("../../roms/Duck Hunt (JUE) [p1].nes"));
    let mut rom = RomFile::from_bytes(include_bytes!("../../roms/Super Mario Bros (E).nes"));
    // let mut rom = RomFile::from_bytes(include_bytes!("../../roms/zapper/zapper_light.nes"));
    // let mut rom = RomFile::from_bytes(include_bytes!("../../roms/Donkey Kong (World) (Rev A).nes"));
    let nes = NesConsole::new();

    nes.bus.borrow_mut().connect_cartridge(&mut rom);

    NesWebContext { nes }
}

#[wasm_bindgen]
impl NesWebContext {
    pub fn nes_frame(&mut self) {
        self.nes.render_full_frame();
    }

    pub fn reset(&mut self) {
        self.nes.reset();
    }

    pub fn attach_joypad(&mut self, input: u8) {
        let mut bus = self.nes.bus.borrow_mut();
        let joypad = Joypad::new();

        match input {
            0 => bus.input0 = InputType::Joypad(joypad),
            1 => bus.input1 = InputType::Joypad(joypad),
            _ => (),
        };
    }

    pub fn attach_zapper_gun(&mut self, input: u8) {
        let mut bus = self.nes.bus.borrow_mut();
        let zapper = ZapperGun::new();

        match input {
            0 => bus.input0 = InputType::Zapper(zapper),
            1 => bus.input1 = InputType::Zapper(zapper),
            _ => (),
        };
    }

    pub fn get_input_type(&self, input: u8) -> InputTypeValue {
        let bus = self.nes.bus.borrow();

        let input_device = match input {
            0 => &bus.input0,
            1 => &bus.input1,
            _ => &InputType::Disconnected,
        };

        match input_device {
            InputType::Joypad(_) => InputTypeValue::Joypad,
            InputType::Zapper(_) => InputTypeValue::ZapperGun,
            InputType::Disconnected => InputTypeValue::Disconnected,
        }
    }

    pub fn key_down(&mut self, key: u8, input: u8) {
        let mut bus = self.nes.bus.borrow_mut();
        let input_device = match input {
            0 => &mut bus.input0,
            1 => &mut bus.input1,
            _ => return,
        };

        if let InputType::Joypad(joypad) = input_device {
            joypad.data.insert(JoypadDataLine::from_bits(key).unwrap());
        }
    }

    pub fn key_up(&mut self, key: u8, input: u8) {
        let mut bus = self.nes.bus.borrow_mut();
        let input_device = match input {
            0 => &mut bus.input0,
            1 => &mut bus.input1,
            _ => return,
        };

        if let InputType::Joypad(joypad) = input_device {
            joypad.data.remove(JoypadDataLine::from_bits(key).unwrap());
        }
    }

    pub fn zapper_gun_input(&mut self, trigger_pulled: bool, light_sense: bool, input: u8) {
        let mut bus = self.nes.bus.borrow_mut();
        let input_device = match input {
            0 => &mut bus.input0,
            1 => &mut bus.input1,
            _ => return,
        };

        if let InputType::Zapper(zapper) = input_device {
            zapper.trigger_pulled = trigger_pulled;
            zapper.light_sense = light_sense;
        }
    }

    pub fn color_at(&self, x: usize, y: usize) -> String {
        let ppu = self.nes.ppu.borrow();
        let pixel_idx = y * 256 + x;
        let color_raw = ppu.output[pixel_idx];

        let (r, g, b) = palette::get_rgb_color_split(color_raw);

        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }

    pub fn brigthness_at(&self, x: usize, y: usize) -> f32 {
        let ppu = self.nes.ppu.borrow();
        let pixel_idx = y * 256 + x;
        let color_raw = ppu.output[pixel_idx];

        let (r, g, b) = palette::get_rgb_color_split(color_raw);

        (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) / 255.0
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

    pub fn get_background_color(&self) -> String {
        let ppu = self.nes.ppu.borrow();
        let color_idx = ppu.palette_vram[0];

        let (r, g, b) = palette::get_rgb_color_split(color_idx);

        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }
}

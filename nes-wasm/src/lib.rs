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
use nes_core::console::NES_HEIGHT;
use nes_core::console::NES_WIDTH;
use nes_core::input::joypad::Joypad;
use nes_core::input::joypad::JoypadDataLine;
use nes_core::input::zapper_gun::ZapperGun;
use nes_core::input::InputType;
use nes_core::palette;
use nes_core::rom::rom_file::RomFile;

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
impl NesWebContext {
    #[wasm_bindgen(constructor)]
    pub fn new() -> NesWebContext {
        utils::set_panic_hook();
        NesWebContext {
            nes: NesConsole::new(),
        }
    }

    pub fn nes_frame(&mut self) {
        self.nes.render_full_frame();
    }

    pub fn reset(&mut self) {
        self.nes.reset();
    }

    pub fn insert_cartridge(&mut self, rom_bytes: Vec<u8>) {
        let rom = RomFile::from_bytes(&rom_bytes);

        self.nes.bus.borrow_mut().connect_cartridge(rom);
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

    pub fn get_input_type(&self, input: u8) -> String {
        let bus = self.nes.bus.borrow();

        let input_device = match input {
            0 => &bus.input0,
            1 => &bus.input1,
            _ => &InputType::Disconnected,
        };

        match input_device {
            InputType::Joypad(_) => "joypad".into(),
            InputType::Zapper(_) => "zapper".into(),
            InputType::Disconnected => "disconnected".into(),
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

    pub fn simulate(&mut self) {
        self.nes.render_full_frame();
    }

    pub fn get_frame_output_rgba_u8(&self) -> Vec<u8> {
        let mut output_buffer = vec![0; (NES_WIDTH * NES_HEIGHT * 4) as usize];
        self.nes.get_output_rgba_u8(&mut output_buffer);
        output_buffer
    }

    pub fn get_frame_output_rgb_u8(&self) -> Vec<u8> {
        let mut output_buffer = vec![0; (NES_WIDTH * NES_HEIGHT * 3) as usize];
        self.nes.get_output_rgb_u8(&mut output_buffer);
        output_buffer
    }

    pub fn get_frame_output_rgb_u32(&self) -> Vec<u32> {
        let mut output_buffer = vec![0; (NES_WIDTH * NES_HEIGHT) as usize];
        self.nes.get_output_rgb_u32(&mut output_buffer);
        output_buffer
    }

    pub fn setup_canvas(&mut self, canvas: &web_sys::HtmlCanvasElement) {
        canvas.set_width(NES_WIDTH);
        canvas.set_height(NES_HEIGHT);
    }

    pub fn update_canvas(&self, canvas: &web_sys::HtmlCanvasElement) -> Result<(), JsValue> {
        let context = canvas
            .get_context("2d")?
            .expect("context 2d is not available")
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

        let output_buffer = self.get_frame_output_rgba_u8();

        let image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&output_buffer),
            NES_WIDTH,
            NES_HEIGHT,
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
}

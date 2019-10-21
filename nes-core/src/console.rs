extern crate gif;
extern crate png;

use crate::bus::DataBus;
use crate::cpu::CPU6502;
use crate::palette;
use crate::ppu::Ppu;

use gif::{Encoder, Frame, Repeat, SetParameter};

use std::borrow::Cow;
use std::cell::RefCell;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use std::rc::Rc;
use std::u16;

pub struct NesConsole {
    pub cpu: Rc<RefCell<CPU6502>>,
    pub bus: Rc<RefCell<DataBus>>,
    pub ppu: Rc<RefCell<Ppu>>,
}

impl NesConsole {
    pub fn new() -> NesConsole {
        let bus = Rc::new(RefCell::new(DataBus::new()));
        let cpu = Rc::new(RefCell::new(CPU6502::new(bus.clone())));
        let ppu = Rc::new(RefCell::new(Ppu::new(cpu.clone())));

        {
            let mut bus = bus.borrow_mut();
            bus.ppu = Some(ppu.clone());
        }

        NesConsole { bus, cpu, ppu }
    }

    pub fn reset(&mut self) {
        println!("Reset!");

        // http://wiki.nesdev.com/w/index.php/CPU_power_up_state
        let mut cpu = self.cpu.borrow_mut();
        let mut bus = self.bus.borrow_mut();
        let pc_high = bus.read(0xFFFD);
        let pc_low = bus.read(0xFFFC);
        cpu.sp -= 3; // S was decremented by 3 (but nothing was written to the stack)
        cpu.sr |= 0b0000_0100; // The I (IRQ disable) flag was set to true (status ORed with $04)
        cpu.pc = join_bytes!(pc_high, pc_low);
    }

    pub fn tick(&mut self) {
        let mut l = self.cpu.borrow().ticks;
        self.cpu.borrow_mut().process_next_opcode();
        l = self.cpu.borrow().ticks - l;

        for _ in 0..l * 3 {
            self.ppu.borrow_mut().tick();
        }
    }

    pub fn render_full_frame(&mut self) {
        static mut RENDER_REQUEST: bool = false;

        {
            let mut ppu = self.ppu.borrow_mut();

            ppu.v_blank_callback = Box::new(|| unsafe {
                RENDER_REQUEST = true;
            });
        }

        loop {
            self.tick();

            unsafe {
                if RENDER_REQUEST {
                    RENDER_REQUEST = false;
                    break;
                }
            }
        }
    }

    pub fn screenshot(&self, path: &str) {
        let file = File::create(Path::new(path)).unwrap();
        let ref mut w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, 256, 240);

        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_compression(png::Compression::Default);
        encoder.set_filter(png::FilterType::NoFilter);

        let output = self.ppu.borrow().output;
        let mut writer = encoder.write_header().unwrap();
        let mut data = vec![0u8; output.len() * 3];

        for i in 0..output.len() {
            let (r, g, b) = palette::get_rgb_color_split(output[i]);

            data[i * 3 + 0] = r;
            data[i * 3 + 1] = g;
            data[i * 3 + 2] = b;
        }
        writer.write_image_data(&data).unwrap();
    }

    pub fn get_gif_encoder(&self, path: &Path) -> Encoder<File> {
        let (width, height) = (256, 240);
        let color_map = palette::get_full_palette_split();
        let image = File::create(path).unwrap();
        let mut encoder = Encoder::new(image, width, height, color_map).unwrap();
        encoder.set(Repeat::Infinite).unwrap();
        encoder
    }

    pub fn frame_to_gif<W: Write>(&self, encoder: &mut Encoder<W>) {
        let (width, height) = (256, 240);
        let output = self.ppu.borrow().output;
        let mut frame = Frame::default();
        frame.width = width;
        frame.height = height;
        frame.delay = 2;
        frame.buffer = Cow::Borrowed(&output);
        encoder.write_frame(&frame).unwrap();
    }
}

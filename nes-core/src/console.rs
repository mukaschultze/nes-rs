extern crate gif;
extern crate png;

use crate::bus::DataBus;
use crate::cpu::CPU6502;
use crate::palette;
use crate::ppu::Ppu;
use crate::rom::rom_file::RomFile;

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
    pub fn new(rom: Rc<RefCell<RomFile>>) -> NesConsole {
        let bus = Rc::new(RefCell::new(DataBus::new(rom.clone())));
        let cpu = Rc::new(RefCell::new(CPU6502::new(bus.clone())));
        let ppu = Rc::new(RefCell::new(Ppu::new(cpu.clone(), rom.clone())));

        {
            let cpu = cpu.clone();
            let bus = bus.clone();
            let mut bus_mut = bus.borrow_mut();
            let mut cpu_mut = cpu.borrow_mut();

            bus_mut.ppu = Some(ppu.clone());

            let pc_high = bus_mut.read(0xFFFD);
            let pc_low = bus_mut.read(0xFFFC);

            cpu_mut.pc = join_bytes!(pc_high, pc_low);
        }

        NesConsole { bus, cpu, ppu }
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

    pub fn screenshot(&self, path: &Path) {
        let file = File::create(path).unwrap();
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

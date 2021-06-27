use crate::controller::Controller;
use crate::ppu::Ppu;
use crate::rom::mapper::Mapper;
use crate::rom::rom_file::RomFile;
use std::cell::RefCell;
use std::rc::Rc;

const RAM_SIZE: usize = 0x0800;

fn repeat_every<T>(n: T, start: T, repeat: T) -> T
where
    T: std::ops::Sub<Output = T> + std::ops::Rem<Output = T> + std::ops::Add<Output = T> + Copy,
{
    ((n - start) % repeat) + start
}

pub struct DataBus {
    ram: [u8; RAM_SIZE],
    pub mapper: Option<Rc<RefCell<Box<dyn Mapper>>>>,
    pub ppu: Option<Rc<RefCell<Ppu>>>,
    pub controller0: Option<Controller>,
    pub controller1: Option<Controller>,
}

impl DataBus {
    pub fn new() -> Self {
        Self {
            ram: [0; RAM_SIZE],
            mapper: None,
            ppu: None,
            controller0: None,
            controller1: None,
        }
    }

    pub fn connect_cartridge(&mut self, rom: &mut RomFile) {
        let b = Rc::new(RefCell::new(rom.get_mapper()));
        self.mapper = Some(b.clone());
        self.ppu.as_mut().unwrap().borrow_mut().mapper = Some(b.clone());
    }

    /// https://wiki.nesdev.com/w/index.php/CPU_memory_map
    #[allow(clippy::match_overlapping_arm)]
    pub fn read(&mut self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.ram[address as usize % RAM_SIZE],
            0x2000..=0x3FFF => self
                .ppu
                .as_ref()
                .unwrap()
                .borrow_mut()
                .read_register_cpu_address(repeat_every(address, 0x2000, 8)),
            0x4014 => 0, // OAMDMA $4014 is write only!
            0x4016 => {
                if let Some(controller) = self.controller0.as_mut() {
                    controller.output()
                } else {
                    0
                }
            }
            0x4017 => {
                if let Some(controller) = self.controller1.as_mut() {
                    controller.output()
                } else {
                    0
                }
            }
            0x4000..=0x401F => 0, // APU and IO registers
            0x4020..=0xFFFF => {
                if let Some(mapper) = self.mapper.as_mut() {
                    mapper.borrow_mut().read_prg(address)
                } else {
                    0
                }
            }
        }
    }

    /// https://wiki.nesdev.com/w/index.php/CPU_memory_map
    #[allow(clippy::match_overlapping_arm)]
    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram[address as usize % RAM_SIZE] = value,
            0x2000..=0x3FFF => self
                .ppu
                .as_ref()
                .unwrap()
                .borrow_mut()
                .write_register_cpu_address(repeat_every(address, 0x2000, 8), value),
            0x4014 => {
                for i in 0..=255 {
                    let v = self.read(((value as u16) << 8) + i as u16);
                    let mut ppu = self.ppu.as_ref().unwrap().borrow_mut();
                    let oam_addr = ppu.oam_address;
                    ppu.oam_memory[unchecked_add!(i, oam_addr) as usize % 256] = v;
                }
            }
            0x4016 => {
                if let Some(controller) = self.controller0.as_mut() {
                    controller.input(value)
                }
                if let Some(controller) = self.controller1.as_mut() {
                    controller.input(value)
                }
            }
            0x4017 => {}
            0x4000..=0x401F => {} // APU and IO registers
            0x4020..=0xFFFF => {
                if let Some(mapper) = self.mapper.as_mut() {
                    mapper.borrow_mut().write_prg(address, value);
                }
            }
        }
    }
}

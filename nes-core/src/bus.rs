use crate::controller::Controller;
use crate::ppu::Ppu;
use crate::rom::rom_file::RomFile;
use std::cell::RefCell;
use std::rc::Rc;

const RAM_SIZE: usize = 0x0800;

pub struct DataBus {
    ram: [u8; RAM_SIZE],
    rom: Rc<RefCell<RomFile>>,
    pub ppu: Option<Rc<RefCell<Ppu>>>,
    pub controller0: Option<Controller>,
    pub controller1: Option<Controller>,
}

impl DataBus {
    pub fn new(rom: Rc<RefCell<RomFile>>) -> Self {
        Self {
            ram: [0; RAM_SIZE],
            rom,
            ppu: None,
            controller0: None,
            controller1: None,
        }
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
                .read_register_cpu_address(address),
            0x4014 => 0, // OAMDMA $4014 is write only!
            0x4016 => match self.controller0.as_mut() {
                Some(controller) => controller.output(),
                None => 0,
            },
            0x4017 => match self.controller1.as_mut() {
                Some(controller) => controller.output(),
                None => 0,
            },
            0x4000..=0x401F => 0, // APU and IO registers
            0x4020..=0x5FFF => 0, // Cartridge space
            0x6000..=0x7FFF => 0, // Battery Backed Save or Work RAM
            0x8000..=0xFFFF => {
                let rom = self.rom.borrow();
                rom.read(address)
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
                .write_register_cpu_address(address, value),
            0x4014 => {
                for i in 0..=255 {
                    let v = self.read(((value as u16) << 8) + i as u16);
                    let mut ppu = self.ppu.as_ref().unwrap().borrow_mut();
                    let oam_addr = ppu.oamAddress;
                    ppu.oamMemory[unchecked_add!(i, oam_addr) as usize % 256] = v;
                }
            }
            0x4016 => match self.controller0.as_mut() {
                Some(controller) => controller.input(value),
                None => {}
            },
            0x4017 => match self.controller1.as_mut() {
                Some(controller) => controller.input(value),
                None => {}
            },
            0x4000..=0x401F => {} // APU and IO registers
            0x4020..=0x5FFF => {} // Cartridge space
            0x6000..=0x7FFF => {} // Battery Backed Save or Work RAM
            0x8000..=0xFFFF => {
                let rom = self.rom.borrow_mut();
                rom.write(address, value)
            }
        }
    }
}

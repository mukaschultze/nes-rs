use crate::rom::rom_file::RomFile;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

const RAM_SIZE: usize = 0x0800;

pub struct DataBus {
    ram: [u8; RAM_SIZE],
    rom: Rc<RefCell<RomFile>>,
}

impl DataBus {
    pub fn new(rom: Rc<RefCell<RomFile>>) -> Self {
        Self {
            ram: [0; RAM_SIZE],
            rom,
        }
    }

    /// https://wiki.nesdev.com/w/index.php/CPU_memory_map
    #[allow(clippy::match_overlapping_arm)]
    pub fn read(&self, address: u16) -> u8 {
        let rom = self.rom.borrow();

        match address {
            0x0000..=0x1FFF => self.ram[address as usize % RAM_SIZE],
            0x2000..=0x3FFF | 0x4014 => 0, // nes.ppu.ReadRegisterCPUAddress(address),
            0x4016 | 0x4017 => 0,          // Controllers
            0x4000..=0x401F => 0,          // APU and IO registers
            0x4020..=0x5FFF => 0,          // Cartridge space
            0x6000..=0x7FFF => 0,          // Battery Backed Save or Work RAM
            0x8000..=0xFFFF => rom.read(address),
        }
    }

    /// https://wiki.nesdev.com/w/index.php/CPU_memory_map
    #[allow(clippy::match_overlapping_arm)]
    pub fn write(&mut self, address: u16, value: u8) {
        let rom = self.rom.borrow_mut();

        match address {
            0x0000..=0x1FFF => self.ram[address as usize % RAM_SIZE] = value,
            0x2000..=0x3FFF | 0x4014 => {} // nes.ppu.WriteRegisterCPUAddress(address),
            0x4016 | 0x4017 => {}          // Controllers
            0x4000..=0x401F => {}          // APU and IO registers
            0x4020..=0x5FFF => {}          // Cartridge space
            0x6000..=0x7FFF => {}          // Battery Backed Save or Work RAM
            0x8000..=0xFFFF => rom.write(address, value),
        }
    }
}

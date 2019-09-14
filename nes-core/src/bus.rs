use crate::rom::mappers::mapper0::Mapper0;
use crate::rom::mappers::Mapper;

const RAM_SIZE: usize = 0x0800;

pub struct DataBus<'a> {
    ram: [u8; RAM_SIZE],
    mapper: &'a mut Mapper0<'a>,
}

impl<'a> DataBus<'a> {
    pub fn new(mapper: &'a mut Mapper0<'a>) -> Self {
        Self {
            ram: [0; RAM_SIZE],
            mapper,
        }
    }

    /// https://wiki.nesdev.com/w/index.php/CPU_memory_map
    #[allow(clippy::match_overlapping_arm)]
    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.ram[address as usize % RAM_SIZE],
            0x2000..=0x3FFF | 0x4014 => 0, // nes.ppu.ReadRegisterCPUAddress(address),
            0x4016 | 0x4017 => 0,          // Controllers
            0x4000..=0x401F => 0,          // APU and IO registers
            0x4020..=0x5FFF => 0,          // Cartridge space
            0x6000..=0x7FFF => 0,          // Battery Backed Save or Work RAM
            0x8000..=0xFFFF => self.mapper.read(address),
        }
    }

    /// https://wiki.nesdev.com/w/index.php/CPU_memory_map
    #[allow(clippy::match_overlapping_arm)]
    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram[address as usize % RAM_SIZE] = value,
            0x2000..=0x3FFF | 0x4014 => {} // nes.ppu.WriteRegisterCPUAddress(address),
            0x4016 | 0x4017 => {}          // Controllers
            0x4000..=0x401F => {}          // APU and IO registers
            0x4020..=0x5FFF => {}          // Cartridge space
            0x6000..=0x7FFF => {}          // Battery Backed Save or Work RAM
            0x8000..=0xFFFF => self.mapper.write(address, value),
        }
    }
}

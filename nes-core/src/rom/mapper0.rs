use crate::rom::mapper::Mapper;
use crate::rom::rom_file::RomFile;
use crate::rom::rom_header::RomHeader;

/// The generic designation NROM refers to the Nintendo cartridge boards NES-NROM-128,
/// NES-NROM-256, their HVC counterparts, and clone boards. The iNES format assigns mapper 0 to NROM.
/// https://wiki.nesdev.com/w/index.php/NROM
pub struct Mapper0 {
    pub header: RomHeader,
    pub pgr_data: Box<[u8]>,
    pub chr_data: Box<[u8]>,
}

impl Mapper0 {
    pub fn new(rom: &mut RomFile) -> Self {
        Self {
            header: rom.header,
            pgr_data: Box::from(rom.pgr_data.as_ref()),
            chr_data: Box::from(rom.chr_data.as_ref()),
        }
    }

    fn rel_address(&self, address: u16) -> u16 {
        match self.header.prg_rom_size {
            1 => (address - 0x8000) % 0x4000,
            _ => (address - 0x8000),
        }
    }
}

impl Mapper for Mapper0 {
    fn read_prg(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x5FFF => unreachable!(),
            0x6000..=0x7FFF => 0,
            0x8000..=0xFFFF => self.pgr_data[self.rel_address(addr) as usize],
        }
    }

    fn write_prg(&mut self, addr: u16, value: u8) {}

    fn read_chr(&self, addr: u16) -> u8 {
        match self.header.chr_rom_size {
            0 => 0,
            1 => self.chr_data[addr as usize & 0x1FFF],
            2 => self.chr_data[addr as usize & 0x3FFF],
            _ => unimplemented!(),
        }
    }

    fn write_chr(&mut self, addr: u16, value: u8) {}
}

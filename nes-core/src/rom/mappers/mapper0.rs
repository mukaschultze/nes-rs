use crate::rom::mappers::Mapper;
use crate::rom::rom_file::RomFile;

pub struct Mapper0<'a> {
    pub rom: &'a mut RomFile<'a>,
}

impl<'a> Mapper0<'a> {
    pub fn new(rom: &'a mut RomFile<'a>) -> Mapper0 {
        Mapper0 { rom }
    }

    pub fn rel_address(&self, address: u16) -> u16 {
        match self.rom.header.prg_rom_size {
            1 => (address - 0x8000) % 0x4000,
            _ => (address - 0x8000),
        }
    }
}

impl Mapper for Mapper0<'_> {
    fn get_id(&self) -> u8 {
        0
    }

    fn get_name(&self) -> &'static str {
        "NROM"
    }

    fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.rom.chr_data[address as usize],
            0x8000..=0xFFFF => self.rom.pgr_data[self.rel_address(address) as usize],
            _address => panic!(
                "Tried to read from address outside ROM bounds: {:#X}",
                address
            ),
        }
    }

    fn write(&self, _address: u16, _value: u8) {}
}

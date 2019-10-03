use crate::rom::rom_header::RomHeader;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct RomFile {
    pub header: RomHeader,
    pub pgr_data: Vec<u8>,
    pub chr_data: Vec<u8>,
}

impl RomFile {
    pub fn new(path: &Path) -> Self {
        let display = path.display();

        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
        };

        let header_buf = &mut [0u8; 16];

        match file.read_exact(header_buf) {
            Ok(file) => file,
            Err(why) => panic!("Error reading {}: {}", display, why.description()),
        };

        let header = RomHeader::new(header_buf);

        println!("Loaded ROM header: {:?}", header);

        let pgr_data = &mut vec![0u8; 16384 * header.prg_rom_size as usize];
        let chr_data = &mut vec![0u8; 8192 * header.chr_rom_size as usize];

        if !header.is_valid() {
            panic!("Invalid NES rom file");
        }

        match file.read_exact(pgr_data) {
            Ok(file) => file,
            Err(why) => panic!("Error reading {}: {}", display, why.description()),
        };
        match file.read_exact(chr_data) {
            Ok(file) => file,
            Err(why) => panic!("Error reading {}: {}", display, why.description()),
        };

        RomFile {
            header,
            pgr_data: pgr_data.to_vec(),
            chr_data: chr_data.to_vec(),
        }
    }

    pub fn rel_address(&self, address: u16) -> u16 {
        match self.header.prg_rom_size {
            1 => (address - 0x8000) % 0x4000,
            _ => (address - 0x8000),
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.chr_data[address as usize],
            0x8000..=0xFFFF => self.pgr_data[self.rel_address(address) as usize],
            _address => panic!(
                "Tried to read from address outside ROM bounds: {:#X}",
                address
            ),
        }
    }

    pub fn write(&self, _address: u16, _value: u8) {}
}

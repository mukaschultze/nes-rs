use crate::rom::mappers::mapper0::Mapper0;
use crate::rom::rom_header::RomHeader;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct RomFile<'a> {
    pub header: RomHeader,
    pub pgr_data: Vec<u8>,
    pub chr_data: Vec<u8>,
    pub mapper: Option<Mapper0<'a>>,
}

impl<'a> RomFile<'a> {
    pub fn new(path: &Path) -> Self {
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
            Ok(file) => file,
        };

        let header_buf = &mut [0u8; 16];

        match file.read_exact(header_buf) {
            Err(why) => panic!("Error reading {}: {}", display, why.description()),
            Ok(file) => file,
        };

        let header = RomHeader::new(header_buf);

        let pgr_data = &mut vec![0u8; 16384 * header.prg_rom_size as usize];
        let chr_data = &mut vec![0u8; 8192 * header.chr_rom_size as usize];

        if !header.is_valid() {
            panic!("Invalid NES rom file");
        }

        match file.read_exact(pgr_data) {
            Err(why) => panic!("Error reading {}: {}", display, why.description()),
            Ok(file) => file,
        };
        match file.read_exact(chr_data) {
            Err(why) => panic!("Error reading {}: {}", display, why.description()),
            Ok(file) => file,
        };

        RomFile {
            header,
            mapper: None,
            pgr_data: pgr_data.to_vec(),
            chr_data: chr_data.to_vec(),
        }
    }

    pub fn get_mapper(&'a mut self) -> Option<Mapper0> {
        match self.header.get_mapper_id() {
            0 => Some(Mapper0::new(self)),
            n => panic!("Mapper {:#X} not implemented", n),
        }
    }

    // public Cartridge GetCartridge() {
    //     return new Cartridge(this);
    // }
}

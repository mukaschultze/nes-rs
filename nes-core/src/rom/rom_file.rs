use crate::rom::mapper::Mapper;
use crate::rom::mapper0::Mapper0;
use crate::rom::rom_header::RomHeader;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

pub struct RomFile {
    pub header: RomHeader,
    pub pgr_data: Box<[u8]>,
    pub chr_data: Box<[u8]>,
}

impl RomFile {
    pub fn new(buffer: &mut dyn Read) -> Self {
        let header_buf = &mut [0u8; 16];

        buffer.read_exact(header_buf).unwrap();

        let header = RomHeader::new(header_buf);

        // println!("Loaded ROM header: {:?}", header);

        let mut pgr_data = vec![0u8; 0x4000 * header.prg_rom_size as usize].into_boxed_slice();
        let mut chr_data = vec![0u8; 0x2000 * header.chr_rom_size as usize].into_boxed_slice();

        assert!(header.is_valid(), "Invalid iNES rom file");

        buffer.read_exact(pgr_data.as_mut()).unwrap();
        buffer.read_exact(chr_data.as_mut()).unwrap();

        RomFile {
            header,
            pgr_data,
            chr_data,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self::new(&mut BufReader::new(bytes))
    }

    pub fn from_file(path: &Path) -> Self {
        Self::new(&mut File::open(&path).unwrap())
    }

    pub fn get_mapper(&mut self) -> impl Mapper {
        match self.header.get_mapper_id() {
            0 => Mapper0::new(self),
            id => panic!("Mapper {} not implemented", id),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn rom_file_load() {
        super::RomFile::from_bytes(include_bytes!("../../test/nestest.nes"));
    }
}

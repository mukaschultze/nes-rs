use crate::rom::rom_header::RomHeader;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

pub struct RomFile {
    pub header: RomHeader,
    pub pgr_data: Vec<u8>,
    pub chr_data: Vec<u8>,
}

impl RomFile {
    pub fn new(buffer: &mut dyn Read) -> Self {
        let header_buf = &mut [0u8; 16];

        buffer.read_exact(header_buf).unwrap();

        let header = RomHeader::new(header_buf);

        // println!("Loaded ROM header: {:?}", header);

        let pgr_data = &mut vec![0u8; 16384 * header.prg_rom_size as usize];
        let chr_data = &mut vec![0u8; 8192 * header.chr_rom_size as usize];

        assert!(header.is_valid(), "Invalid NES rom");

        buffer.read_exact(pgr_data).unwrap();
        buffer.read_exact(chr_data).unwrap();

        RomFile {
            header,
            pgr_data: pgr_data.to_vec(),
            chr_data: chr_data.to_vec(),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self::new(&mut BufReader::new(bytes))
    }

    pub fn from_file(path: &Path) -> Self {
        Self::new(&mut File::open(&path).unwrap())
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

#[cfg(test)]
mod tests {
    #[test]
    fn rom_file_load() {
        super::RomFile::from_bytes(include_bytes!("../../test/nestest.nes"));
    }
}

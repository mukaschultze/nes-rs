#[derive(Debug, Copy, Clone)]
pub struct RomHeader {
    pub nes: [u8; 4],
    pub prg_rom_size: u8,
    pub chr_rom_size: u8,
    pub flags6: u8,
    pub flags7: u8,
    pub prg_ram_size: u8,
    pub flags9: u8,
    pub flags10: u8,
}

impl RomHeader {
    pub fn new(data: &[u8]) -> RomHeader {
        assert_eq!(data.len(), 16, "ROM header should have 16 bytes");

        let mut nes: [u8; 4] = Default::default();
        nes.copy_from_slice(&data[0..4]);

        RomHeader {
            nes,
            prg_rom_size: data[4],
            chr_rom_size: data[5],
            flags6: data[6],
            flags7: data[7],
            prg_ram_size: data[8],
            flags9: data[9],
            flags10: data[10],
        }
    }

    pub fn get_mapper_id(&self) -> u8 {
        (self.flags6 >> 4) | (self.flags7 & 0xF0)
    }

    pub fn is_valid(&self) -> bool {
        const MAGIC_STRING: &[u8; 4] = b"NES\x1A";
        self.nes == *MAGIC_STRING
    }
}

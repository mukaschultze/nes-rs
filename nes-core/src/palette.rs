pub fn get_rgb_color(color_idx: u8) -> u32 {
    match color_idx {
        0x00 => 0x7C7C7C,
        0x01 => 0x0000FC,
        0x02 => 0x0000BC,
        0x03 => 0x4428BC,
        0x04 => 0x940084,
        0x05 => 0xA80020,
        0x06 => 0xA81000,
        0x07 => 0x881400,
        0x08 => 0x503000,
        0x09 => 0x007800,
        0x0A => 0x006800,
        0x0B => 0x005800,
        0x0C => 0x004058,
        0x0D => 0x000000,
        0x0E => 0x000000,
        0x0F => 0x000000,
        0x10 => 0xBCBCBC,
        0x11 => 0x0078F8,
        0x12 => 0x0058F8,
        0x13 => 0x6844FC,
        0x14 => 0xD800CC,
        0x15 => 0xE40058,
        0x16 => 0xF83800,
        0x17 => 0xE45C10,
        0x18 => 0xAC7C00,
        0x19 => 0x00B800,
        0x1A => 0x00A800,
        0x1B => 0x00A844,
        0x1C => 0x008888,
        0x1D => 0x000000,
        0x1E => 0x000000,
        0x1F => 0x000000,
        0x20 => 0xF8F8F8,
        0x21 => 0x3CBCFC,
        0x22 => 0x6888FC,
        0x23 => 0x9878F8,
        0x24 => 0xF878F8,
        0x25 => 0xF85898,
        0x26 => 0xF87858,
        0x27 => 0xFCA044,
        0x28 => 0xF8B800,
        0x29 => 0xB8F818,
        0x2A => 0x58D854,
        0x2B => 0x58F898,
        0x2C => 0x00E8D8,
        0x2D => 0x787878,
        0x2E => 0x000000,
        0x2F => 0x000000,
        0x30 => 0xFCFCFC,
        0x31 => 0xA4E4FC,
        0x32 => 0xB8B8F8,
        0x33 => 0xD8B8F8,
        0x34 => 0xF8B8F8,
        0x35 => 0xF8A4C0,
        0x36 => 0xF0D0B0,
        0x37 => 0xFCE0A8,
        0x38 => 0xF8D878,
        0x39 => 0xD8F878,
        0x3A => 0xB8F8B8,
        0x3B => 0xB8F8D8,
        0x3C => 0x00FCFC,
        0x3D => 0xF8D8F8,
        0x3E => 0x000000,
        0x3F => 0x000000,
        0x40..=0xFF => 0x000000,
    }
}

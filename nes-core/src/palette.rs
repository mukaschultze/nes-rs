const COLORS: &[u32] = &[
    0x7C7C7C, 0x0000FC, 0x0000BC, 0x4428BC, 0x940084, 0xA80020, 0xA81000, 0x881400, 0x503000,
    0x007800, 0x006800, 0x005800, 0x004058, 0x000000, 0x000000, 0x000000, 0xBCBCBC, 0x0078F8,
    0x0058F8, 0x6844FC, 0xD800CC, 0xE40058, 0xF83800, 0xE45C10, 0xAC7C00, 0x00B800, 0x00A800,
    0x00A844, 0x008888, 0x000000, 0x000000, 0x000000, 0xF8F8F8, 0x3CBCFC, 0x6888FC, 0x9878F8,
    0xF878F8, 0xF85898, 0xF87858, 0xFCA044, 0xF8B800, 0xB8F818, 0x58D854, 0x58F898, 0x00E8D8,
    0x787878, 0x000000, 0x000000, 0xFCFCFC, 0xA4E4FC, 0xB8B8F8, 0xD8B8F8, 0xF8B8F8, 0xF8A4C0,
    0xF0D0B0, 0xFCE0A8, 0xF8D878, 0xD8F878, 0xB8F8B8, 0xB8F8D8, 0x00FCFC, 0xF8D8F8, 0x000000,
    0x000000,
];

const COLORS_SPLIT: &[u8] = &[
    0x7C, 0x7C, 0x7C, 0x00, 0x00, 0xFC, 0x00, 0x00, 0xBC, 0x44, 0x28, 0xBC, 0x94, 0x00, 0x84, 0xA8,
    0x00, 0x20, 0xA8, 0x10, 0x00, 0x88, 0x14, 0x00, 0x50, 0x30, 0x00, 0x00, 0x78, 0x00, 0x00, 0x68,
    0x00, 0x00, 0x58, 0x00, 0x00, 0x40, 0x58, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xBC, 0xBC, 0xBC, 0x00, 0x78, 0xF8, 0x00, 0x58, 0xF8, 0x68, 0x44, 0xFC, 0xD8, 0x00, 0xCC, 0xE4,
    0x00, 0x58, 0xF8, 0x38, 0x00, 0xE4, 0x5C, 0x10, 0xAC, 0x7C, 0x00, 0x00, 0xB8, 0x00, 0x00, 0xA8,
    0x00, 0x00, 0xA8, 0x44, 0x00, 0x88, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xF8, 0xF8, 0xF8, 0x3C, 0xBC, 0xFC, 0x68, 0x88, 0xFC, 0x98, 0x78, 0xF8, 0xF8, 0x78, 0xF8, 0xF8,
    0x58, 0x98, 0xF8, 0x78, 0x58, 0xFC, 0xA0, 0x44, 0xF8, 0xB8, 0x00, 0xB8, 0xF8, 0x18, 0x58, 0xD8,
    0x54, 0x58, 0xF8, 0x98, 0x00, 0xE8, 0xD8, 0x78, 0x78, 0x78, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFC, 0xFC, 0xFC, 0xA4, 0xE4, 0xFC, 0xB8, 0xB8, 0xF8, 0xD8, 0xB8, 0xF8, 0xF8, 0xB8, 0xF8, 0xF8,
    0xA4, 0xC0, 0xF0, 0xD0, 0xB0, 0xFC, 0xE0, 0xA8, 0xF8, 0xD8, 0x78, 0xD8, 0xF8, 0x78, 0xB8, 0xF8,
    0xB8, 0xB8, 0xF8, 0xD8, 0x00, 0xFC, 0xFC, 0xF8, 0xD8, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

pub fn get_rgb_color(color_idx: u8) -> u32 {
    COLORS[color_idx as usize]
}

pub fn get_rgb_color_split(color_idx: u8) -> (u8, u8, u8) {
    let idx = color_idx as usize * 3;

    (
        COLORS_SPLIT[idx + 0],
        COLORS_SPLIT[idx + 1],
        COLORS_SPLIT[idx + 2],
    )
}

pub fn get_full_palette() -> &'static [u32] {
    COLORS
}

pub fn get_full_palette_split() -> &'static [u8] {
    COLORS_SPLIT
}

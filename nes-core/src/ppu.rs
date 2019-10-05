use crate::bus::DataBus;
use crate::cpu::CPU6502;
use crate::rom::rom_file::RomFile;
use std::cell::RefCell;
use std::rc::Rc;

/// https://wiki.nesdev.com/w/index.php/PPU
#[allow(non_snake_case)]
pub struct Ppu {
    // TODO: Use bitflags
    // #region PPUCTRL $2000
    /// Generate an NMI at the start of the vertical blanking interval (0: off; 1: on)
    nmi_enable: u8,
    /// PPU master/slave select (0: read backdrop from EXT pins; 1: output color on EXT pins)
    ppu_master_slave: u8,
    /// Sprite size (0: 8x8 pixels; 1: 8x16 pixels)
    sprite_height: u8,
    /// Background pattern table address (0: 0x0000; 1: 0x1000)
    background_tile_select: u16,
    /// Sprite pattern table address for 8x8 sprites (0: 0x0000; 1: 0x1000; ignored in 8x16 mode)
    sprite_tile_select: u16,
    /// VRAM address increment per CPU read/write of PPUDATA (0: add 1, going across; 1: add 32, going down)
    increment_mode: u16,
    /// Base nametable address (0 = 0x2000; 1 = 0x2400; 2 = 0x2800; 3 = 0x2C00)
    nametable_select: u16,
    // #endregion

    // TODO: Use bitflags
    // #region PPUMASK $2001
    /// Emphasize red, green, blue
    color_emphasis_bgr: u8,
    /// 1: Show sprites
    sprite_enable: u8,
    /// 1: Show background
    background_enable: u8,
    /// 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
    sprite_left_column_enable: u8,
    /// 1: Show background in leftmost 8 pixels of screen, 0: Hide
    background_left_column_enable: u8,
    /// Greyscale (0: normal color, 1: produce a greyscale display)
    greyscale: u8,
    // #endregion

    // TODO: Use bitflags
    // #region PPUSTATUS $2002
    /// Vertical blank has started.
    vblank: u8,
    /// Sprite 0 Hit. Set when a nonzero pixel of sprite 0 overlaps a nonzero background pixel.
    sprite0Hit: u8,
    /// Sprite overflow.
    spriteOverflow: u8,
    // #endregion
    /// OAM address port
    pub oamAddress: u8,

    // https://wiki.nesdev.com/w/index.php/PPU_scrolling#PPU_internal_registers
    /// Current VRAM address (15 bits)
    v: u16,
    /// Temporary VRAM address (15 bits); can also be thought of as the address of the top left onscreen tile.
    t: u16,
    /// Fine X scroll (3 bits)
    x: u8,
    /// First or second write toggle (1 bit)
    w: bool,

    pub output: [u8; 256 * 240], // 256x240 pixels
    vram: [u8; 0x4000],          // 16kb
    pub paletteRAM: [u8; 32],
    vramBuffer: u8,

    dot: u16,
    scanline: u16,

    // #region Background
    ntTileLatch: u8,
    loPatternLatch: u8,
    hiPatternLatch: u8,
    atTileLatch: u8,

    lo_at_Latch: u16,
    hi_at_Latch: u16,
    lo_bitmap_Latch: u16,
    hi_bitmap_Latch: u16,
    // bitmap: u32,
    // palette: u16,
    // #endregion

    // #region Sprites
    /// Primary OAM (holds 64 sprites for the frame)
    pub oamMemory: [u8; 64 * 4],
    /// Secondary OAM (holds 8 sprites for the current scanline)
    pub secondaryOamMemory: [u8; 8 * 4],

    /// 8 pairs of 8-bit shift registers - These contain the bitmap data for up to 8 sprites,
    /// to be rendered on the current scanline. Unused sprites are loaded with an all-transparent bitmap.
    spritePatternLo: [u8; 8],
    spritePatternHi: [u8; 8],
    /// 8 latches - These contain the attribute bytes for up to 8 sprites.
    spriteAttributes: [u8; 8],
    /// 8 counters - These contain the X positions for up to 8 sprites.
    spriteXPos: [i16; 8],
    // #endregion
    #[allow(dead_code)]
    oamData: u8,
    n: u8,
    spriteCount: u8,

    cpu: Rc<RefCell<CPU6502>>,
    #[allow(dead_code)]
    bus: Rc<RefCell<DataBus>>,
}

impl Ppu {
    pub fn new(
        cpu: Rc<RefCell<CPU6502>>,
        bus: Rc<RefCell<DataBus>>,
        rom: Rc<RefCell<RomFile>>,
    ) -> Self {
        let mut ppu = Ppu {
            nmi_enable: 0,
            ppu_master_slave: 0,
            sprite_height: 0,
            background_tile_select: 0,
            sprite_tile_select: 0,
            increment_mode: 0,
            nametable_select: 0,
            color_emphasis_bgr: 0,
            sprite_enable: 0,
            background_enable: 0,
            sprite_left_column_enable: 0,
            background_left_column_enable: 0,
            greyscale: 0,
            vblank: 0,
            sprite0Hit: 0,
            spriteOverflow: 0,
            oamAddress: 0,
            v: 0,
            t: 0,
            x: 0,
            w: false,
            output: [0; 256 * 240],
            vram: [0; 0x4000],
            paletteRAM: [0; 32],
            vramBuffer: 0,
            dot: 0,
            scanline: 261, // Equivalent to -1
            ntTileLatch: 0,
            atTileLatch: 0,
            lo_at_Latch: 0,
            hi_at_Latch: 0,
            lo_bitmap_Latch: 0,
            hi_bitmap_Latch: 0,
            loPatternLatch: 0,
            hiPatternLatch: 0,
            oamMemory: [0; 64 * 4],
            secondaryOamMemory: [0; 8 * 4],
            spritePatternLo: [0; 8],
            spritePatternHi: [0; 8],
            spriteAttributes: [0; 8],
            spriteXPos: [0; 8],
            oamData: 0,
            n: 0,
            spriteCount: 0,
            cpu,
            bus,
        };

        let r = rom.as_ref().borrow();

        ppu.vram[0..0x2000].copy_from_slice(&r.chr_data[0..0x2000]);

        ppu
    }

    pub fn read_vram(&self, mut addr: u16) -> u8 {
        match addr {
            0x3000..=0x3EFF => self.vram[(addr as usize - 0x1000) & 0x3FFF], // Mirrors of $2000-$2EFF
            0x3F00..=0x3FFF => {
                // Palette RAM indexes
                if addr % 4 == 0 {
                    addr &= 0x0F
                }
                self.paletteRAM[addr as usize & 0x1F]
            }
            _ => self.vram[addr as usize & 0x3FFF],
        }
    }

    pub fn write_vram(&mut self, mut addr: u16, value: u8) {
        match addr {
            0x3000..=0x3EFF => self.vram[(addr as usize - 0x1000) & 0x3FFF] = value, // Mirrors of $2000-$2EFF
            0x3F00..=0x3FFF => {
                // Palette RAM indexes
                if addr % 4 == 0 {
                    addr &= 0x0F;
                }
                self.paletteRAM[addr as usize & 0x1F] = value;
            }
            _ => self.vram[addr as usize & 0x3FFF] = value,
        }
    }

    /// https://wiki.nesdev.com/w/index.php/PPU_sprite_evaluation
    /// Based off on https://github.com/ulfalizer/nesalizer/blob/master/src/ppu.cpp
    fn sprite_evaluation(&mut self) {
        if self.dot != 256 {
            return;
        }

        let mut sprite_count = 0;
        #[allow(unused_variables)]
        let sprite_overflow = 0;
        let y_pos = self.scanline;

        for i in self.oamAddress as usize..256 {
            if i % 4 != 0 {
                continue;
            }

            let sprite_y_top = self.oamMemory[i];
            let mut offset = y_pos as i16 - sprite_y_top as i16;

            // If this sprite is on the next scanline, copy it to the _sprites array for rendering
            if offset < self.sprite_height as i16 && offset >= 0 {
                if sprite_count == 8 {
                    // sprite_overflow = 1;
                } else {
                    let sprite_idx = self.oamMemory[i + 1] as u16;
                    self.spriteAttributes[sprite_count] = self.oamMemory[i + 2];

                    if (self.spriteAttributes[sprite_count] & 0x80) != 0 {
                        // Flip vertically
                        offset = 7 - offset;
                    }

                    self.spritePatternLo[sprite_count] = self
                        .read_vram((sprite_idx * 16 + 0 + offset as u16) | self.sprite_tile_select);
                    self.spritePatternHi[sprite_count] = self
                        .read_vram((sprite_idx * 16 + 8 + offset as u16) | self.sprite_tile_select);
                    self.spriteXPos[sprite_count] = self.oamMemory[i + 3] as i16;
                    sprite_count += 1;
                }
            }
        }
    }

    /// https://wiki.nesdev.com/w/index.php/PPU_scrolling#Coarse_X_increment
    fn inc_horizontal(&mut self) {
        if (self.v & 0x001F) == 31 {
            // if coarse X == 31
            self.v &= !0x001F; // coarse X = 0
            self.v ^= 0x0400; // switch horizontal nametable
        } else {
            self.v += 1; // increment coarse X
        }
    }

    /// https://wiki.nesdev.com/w/index.php/PPU_scrolling#Y_increment
    fn inc_vertical(&mut self) {
        if (self.v & 0x7000) != 0x7000 {
            // if fine Y < 7
            self.v += 0x1000; // increment fine Y
        } else {
            self.v &= !0x7000; // fine Y = 0
            let mut y = (self.v & 0x03E0) >> 5; // let y = coarse Y
            if y == 29 {
                // y = 0; // coarse Y = 0
                self.v ^= 0x0800; // switch vertical nametable
            } else if y == 31 {
                // y = 0; // coarse Y = 0, nametable not switched
            } else {
                y += 1; // increment coarse Y
                self.v &= !0x03E0;
                self.v |= y << 5; // put coarse Y back into v
            }
        }
    }

    fn render_background(&mut self) {
        let x_pos = self.dot - 1;
        let y_pos = self.scanline;
        let fine_x = self.x as u16;

        // let mut pixel = ((self.bitmap >> (fine_x * 2)) & 0x3) as u8;
        let pixel_hi = (self.hi_bitmap_Latch >> fine_x) & 0x01;
        let pixel_lo = (self.lo_bitmap_Latch >> fine_x) & 0x01;
        let pixel = (pixel_hi << 1) | pixel_lo;

        let color = if pixel == 0 {
            self.read_vram(0x3F00) // Background color
        } else {
            let at_data_lo = (self.lo_at_Latch >> fine_x) & 0x01;
            let at_data_hi = (self.hi_at_Latch >> fine_x) & 0x01;
            let at_data = (at_data_hi << 3) | (at_data_lo << 2);

            self.read_vram(0x3F00 | at_data | pixel)
        };

        self.output[(y_pos * 256 + x_pos) as usize] = color;

        self.lo_at_Latch >>= 1;
        self.hi_at_Latch >>= 1;
        self.lo_bitmap_Latch >>= 1;
        self.hi_bitmap_Latch >>= 1;
    }

    fn render_sprites(&mut self) {
        let x_pos = self.dot - 1;
        let y_pos = self.scanline;

        for i in 0..self.spriteXPos.len() {
            self.spriteXPos[i] = unchecked_sub!(self.spriteXPos[i], 1);
            if self.spriteXPos[i] <= 0 && self.spriteXPos[i] > -8 {
                // Sprite becomes active
                let mut bit_index = 7 + self.spriteXPos[i];

                if (self.spriteAttributes[i] & 0x40) != 0 {
                    // Flip horizontally
                    bit_index = 7 - bit_index;
                }

                let pattern_lo = (self.spritePatternLo[i] >> bit_index) & 1;
                let pattern_hi = (self.spritePatternHi[i] >> bit_index) & 1;
                let pattern = (pattern_hi << 1) | pattern_lo;

                if pattern != 0 {
                    let at_data = (self.spriteAttributes[i] & 0x3) + 4;
                    let color = self.read_vram(0x3F00 + (at_data << 2) as u16 + pattern as u16);

                    // https://wiki.nesdev.com/w/index.php?title=PPU_OAM&redirect=no#Sprite_zero_hits
                    if i == 0 && // Sprite 0
                        self.sprite0Hit == 0 &&
                        self.background_enable != 0 && // Background rendering enabled
                        self.sprite_enable != 0 && // Sprites rendering enabled
                        // At x=0 to x=7 if the left-side clipping window is enabled (if bit 2 or bit 1 of PPUMASK is 0).
                        !((x_pos == 0 || x_pos == 7) && (self.background_left_column_enable == 0 || self.sprite_left_column_enable ==0)) &&
                        x_pos != 255 && // At x=255, for an obscure reason related to the pixel pipeline.
                        color & 0x03 != 0x00 && // Sprite non-transparent
                        self.output[(y_pos * 256 + x_pos) as usize] & 0x03 != 0x00
                    {
                        self.sprite0Hit = 1;
                    }

                    self.output[(y_pos * 256 + x_pos) as usize] = color;
                }
            }
        }
    }

    fn inc_dot(&mut self) {
        let pre_render_line = self.scanline == 261;

        if self.dot == 1 && self.scanline == 241 {
            self.vblank = 1;
            if self.nmi_enable != 0 {
                self.cpu.borrow_mut().request_nmi();
            }
        }

        if pre_render_line && self.dot == 1 {
            self.vblank = 0;
            self.sprite0Hit = 0;
            self.spriteOverflow = 0;
        }

        self.dot += 1;

        if self.dot > 341 {
            self.dot = 0;
            self.scanline += 1;
        }

        if self.scanline > 261 {
            self.scanline = 0;
        }
    }

    /// https://wiki.nesdev.com/w/index.php/File:Ntsc_timing.png
    pub fn tick(&mut self) {
        let x_pos = self.dot as i32 - 1;
        let y_pos = self.scanline as i32;

        let rendering_enabled = (self.background_enable != 0) || (self.sprite_enable != 0);
        let phase = self.dot % 8;
        let render_cycle = self.dot >= 1 && self.dot <= 256;
        let visible_scanline = self.scanline <= 239;
        let pre_render_line = self.scanline == 261;
        let fetch_scanline = visible_scanline || pre_render_line;
        let fetch_cycle = fetch_scanline && (render_cycle || self.dot >= 321);
        let fine_y = (self.v >> 12) & 0x7;

        if !rendering_enabled && visible_scanline && render_cycle {
            // Background color
            self.output[(y_pos * 256 + x_pos) as usize] = self.read_vram(0x3F00);
        }

        if rendering_enabled {
            if render_cycle && visible_scanline {
                self.render_background();
                self.render_sprites();
            }

            if fetch_cycle {
                match phase {
                    1 => {
                        let bitmap = ((self.atTileLatch
                            >> (((self.v >> 4) & 4) | (unchecked_sub!(self.v, 1) & 2)))
                            & 0x03) as u16;

                        // https://forums.nesdev.com/viewtopic.php?f=3&t=10348
                        self.lo_at_Latch |= (bitmap & 1) * 0xFF00;
                        self.hi_at_Latch |= (bitmap >> 1) * 0xFF00;
                        self.lo_bitmap_Latch |= (reverse_bits(self.loPatternLatch) as u16) << 8;
                        self.hi_bitmap_Latch |= (reverse_bits(self.hiPatternLatch) as u16) << 8;

                        // Fetch a nametable entry from $2000-$2FBF.
                        self.ntTileLatch = self.read_vram(0x2000 | (self.v & 0x0FFF));
                    }

                    3 => {
                        // Fetch the corresponding attribute table entry from $23C0-$2FFF and increment the current VRAM address within the same row.
                        // https://wiki.nesdev.com/w/index.php/PPU_scrolling#Tile_and_attribute_fetching
                        self.atTileLatch = self.read_vram(
                            0x23C0
                                | (self.v & 0x0C00)
                                | ((self.v >> 4) & 0x38)
                                | ((self.v >> 2) & 0x07),
                        );
                    }

                    5 => {
                        // Fetch the low-order byte of an 8x1 pixel sliver of pattern table from $0000-$0FF7 or $1000-$1FF7.
                        let addr =
                            self.background_tile_select | ((self.ntTileLatch as u16) * 16 + fine_y);
                        self.loPatternLatch = self.read_vram(addr);
                    }

                    7 => {
                        // Fetch the high-order byte of this sliver from an address 8 bytes higher.
                        let addr = self.background_tile_select
                            | ((self.ntTileLatch as u16) * 16 + fine_y + 8);
                        self.hiPatternLatch = self.read_vram(addr);
                    }

                    0 => {
                        self.inc_horizontal();
                        if self.dot == 256 {
                            self.inc_vertical();
                        }
                    }

                    _ => {}
                }
            }

            // #region Sprites
            if visible_scanline || pre_render_line {
                match self.dot {
                    1 => {
                        // Clear OAM
                        for i in 0..self.secondaryOamMemory.len() {
                            self.secondaryOamMemory[i] = 0xFF;
                        }
                        self.n = 0;
                        self.spriteCount = 0;
                    }
                    65..=256 => self.sprite_evaluation(),
                    257..=320 => {}     // Sprite fetches
                    321..=340 | 0 => {} //  Background render pipeline initialization
                    _ => {}
                }
            }
            // #endregion

            if self.dot == 257 {
                // hori (v) = hori (t)
                self.v = (self.v & 0x7BE0) | (self.t & 0x041F);
            }
            if self.dot >= 280 && self.dot <= 304 && pre_render_line {
                // vert (v) = vert (t)
                self.v = (self.v & 0x041F) | (self.t & 0x7BE0);
            }
        }

        self.inc_dot();
    }

    // #region CPU mapped registers
    pub fn write_register_cpu_address(&mut self, address: u16, value: u8) {
        match address {
            0x2000 => {
                // PPUCTRL $2000 VPHB SINN
                self.nmi_enable = (value >> 7) & 1;
                self.ppu_master_slave = (value >> 6) & 1;
                self.sprite_height = ((value >> 5) & 1) * 8 + 8; // 8x16 or 8x8
                self.background_tile_select = ((value as u16 >> 4) & 1) * 0x1000;
                self.sprite_tile_select = ((value as u16 >> 3) & 1) * 0x1000;
                self.increment_mode = ((value as u16 >> 2) & 1) * 31 + 1;
                self.nametable_select = (value as u16 & 0x03) * 0x400 + 0x2000; // !MIGHT REMOVE
                self.t = (self.t & 0xF3FF) | ((value as u16 & 0x3) << 10);
            }

            0x2001 => {
                // PPUMASK $2001 BGRs bMmG
                self.color_emphasis_bgr = (value >> 5) & 3;
                self.sprite_enable = (value >> 4) & 1;
                self.background_enable = (value >> 3) & 1;
                self.sprite_left_column_enable = (value >> 2) & 1;
                self.background_left_column_enable = (value >> 1) & 1;
                self.greyscale = value & 1;
            }

            0x2002 => {
                // Debug.Log("PPUSTATUS $2002 is read only!");
            }

            0x2003 => {
                // OAMADDR $2003 aaaa aaaa
                self.oamAddress = value;
            }

            0x2004 => {
                // OAMDATA $2004 dddd dddd
                self.oamMemory[self.oamAddress as usize] = value;
                self.oamAddress = unchecked_add!(self.oamAddress, 1);
            }

            0x2005 => {
                // PPUSCROLL $2005 xxxx xxxx
                if !self.w {
                    self.t = (self.t & 0xFFE0) | ((value as u16 >> 0x3) & 0x1F);
                    self.x = value & 0b111;
                } else {
                    self.t = (self.t & 0xFC1F) | ((value as u16 & 0xF8) << 2);
                    self.t = (self.t & 0xF3FF) | ((value as u16 & 0x03) << 15);
                }
                self.w = !self.w;
            }

            0x2006 => {
                // PPUADDR $2006 aaaa aaaa
                if !self.w {
                    self.t = (self.t & 0x00FF) | ((value as u16) << 8);
                } else {
                    self.t = (self.t & 0xFF00) | value as u16;
                    self.v = self.t;
                }
                self.w = !self.w;
            }

            0x2007 => {
                // PPUDATA $2007 dddd dddd
                self.write_vram(self.v, value);
                self.v += self.increment_mode as u16;
                self.v &= 0x3FFF;
            }

            _ => unreachable!(),
        };
    }

    pub fn read_register_cpu_address(&mut self, address: u16) -> u8 {
        match address {
            0x2000 => 0, // PPUCTRL $2000 is write only!
            0x2001 => 0, // PPUMASK $2001 is write only!
            0x2002 => {
                // PPUSTATUS $2002 VSO- ----
                self.w = false;
                (self.vblank << 7) | (self.sprite0Hit << 6) | (self.spriteOverflow << 5)
            }
            0x2003 => 0, // OAMADDR $2003 is write only!
            0x2004 => self.oamMemory[self.oamAddress as usize], // OAMDATA $2004 dddd dddd
            0x2005 => 0, // PPUSCROLL $2005 is write only!
            0x2006 => 0, // PPUADDR $2006 is write only!
            0x2007 => {
                // PPUDATA $2007 dddd dddd
                let ret = self.vramBuffer;
                self.vramBuffer = self.read_vram(self.v);
                self.v += self.increment_mode;
                self.v &= 0x3FFF;
                if self.v >= 0x3F00 {
                    self.vramBuffer
                } else {
                    ret
                }
            }
            _ => unreachable!(),
        }
    }
    // #endregion
}

#[inline]
fn reverse_bits(value: u8) -> u8 {
    // https://stackoverflow.com/questions/2602823/in-c-c-whats-the-simplest-way-to-reverse-the-order-of-bits-in-a-byte
    static LOOKUP_TABLE: [u8; 256] = [
        0x00, 0x80, 0x40, 0xC0, 0x20, 0xA0, 0x60, 0xE0, 0x10, 0x90, 0x50, 0xD0, 0x30, 0xB0, 0x70,
        0xF0, 0x08, 0x88, 0x48, 0xC8, 0x28, 0xA8, 0x68, 0xE8, 0x18, 0x98, 0x58, 0xD8, 0x38, 0xB8,
        0x78, 0xF8, 0x04, 0x84, 0x44, 0xC4, 0x24, 0xA4, 0x64, 0xE4, 0x14, 0x94, 0x54, 0xD4, 0x34,
        0xB4, 0x74, 0xF4, 0x0C, 0x8C, 0x4C, 0xCC, 0x2C, 0xAC, 0x6C, 0xEC, 0x1C, 0x9C, 0x5C, 0xDC,
        0x3C, 0xBC, 0x7C, 0xFC, 0x02, 0x82, 0x42, 0xC2, 0x22, 0xA2, 0x62, 0xE2, 0x12, 0x92, 0x52,
        0xD2, 0x32, 0xB2, 0x72, 0xF2, 0x0A, 0x8A, 0x4A, 0xCA, 0x2A, 0xAA, 0x6A, 0xEA, 0x1A, 0x9A,
        0x5A, 0xDA, 0x3A, 0xBA, 0x7A, 0xFA, 0x06, 0x86, 0x46, 0xC6, 0x26, 0xA6, 0x66, 0xE6, 0x16,
        0x96, 0x56, 0xD6, 0x36, 0xB6, 0x76, 0xF6, 0x0E, 0x8E, 0x4E, 0xCE, 0x2E, 0xAE, 0x6E, 0xEE,
        0x1E, 0x9E, 0x5E, 0xDE, 0x3E, 0xBE, 0x7E, 0xFE, 0x01, 0x81, 0x41, 0xC1, 0x21, 0xA1, 0x61,
        0xE1, 0x11, 0x91, 0x51, 0xD1, 0x31, 0xB1, 0x71, 0xF1, 0x09, 0x89, 0x49, 0xC9, 0x29, 0xA9,
        0x69, 0xE9, 0x19, 0x99, 0x59, 0xD9, 0x39, 0xB9, 0x79, 0xF9, 0x05, 0x85, 0x45, 0xC5, 0x25,
        0xA5, 0x65, 0xE5, 0x15, 0x95, 0x55, 0xD5, 0x35, 0xB5, 0x75, 0xF5, 0x0D, 0x8D, 0x4D, 0xCD,
        0x2D, 0xAD, 0x6D, 0xED, 0x1D, 0x9D, 0x5D, 0xDD, 0x3D, 0xBD, 0x7D, 0xFD, 0x03, 0x83, 0x43,
        0xC3, 0x23, 0xA3, 0x63, 0xE3, 0x13, 0x93, 0x53, 0xD3, 0x33, 0xB3, 0x73, 0xF3, 0x0B, 0x8B,
        0x4B, 0xCB, 0x2B, 0xAB, 0x6B, 0xEB, 0x1B, 0x9B, 0x5B, 0xDB, 0x3B, 0xBB, 0x7B, 0xFB, 0x07,
        0x87, 0x47, 0xC7, 0x27, 0xA7, 0x67, 0xE7, 0x17, 0x97, 0x57, 0xD7, 0x37, 0xB7, 0x77, 0xF7,
        0x0F, 0x8F, 0x4F, 0xCF, 0x2F, 0xAF, 0x6F, 0xEF, 0x1F, 0x9F, 0x5F, 0xDF, 0x3F, 0xBF, 0x7F,
        0xFF,
    ];
    LOOKUP_TABLE[value as usize]
}

#[test]
fn bit_reversal() {
    for original in 0..=0xFF {
        let mut b = original;
        b = (b & 0xF0) >> 4 | (b & 0x0F) << 4;
        b = (b & 0xCC) >> 2 | (b & 0x33) << 2;
        b = (b & 0xAA) >> 1 | (b & 0x55) << 1;
        assert_eq!(reverse_bits(original), b);
    }
}

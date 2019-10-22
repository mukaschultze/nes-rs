use crate::cpu::CPU6502;
use crate::rom::mapper::Mapper;
use std::cell::RefCell;
use std::rc::Rc;

bitflags! {
    /// Various flags controlling PPU operation
    /// https://wiki.nesdev.com/w/index.php/PPU_registers#Controller_.28.242000.29_.3E_write
    #[derive(Default)]
    struct PPUCTRL: u8 {
        /// Base nametable address (0 = 0x2000; 1 = 0x2400; 2 = 0x2800; 3 = 0x2C00)
        const NAME_TABLE = 0b0000_0011;
        /// VRAM address increment per CPU read/write of PPUDATA (0: add 1, going across; 1: add 32, going down)
        const VRAM_INCREMENT = 0b0000_0100;
        /// Sprite pattern table address for 8x8 sprites (0: 0x0000; 1: 0x1000; ignored in 8x16 mode)
        const SPRITE_TILE_SELECT =0b0000_1000;
        /// Background pattern table address (0: 0x0000; 1: 0x1000)
        const BG_TILE_SELECT =0b0001_0000;
        /// Sprite size (0: 8x8 pixels; 1: 8x16 pixels)
        const SPRITE_HEIGHT = 0b0010_0000;
        /// PPU master/slave select (0: read backdrop from EXT pins; 1: output color on EXT pins)
        const PPU_MASTER_SLAVE = 0b0100_0000;
        /// Generate an NMI at the start of the vertical blanking interval (0: off; 1: on)
        const NMI_ENABLE = 0b1000_0000;
    }
}

impl PPUCTRL {
    fn vram_increment(self) -> u16 {
        if self.contains(Self::VRAM_INCREMENT) {
            32
        } else {
            1
        }
    }

    fn sprite_tile_select(self) -> u16 {
        ((self & Self::SPRITE_TILE_SELECT).bits() as u16) << 9
    }

    fn bg_tile_select(self) -> u16 {
        ((self & Self::BG_TILE_SELECT).bits() as u16) << 8
    }

    fn sprite_height(self) -> u8 {
        8 << (self & Self::SPRITE_HEIGHT).bits()
    }
}

bitflags! {
    /// This register controls the rendering of sprites and backgrounds, as well as colour effects.
    /// https://wiki.nesdev.com/w/index.php/PPU_registers#PPUMASK
    #[derive(Default)]
    struct PPUMASK: u8 {
        /// Greyscale (0: normal color, 1: produce a greyscale display)
        const GREYSCALE = 0b0000_0001;
        /// 1: Show background in leftmost 8 pixels of screen, 0: Hide
        const BACKGROUND_LEFTMOST_COLUMN = 0b0000_0010;
        /// 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
        const SPRITE_LEFTMOST_COLUMN = 0b0000_0100;
        /// 1: Show background
        const BACKGROUND_ENABLE = 0b0000_1000;
        /// 1: Show sprites
        const SPRITE_ENABLE = 0b0001_0000;
        /// Emphasize red, green, blue
        const COLOR_EMPHASIS_BGR = 0b1110_0000;
    }
}

bitflags! {
    /// This register reflects the state of various functions inside the PPU.
    /// It is often used for determining timing. To determine when the PPU has reached
    /// a given pixel of the screen, put an opaque (non-transparent) pixel of sprite 0 there.
    /// https://wiki.nesdev.com/w/index.php/PPU_registers#PPUSTATUS
    #[derive(Default)]
    struct PPUSTATUS: u8 {
        /// Sprite overflow.
        const SPRITE_OVERFLOW = 0b0010_0000;
        /// Sprite 0 Hit. Set when a nonzero pixel of sprite 0 overlaps a nonzero background pixel.
        const SPRITE_0_HIT = 0b0100_0000;
        /// Vertical blank has started.
        const V_BLANK = 0b1000_0000;
    }
}

/// https://wiki.nesdev.com/w/index.php/PPU
pub struct Ppu {
    ppuctrl: PPUCTRL,
    ppumask: PPUMASK,
    ppustatus: PPUSTATUS,
    /// OAM address port
    pub oam_address: u8,

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
    vram: [u8; 0x1000],          // 2kb // TODO: Implement mirroring and fix the VRAM size
    pub palette_vram: [u8; 32],
    vram_buffer: u8,

    pub dot: u16,
    pub scanline: u16,

    // #region Background
    nt_latch: u8,
    at_latch: u8,
    pattern_latch_lo: u8,
    pattern_latch_hi: u8,

    lo_at_reg: u16,
    hi_at_reg: u16,
    lo_bitmap_reg: u16,
    hi_bitmap_reg: u16,
    // #endregion

    // #region Sprites
    /// Primary OAM (holds 64 sprites for the frame)
    pub oam_memory: [u8; 64 * 4],
    /// Secondary OAM (holds 8 sprites for the current scanline)
    pub secondary_oam_memory: [u8; 8 * 4],

    /// 8 pairs of 8-bit shift registers - These contain the bitmap data for up to 8 sprites,
    /// to be rendered on the current scanline. Unused sprites are loaded with an all-transparent bitmap.
    sprite_pattern_lo: [u8; 8],
    sprite_pattern_hi: [u8; 8],
    /// 8 latches - These contain the attribute bytes for up to 8 sprites.
    sprite_at: [u8; 8],
    /// 8 counters - These contain the X positions for up to 8 sprites.
    sprite_x_pos: [i16; 8],
    // #endregion
    sprite_count: u8,

    cpu: Rc<RefCell<CPU6502>>,

    pub mapper: Option<Rc<RefCell<dyn Mapper>>>,
    pub v_blank_callback: Box<dyn FnMut()>,
}

impl Ppu {
    pub fn new(cpu: Rc<RefCell<CPU6502>>) -> Self {
        Ppu {
            ppuctrl: Default::default(),
            ppumask: Default::default(),
            ppustatus: Default::default(),
            oam_address: 0,
            v: 0,
            t: 0,
            x: 0,
            w: false,
            output: [0; 256 * 240],
            vram: [0; 0x1000],
            palette_vram: [0; 32],
            vram_buffer: 0,
            dot: 0,
            scanline: 261, // Equivalent to -1
            nt_latch: 0,
            at_latch: 0,
            lo_at_reg: 0,
            hi_at_reg: 0,
            lo_bitmap_reg: 0,
            hi_bitmap_reg: 0,
            pattern_latch_lo: 0,
            pattern_latch_hi: 0,
            oam_memory: [0; 64 * 4],
            secondary_oam_memory: [0; 8 * 4],
            sprite_pattern_lo: [0; 8],
            sprite_pattern_hi: [0; 8],
            sprite_at: [0; 8],
            sprite_x_pos: [0; 8],
            sprite_count: 0,
            cpu,
            mapper: None,
            v_blank_callback: Box::new(|| {}),
        }
    }

    pub fn read_vram(&mut self, mut addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => {
                if let Some(mapper) = self.mapper.as_mut() {
                    mapper.borrow_mut().read_chr(addr)
                } else {
                    0
                }
            }
            0x2000..=0x2FFF => self.vram[(addr - 0x2000) as usize],
            0x3000..=0x3EFF => self.vram[(addr - 0x3000) as usize], // Mirrors of $2000-$2EFF
            0x3F00..=0x3FFF => {
                // Palette RAM indexes
                if addr % 4 == 0 {
                    addr &= 0x0F
                }
                self.palette_vram[addr as usize & 0x1F]
            }
            0x4000..=0xFFFF => unreachable!(),
        }
    }

    pub fn write_vram(&mut self, mut addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => {
                if let Some(mapper) = self.mapper.as_mut() {
                    mapper.borrow_mut().write_chr(addr, value);
                }
            }
            0x2000..=0x2FFF => self.vram[(addr - 0x2000) as usize] = value,
            0x3000..=0x3EFF => self.vram[(addr - 0x3000) as usize] = value, // Mirrors of $2000-$2EFF
            0x3F00..=0x3FFF => {
                // Palette RAM indexes
                if addr % 4 == 0 {
                    addr &= 0x0F;
                }
                self.palette_vram[addr as usize & 0x1F] = value;
            }
            0x4000..=0xFFFF => unreachable!(),
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

        for i in self.oam_address as usize..256 {
            if i % 4 != 0 {
                continue;
            }

            let sprite_y_top = self.oam_memory[i];
            let mut offset = y_pos as i16 - sprite_y_top as i16;

            let sprite_height = self.ppuctrl.sprite_height() as i16;
            let sprite_tile_select = self.ppuctrl.sprite_tile_select();

            // If this sprite is on the next scanline, copy it to the _sprites array for rendering
            if offset < sprite_height && offset >= 0 {
                if sprite_count == 8 {
                    // sprite_overflow = 1;
                } else {
                    let sprite_idx = self.oam_memory[i + 1] as u16;
                    self.sprite_at[sprite_count] = self.oam_memory[i + 2];

                    // Flip vertically
                    if (self.sprite_at[sprite_count] & 0x80) != 0 {
                        offset = 7 - offset;
                    }

                    let addr = (sprite_idx * 16 + offset as u16) | sprite_tile_select;
                    let mut pattern_lo = self.read_vram(addr);
                    let mut pattern_hi = self.read_vram(addr + 8);
                    self.sprite_x_pos[sprite_count] = self.oam_memory[i + 3] as i16;

                    // Flip horizontally
                    if (self.sprite_at[sprite_count] & 0x40) != 0 {
                        pattern_lo = reverse_bits(pattern_lo);
                        pattern_hi = reverse_bits(pattern_hi);
                    }

                    self.sprite_pattern_lo[sprite_count] = pattern_lo;
                    self.sprite_pattern_hi[sprite_count] = pattern_hi;

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
        let pixel_hi = (self.hi_bitmap_reg >> fine_x) & 0x01;
        let pixel_lo = (self.lo_bitmap_reg >> fine_x) & 0x01;
        let pixel = (pixel_hi << 1) | pixel_lo;

        let color = if pixel == 0 {
            self.read_vram(0x3F00) // Background color
        } else {
            let at_data_lo = (self.lo_at_reg >> fine_x) & 0x01;
            let at_data_hi = (self.hi_at_reg >> fine_x) & 0x01;
            let at_data = (at_data_hi << 3) | (at_data_lo << 2);

            self.read_vram(0x3F00 | at_data | pixel)
        };

        self.output[(y_pos * 256 + x_pos) as usize] = color;

        self.lo_at_reg >>= 1;
        self.hi_at_reg >>= 1;
        self.lo_bitmap_reg >>= 1;
        self.hi_bitmap_reg >>= 1;
    }

    fn render_sprites(&mut self) {
        let x_pos = self.dot - 1;
        let y_pos = self.scanline;

        for i in 0..self.sprite_x_pos.len() {
            self.sprite_x_pos[i] = unchecked_sub!(self.sprite_x_pos[i], 1);
            if self.sprite_x_pos[i] <= 0 && self.sprite_x_pos[i] > -8 {
                // Sprite becomes active
                let bit_idx = 7 + self.sprite_x_pos[i];
                let pattern_lo = (self.sprite_pattern_lo[i] >> bit_idx) & 1;
                let pattern_hi = (self.sprite_pattern_hi[i] >> bit_idx) & 1;
                let pattern = (pattern_hi << 1) | pattern_lo;

                if pattern != 0 {
                    let at_data = (self.sprite_at[i] & 0x3) + 4;
                    let color = self.read_vram(0x3F00 + (at_data << 2) as u16 + pattern as u16);

                    // https://wiki.nesdev.com/w/index.php?title=PPU_OAM&redirect=no#Sprite_zero_hits
                    if i == 0 && // Sprite 0
                        !self.ppustatus.contains(PPUSTATUS::SPRITE_0_HIT) &&
                        self.ppumask.contains(PPUMASK::BACKGROUND_ENABLE) && // Background rendering enabled
                        self.ppumask.contains(PPUMASK::SPRITE_ENABLE) && // Sprites rendering enabled
                        // At x=0 to x=7 if the left-side clipping window is enabled (if bit 2 or bit 1 of PPUMASK is 0).
                        !((x_pos == 0 || x_pos == 7) && (!self.ppumask.contains(PPUMASK::BACKGROUND_LEFTMOST_COLUMN) || !self.ppumask.contains(PPUMASK::SPRITE_LEFTMOST_COLUMN))) &&
                        x_pos != 255 && // At x=255, for an obscure reason related to the pixel pipeline.
                        color & 0x03 != 0x00 && // Sprite non-transparent

                        // TODO: Use mux to check if sprite is transparent
                        // !This check is incorrect, & 0x03 should be checked against the bitmap
                        self.output[(y_pos * 256 + x_pos) as usize] & 0x03 != 0x00
                    {
                        self.ppustatus.set(PPUSTATUS::SPRITE_0_HIT, true);
                    }

                    let priority = self.sprite_at[i] & 0x20; // 0: in front of background; 1: behind background

                    // TODO: Use mux to check if sprite is transparent
                    if priority == 0
                        || self.output[(y_pos * 256 + x_pos) as usize] == self.palette_vram[0]
                    {
                        self.output[(y_pos * 256 + x_pos) as usize] = color;
                    }
                }
            }
        }
    }

    fn inc_dot(&mut self) {
        let pre_render_line = self.scanline == 261;

        if self.dot == 1 && self.scanline == 241 {
            self.ppustatus.set(PPUSTATUS::V_BLANK, true);
            if self.ppuctrl.contains(PPUCTRL::NMI_ENABLE) {
                self.cpu.borrow_mut().request_nmi();
            }

            self.v_blank_callback.as_mut()();
        }

        if pre_render_line && self.dot == 1 {
            self.ppustatus.set(PPUSTATUS::V_BLANK, false);
            self.ppustatus.set(PPUSTATUS::SPRITE_0_HIT, false);
            self.ppustatus.set(PPUSTATUS::SPRITE_OVERFLOW, false);
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

        let rendering_enabled = self.ppumask.contains(PPUMASK::BACKGROUND_ENABLE)
            || self.ppumask.contains(PPUMASK::SPRITE_ENABLE);
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
                        let bitmap = ((self.at_latch
                            >> (((self.v >> 4) & 4) | (unchecked_sub!(self.v, 1) & 2)))
                            & 0x03) as u16;

                        // https://forums.nesdev.com/viewtopic.php?f=3&t=10348
                        self.lo_at_reg |= (bitmap & 1) * 0xFF00;
                        self.hi_at_reg |= (bitmap >> 1) * 0xFF00;
                        self.lo_bitmap_reg |= (reverse_bits(self.pattern_latch_lo) as u16) << 8;
                        self.hi_bitmap_reg |= (reverse_bits(self.pattern_latch_hi) as u16) << 8;

                        // Fetch a nametable entry from $2000-$2FBF.
                        self.nt_latch = self.read_vram(0x2000 | (self.v & 0x0FFF));
                    }

                    3 => {
                        // Fetch the corresponding attribute table entry from $23C0-$2FFF and increment the current VRAM address within the same row.
                        // https://wiki.nesdev.com/w/index.php/PPU_scrolling#Tile_and_attribute_fetching
                        self.at_latch = self.read_vram(
                            0x23C0
                                | (self.v & 0x0C00)
                                | ((self.v >> 4) & 0x38)
                                | ((self.v >> 2) & 0x07),
                        );
                    }

                    5 => {
                        // Fetch the low-order byte of an 8x1 pixel sliver of pattern table from $0000-$0FF7 or $1000-$1FF7.
                        let bg_tile_select = self.ppuctrl.bg_tile_select();
                        let addr = bg_tile_select | ((self.nt_latch as u16) * 16 + fine_y);
                        self.pattern_latch_lo = self.read_vram(addr);
                    }

                    7 => {
                        // Fetch the high-order byte of this sliver from an address 8 bytes higher.
                        let bg_tile_select = self.ppuctrl.bg_tile_select();
                        let addr = bg_tile_select | ((self.nt_latch as u16) * 16 + fine_y + 8);
                        self.pattern_latch_hi = self.read_vram(addr);
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
                        for i in 0..self.secondary_oam_memory.len() {
                            self.secondary_oam_memory[i] = 0xFF;
                        }
                        self.sprite_count = 0;
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
                self.ppuctrl = PPUCTRL::from_bits_truncate(value);
                self.t = (self.t & 0xF3FF) | ((value as u16 & 0x3) << 10);
            }

            0x2001 => self.ppumask = PPUMASK::from_bits_truncate(value),
            0x2002 => {} // PPUSTATUS $2002 is read only!
            0x2003 => self.oam_address = value,

            0x2004 => {
                // OAMDATA $2004 dddd dddd
                self.oam_memory[self.oam_address as usize] = value;
                self.oam_address = unchecked_add!(self.oam_address, 1);
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
                self.v += self.ppuctrl.vram_increment();
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
                self.w = false;
                let ret = self.ppustatus.bits();
                self.ppustatus.set(PPUSTATUS::V_BLANK, false);
                ret
            }
            0x2003 => 0, // OAMADDR $2003 is write only!
            0x2004 => self.oam_memory[self.oam_address as usize], // OAMDATA $2004 dddd dddd
            0x2005 => 0, // PPUSCROLL $2005 is write only!
            0x2006 => 0, // PPUADDR $2006 is write only!
            0x2007 => {
                // PPUDATA $2007 dddd dddd
                let ret = self.vram_buffer;
                self.vram_buffer = self.read_vram(self.v & 0x2FFF);
                let pal = self.read_vram(self.v);
                self.v += self.ppuctrl.vram_increment();
                self.v &= 0x3FFF;
                if self.v >= 0x3F00 {
                    pal
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
    const LOOKUP_TABLE: [u8; 256] = [
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

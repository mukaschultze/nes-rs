use crate::cpu::address_mode::AddressMode;
use crate::cpu::CPU6502;
use std::u8;

#[allow(clippy::cast_lossless)]
impl CPU6502<'_> {
    fn illegal_op_code(&mut self, opcode: u8) {
        println!("Illegal opcode {:#2X} @ {:#4X}", opcode, self.pc);
    }

    pub fn process_opcode(&mut self, opcode: u8, ll: u8, hh: u8) {
        match opcode {
            0x00 => self.brk(AddressMode::Implied, ll, hh),
            0x01 => self.ora(AddressMode::IndirectX, ll, hh),
            0x02 => self.illegal_op_code(0x02),
            0x03 => self.illegal_op_code(0x03),
            0x04 => self.illegal_op_code(0x04),
            0x05 => self.ora(AddressMode::Zeropage, ll, hh),
            0x06 => self.asl(AddressMode::Zeropage, ll, hh),
            0x07 => self.illegal_op_code(0x07),
            0x08 => self.php(AddressMode::Implied, ll, hh),
            0x09 => self.ora(AddressMode::Immediate, ll, hh),
            0x0A => self.asl(AddressMode::Accumulator, ll, hh),
            0x0B => self.illegal_op_code(0x0B),
            0x0C => self.illegal_op_code(0x0C),
            0x0D => self.ora(AddressMode::Absolute, ll, hh),
            0x0E => self.asl(AddressMode::Absolute, ll, hh),
            0x0F => self.illegal_op_code(0x0F),

            0x10 => self.bpl(AddressMode::Relative, ll, hh),
            0x11 => self.ora(AddressMode::IndirectY, ll, hh),
            0x12 => self.illegal_op_code(0x12),
            0x13 => self.illegal_op_code(0x13),
            0x14 => self.illegal_op_code(0x14),
            0x15 => self.ora(AddressMode::ZeropageX, ll, hh),
            0x16 => self.asl(AddressMode::ZeropageX, ll, hh),
            0x17 => self.illegal_op_code(0x17),
            0x18 => self.clc(AddressMode::Implied, ll, hh),
            0x19 => self.ora(AddressMode::AbsoluteY, ll, hh),
            0x1A => self.illegal_op_code(0x1A),
            0x1B => self.illegal_op_code(0x1B),
            0x1C => self.illegal_op_code(0x1C),
            0x1D => self.ora(AddressMode::AbsoluteX, ll, hh),
            0x1E => self.asl(AddressMode::AbsoluteX, ll, hh),
            0x1F => self.illegal_op_code(0x1F),

            0x20 => self.jsr(AddressMode::Absolute, ll, hh),
            0x21 => self.and(AddressMode::IndirectX, ll, hh),
            0x22 => self.illegal_op_code(0x22),
            0x23 => self.illegal_op_code(0x23),
            0x24 => self.bit(AddressMode::Zeropage, ll, hh),
            0x25 => self.and(AddressMode::Zeropage, ll, hh),
            0x26 => self.rol(AddressMode::Zeropage, ll, hh),
            0x27 => self.illegal_op_code(0x27),
            0x28 => self.plp(AddressMode::Implied, ll, hh),
            0x29 => self.and(AddressMode::Immediate, ll, hh),
            0x2A => self.rol(AddressMode::Accumulator, ll, hh),
            0x2B => self.illegal_op_code(0x2B),
            0x2C => self.bit(AddressMode::Absolute, ll, hh),
            0x2D => self.and(AddressMode::Absolute, ll, hh),
            0x2E => self.rol(AddressMode::Absolute, ll, hh),
            0x2F => self.illegal_op_code(0x2F),

            0x30 => self.bmi(AddressMode::Relative, ll, hh),
            0x31 => self.and(AddressMode::IndirectY, ll, hh),
            0x32 => self.illegal_op_code(0x32),
            0x33 => self.illegal_op_code(0x33),
            0x34 => self.illegal_op_code(0x34),
            0x35 => self.and(AddressMode::ZeropageX, ll, hh),
            0x36 => self.rol(AddressMode::ZeropageX, ll, hh),
            0x37 => self.illegal_op_code(0x37),
            0x38 => self.sec(AddressMode::Implied, ll, hh),
            0x39 => self.and(AddressMode::AbsoluteY, ll, hh),
            0x3A => self.illegal_op_code(0x3A),
            0x3B => self.illegal_op_code(0x3B),
            0x3C => self.illegal_op_code(0x3C),
            0x3D => self.and(AddressMode::AbsoluteX, ll, hh),
            0x3E => self.rol(AddressMode::AbsoluteX, ll, hh),
            0x3F => self.illegal_op_code(0x3F),

            0x40 => self.rti(AddressMode::Implied, ll, hh),
            0x41 => self.eor(AddressMode::IndirectX, ll, hh),
            0x42 => self.illegal_op_code(0x42),
            0x43 => self.illegal_op_code(0x43),
            0x44 => self.illegal_op_code(0x44),
            0x45 => self.eor(AddressMode::Zeropage, ll, hh),
            0x46 => self.lsr(AddressMode::Zeropage, ll, hh),
            0x47 => self.illegal_op_code(0x47),
            0x48 => self.pha(AddressMode::Implied, ll, hh),
            0x49 => self.eor(AddressMode::Immediate, ll, hh),
            0x4A => self.lsr(AddressMode::Accumulator, ll, hh),
            0x4B => self.illegal_op_code(0x4B),
            0x4C => self.jmp(AddressMode::Absolute, ll, hh),
            0x4D => self.eor(AddressMode::Absolute, ll, hh),
            0x4E => self.lsr(AddressMode::Absolute, ll, hh),
            0x4F => self.illegal_op_code(0x4F),

            0x50 => self.bvc(AddressMode::Relative, ll, hh),
            0x51 => self.eor(AddressMode::IndirectY, ll, hh),
            0x52 => self.illegal_op_code(0x52),
            0x53 => self.illegal_op_code(0x53),
            0x54 => self.illegal_op_code(0x54),
            0x55 => self.eor(AddressMode::ZeropageX, ll, hh),
            0x56 => self.lsr(AddressMode::ZeropageX, ll, hh),
            0x57 => self.illegal_op_code(0x57),
            0x58 => self.cli(AddressMode::Implied, ll, hh),
            0x59 => self.eor(AddressMode::AbsoluteY, ll, hh),
            0x5A => self.illegal_op_code(0x5A),
            0x5B => self.illegal_op_code(0x5B),
            0x5C => self.illegal_op_code(0x5C),
            0x5D => self.eor(AddressMode::AbsoluteX, ll, hh),
            0x5E => self.lsr(AddressMode::AbsoluteX, ll, hh),
            0x5F => self.illegal_op_code(0x5F),

            0x60 => self.rts(AddressMode::Implied, ll, hh),
            0x61 => self.adc(AddressMode::IndirectX, ll, hh),
            0x62 => self.illegal_op_code(0x62),
            0x63 => self.illegal_op_code(0x63),
            0x64 => self.illegal_op_code(0x64),
            0x65 => self.adc(AddressMode::Zeropage, ll, hh),
            0x66 => self.ror(AddressMode::Zeropage, ll, hh),
            0x67 => self.illegal_op_code(0x67),
            0x68 => self.pla(AddressMode::Implied, ll, hh),
            0x69 => self.adc(AddressMode::Immediate, ll, hh),
            0x6A => self.ror(AddressMode::Accumulator, ll, hh),
            0x6B => self.illegal_op_code(0x6B),
            0x6C => self.jmp(AddressMode::Indirect, ll, hh),
            0x6D => self.adc(AddressMode::Absolute, ll, hh),
            0x6E => self.ror(AddressMode::Absolute, ll, hh),
            0x6F => self.illegal_op_code(0x6F),

            0x70 => self.bvs(AddressMode::Relative, ll, hh),
            0x71 => self.adc(AddressMode::IndirectY, ll, hh),
            0x72 => self.illegal_op_code(0x72),
            0x73 => self.illegal_op_code(0x73),
            0x74 => self.illegal_op_code(0x74),
            0x75 => self.adc(AddressMode::ZeropageX, ll, hh),
            0x76 => self.ror(AddressMode::ZeropageX, ll, hh),
            0x77 => self.illegal_op_code(0x77),
            0x78 => self.sei(AddressMode::Implied, ll, hh),
            0x79 => self.adc(AddressMode::AbsoluteY, ll, hh),
            0x7A => self.illegal_op_code(0x7A),
            0x7B => self.illegal_op_code(0x7B),
            0x7C => self.illegal_op_code(0x7C),
            0x7D => self.adc(AddressMode::AbsoluteX, ll, hh),
            0x7E => self.ror(AddressMode::AbsoluteX, ll, hh),
            0x7F => self.illegal_op_code(0x7F),

            0x80 => self.illegal_op_code(0x80),
            0x81 => self.sta(AddressMode::IndirectX, ll, hh),
            0x82 => self.illegal_op_code(0x82),
            0x83 => self.illegal_op_code(0x83),
            0x84 => self.sty(AddressMode::Zeropage, ll, hh),
            0x85 => self.sta(AddressMode::Zeropage, ll, hh),
            0x86 => self.stx(AddressMode::Zeropage, ll, hh),
            0x87 => self.illegal_op_code(0x87),
            0x88 => self.dey(AddressMode::Implied, ll, hh),
            0x89 => self.illegal_op_code(0x89),
            0x8A => self.txa(AddressMode::Implied, ll, hh),
            0x8B => self.illegal_op_code(0x8B),
            0x8C => self.sty(AddressMode::Absolute, ll, hh),
            0x8D => self.sta(AddressMode::Absolute, ll, hh),
            0x8E => self.stx(AddressMode::Absolute, ll, hh),
            0x8F => self.illegal_op_code(0x8F),

            0x90 => self.bcc(AddressMode::Relative, ll, hh),
            0x91 => self.sta(AddressMode::IndirectY, ll, hh),
            0x92 => self.illegal_op_code(0x92),
            0x93 => self.illegal_op_code(0x93),
            0x94 => self.sty(AddressMode::ZeropageX, ll, hh),
            0x95 => self.sta(AddressMode::ZeropageX, ll, hh),
            0x96 => self.stx(AddressMode::ZeropageY, ll, hh),
            0x97 => self.illegal_op_code(0x97),
            0x98 => self.tya(AddressMode::Implied, ll, hh),
            0x99 => self.sta(AddressMode::AbsoluteY, ll, hh),
            0x9A => self.txs(AddressMode::Implied, ll, hh),
            0x9B => self.illegal_op_code(0x9B),
            0x9C => self.illegal_op_code(0x9C),
            0x9D => self.sta(AddressMode::AbsoluteX, ll, hh),
            0x9E => self.illegal_op_code(0x9E),
            0x9F => self.illegal_op_code(0x9F),

            0xA0 => self.ldy(AddressMode::Immediate, ll, hh),
            0xA1 => self.lda(AddressMode::IndirectX, ll, hh),
            0xA2 => self.ldx(AddressMode::Immediate, ll, hh),
            0xA3 => self.illegal_op_code(0xA3),
            0xA4 => self.ldy(AddressMode::Zeropage, ll, hh),
            0xA5 => self.lda(AddressMode::Zeropage, ll, hh),
            0xA6 => self.ldx(AddressMode::Zeropage, ll, hh),
            0xA7 => self.illegal_op_code(0xA7),
            0xA8 => self.tay(AddressMode::Implied, ll, hh),
            0xA9 => self.lda(AddressMode::Immediate, ll, hh),
            0xAA => self.tax(AddressMode::Implied, ll, hh),
            0xAB => self.illegal_op_code(0xAB),
            0xAC => self.ldy(AddressMode::Absolute, ll, hh),
            0xAD => self.lda(AddressMode::Absolute, ll, hh),
            0xAE => self.ldx(AddressMode::Absolute, ll, hh),
            0xAF => self.illegal_op_code(0xAF),

            0xB0 => self.bcs(AddressMode::Relative, ll, hh),
            0xB1 => self.lda(AddressMode::IndirectY, ll, hh),
            0xB2 => self.illegal_op_code(0xB2),
            0xB3 => self.illegal_op_code(0xB3),
            0xB4 => self.ldy(AddressMode::ZeropageX, ll, hh),
            0xB5 => self.lda(AddressMode::ZeropageX, ll, hh),
            0xB6 => self.ldx(AddressMode::ZeropageY, ll, hh),
            0xB7 => self.illegal_op_code(0xB7),
            0xB8 => self.clv(AddressMode::Implied, ll, hh),
            0xB9 => self.lda(AddressMode::AbsoluteY, ll, hh),
            0xBA => self.tsx(AddressMode::Implied, ll, hh),
            0xBB => self.illegal_op_code(0xBB),
            0xBC => self.ldy(AddressMode::AbsoluteX, ll, hh),
            0xBD => self.lda(AddressMode::AbsoluteX, ll, hh),
            0xBE => self.ldx(AddressMode::AbsoluteY, ll, hh),
            0xBF => self.illegal_op_code(0xBF),

            0xC0 => self.cpy(AddressMode::Immediate, ll, hh),
            0xC1 => self.cmp(AddressMode::IndirectX, ll, hh),
            0xC2 => self.illegal_op_code(0xC2),
            0xC3 => self.illegal_op_code(0xC3),
            0xC4 => self.cpy(AddressMode::Zeropage, ll, hh),
            0xC5 => self.cmp(AddressMode::Zeropage, ll, hh),
            0xC6 => self.dec(AddressMode::Zeropage, ll, hh),
            0xC7 => self.illegal_op_code(0xC7),
            0xC8 => self.iny(AddressMode::Implied, ll, hh),
            0xC9 => self.cmp(AddressMode::Immediate, ll, hh),
            0xCA => self.dex(AddressMode::Implied, ll, hh),
            0xCB => self.illegal_op_code(0xCB),
            0xCC => self.cpy(AddressMode::Absolute, ll, hh),
            0xCD => self.cmp(AddressMode::Absolute, ll, hh),
            0xCE => self.dec(AddressMode::Absolute, ll, hh),
            0xCF => self.illegal_op_code(0xCF),

            0xD0 => self.bne(AddressMode::Relative, ll, hh),
            0xD1 => self.cmp(AddressMode::IndirectY, ll, hh),
            0xD2 => self.illegal_op_code(0xD2),
            0xD3 => self.illegal_op_code(0xD3),
            0xD4 => self.illegal_op_code(0xD4),
            0xD5 => self.cmp(AddressMode::ZeropageX, ll, hh),
            0xD6 => self.dec(AddressMode::ZeropageX, ll, hh),
            0xD7 => self.illegal_op_code(0xD7),
            0xD8 => self.cld(AddressMode::Implied, ll, hh),
            0xD9 => self.cmp(AddressMode::AbsoluteY, ll, hh),
            0xDA => self.illegal_op_code(0xDA),
            0xDB => self.illegal_op_code(0xDB),
            0xDC => self.illegal_op_code(0xDC),
            0xDD => self.cmp(AddressMode::AbsoluteX, ll, hh),
            0xDE => self.dec(AddressMode::AbsoluteX, ll, hh),
            0xDF => self.illegal_op_code(0xDF),

            0xE0 => self.cpx(AddressMode::Immediate, ll, hh),
            0xE1 => self.sbc(AddressMode::IndirectX, ll, hh),
            0xE2 => self.illegal_op_code(0xE2),
            0xE3 => self.illegal_op_code(0xE3),
            0xE4 => self.cpx(AddressMode::Zeropage, ll, hh),
            0xE5 => self.sbc(AddressMode::Zeropage, ll, hh),
            0xE6 => self.inc(AddressMode::Zeropage, ll, hh),
            0xE7 => self.illegal_op_code(0xE7),
            0xE8 => self.inx(AddressMode::Implied, ll, hh),
            0xE9 => self.sbc(AddressMode::Immediate, ll, hh),
            0xEA => self.nop(AddressMode::Implied, ll, hh),
            0xEB => self.illegal_op_code(0xEB),
            0xEC => self.cpx(AddressMode::Absolute, ll, hh),
            0xED => self.sbc(AddressMode::Absolute, ll, hh),
            0xEE => self.inc(AddressMode::Absolute, ll, hh),
            0xEF => self.illegal_op_code(0xEF),

            0xF0 => self.beq(AddressMode::Relative, ll, hh),
            0xF1 => self.sbc(AddressMode::IndirectY, ll, hh),
            0xF2 => self.illegal_op_code(0xF2),
            0xF3 => self.illegal_op_code(0xF3),
            0xF4 => self.illegal_op_code(0xF4),
            0xF5 => self.sbc(AddressMode::ZeropageX, ll, hh),
            0xF6 => self.inc(AddressMode::ZeropageX, ll, hh),
            0xF7 => self.illegal_op_code(0xF7),
            0xF8 => self.sed(AddressMode::Implied, ll, hh),
            0xF9 => self.sbc(AddressMode::AbsoluteY, ll, hh),
            0xFA => self.illegal_op_code(0xFA),
            0xFB => self.illegal_op_code(0xFB),
            0xFC => self.illegal_op_code(0xFC),
            0xFD => self.sbc(AddressMode::AbsoluteX, ll, hh),
            0xFE => self.inc(AddressMode::AbsoluteX, ll, hh),
            0xFF => self.illegal_op_code(0xFF),
        }
    }

    pub fn get_memory_value(
        &mut self,
        ll: u8,
        hh: u8,
        mode: AddressMode,
        page_cross_penalty: u8,
    ) -> u16 {
        let address = join_bytes!(hh, ll);

        match mode {
            AddressMode::Accumulator => self.ac as u16,
            AddressMode::Absolute => self.load8(address) as u16,
            AddressMode::AbsoluteX => {
                if page_crossed!(address, unchecked_add!(address, self.xr as u16)) {
                    self.ticks += page_cross_penalty as u64;
                }
                self.load8(unchecked_add!(address, self.xr as u16)) as u16
            }

            AddressMode::AbsoluteY => {
                if page_crossed!(address, unchecked_add!(address, self.yr as u16)) {
                    self.ticks += page_cross_penalty as u64;
                }
                self.load8(unchecked_add!(address, self.yr as u16)) as u16
            }

            AddressMode::Immediate => ll as u16,
            AddressMode::Implied => u8::min_value() as u16,
            AddressMode::Indirect => {
                let indirect = self.load16(address);
                self.load16(indirect)
            }

            AddressMode::IndirectX => {
                let hh = self.load8(unchecked_add!(ll, self.xr, 1) as u16);
                let ll = self.load8(unchecked_add!(ll, self.xr) as u16);
                let address = join_bytes!(hh, ll);
                self.load8(address) as u16
            }

            AddressMode::IndirectY => {
                let hh = self.load8(unchecked_add!(ll, 1) as u16);
                let ll = self.load8(ll as u16);
                let sum = unchecked_add!(join_bytes!(hh, ll), self.yr as u16);

                if (address & 0xFF00) != (sum & 0xFF00) {
                    // Cross-page
                    self.ticks += page_cross_penalty as u64;
                }

                self.load8(sum) as u16
            }

            AddressMode::Relative => self.rel_addr(self.pc, ll as i8),
            AddressMode::Zeropage => self.load8(ll as u16) as u16,
            AddressMode::ZeropageX => self.load8(unchecked_add!(ll, self.xr) as u16) as u16,
            AddressMode::ZeropageY => self.load8(unchecked_add!(ll, self.yr) as u16) as u16,
            _ => unreachable!(),
        }
    }

    pub fn set_memory_value(&mut self, bb: u8, ll: u8, hh: u8, mode: AddressMode) {
        let address = join_bytes!(hh, ll);

        match mode {
            AddressMode::Accumulator => self.ac = bb,
            AddressMode::Absolute => self.store8(address, bb),
            AddressMode::AbsoluteX => self.store8((address + self.xr as u16) as u16, bb),
            AddressMode::Zeropage => self.store8(address, bb),
            AddressMode::ZeropageX => self.store8(low_byte!(address + self.xr as u16) as u16, bb),
            AddressMode::IndirectX => {
                let hh = self.load8(unchecked_add!(ll, self.xr, 1) as u16);
                let ll = self.load8(unchecked_add!(ll, self.xr) as u16);
                let address = join_bytes!(hh, ll);
                self.store8(address, bb);
            }

            AddressMode::IndirectY => {
                let hh = self.load8(unchecked_add!(ll, 1) as u16);
                let ll = self.load8(ll as u16);
                let sum = join_bytes!(hh, ll) + self.yr as u16;

                // if ((address & 0xFF00) != (sum & 0xFF00))
                //     self.ticks += pageCrossPenalty;

                self.store8(sum as u16, bb);
            }

            AddressMode::AbsoluteY => {
                // if ((address & 0xFF00) != ((address + self.yr) & 0xFF00))
                //     self.ticks += pageCrossPenalty;

                self.store8((address + self.yr as u16) as u16, bb);
            }

            AddressMode::Indirect => unimplemented!(),
            AddressMode::ZeropageY => self.store8(unchecked_add!(ll, self.yr) as u16, bb),
            AddressMode::Relative => unreachable!(),
            AddressMode::Immediate => unreachable!(),
            AddressMode::Implied => unreachable!(),
            _ => unreachable!(),
        }
    }

    /// Request a non-maskable interrupt.
    pub fn request_nmi(&mut self) {
        self.nmi_requested = true;
    }

    /// Request an interrupt.
    pub fn request_irq(&mut self) {
        self.irq_requested = true;
    }
}

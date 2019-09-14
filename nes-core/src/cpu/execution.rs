use crate::cpu::address_mode::AddressMode;
use crate::cpu::CPU6502;

macro_rules! branch {
    ($s:ident, $value:expr, $offset:expr) => {
        if $value {
            if ($s.pc & 0xFF00) != ($s.rel_addr($s.pc, $offset) & 0xFF00) {
                $s.ticks += 1;
            }

            $s.pc = $s.rel_addr($s.pc, $offset);
            $s.ticks += 1;
        }
    };
}

#[allow(clippy::cast_lossless)]
#[allow(unused_variables)] // TODO: Remove
impl CPU6502<'_> {
    /// Add with carry.
    pub fn adc(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        let src = self.get_memory_value(ll, hh, mode, 0); // TODO: Fix page cross add
        let temp = self.ac as u16 + src + (if self.if_carry() { 1 } else { 0 });
        let bb = temp as u8;

        self.set_zero(bb);
        self.set_sign(bb);
        // http://forums.nesdev.com/viewtopic.php?t=6331
        self.set_overflow(((self.ac ^ bb) as u16 & (src ^ bb as u16) & 0x80) != 0);
        self.set_carry(temp > 0xFF);
        self.ac = bb;
    }

    /// And (with accumulator).
    pub fn and(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        let src = self.get_memory_value(ll, hh, mode, 0); // TODO: Fix page cross add
        self.ac &= src as u8;
        self.set_sign(self.ac);
        self.set_zero(self.ac);
    }

    /// Arithmetic shift left.
    pub fn asl(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        let mut src = self.get_memory_value(ll, hh, mode, 0); // TODO: Fix page cross add
        self.set_carry((src & 0x80) != 0);
        src <<= 1;
        self.set_sign(src as u8);
        self.set_zero(src as u8);
        self.set_memory_value(src as u8, ll, hh, mode);
    }

    /// Branch on carry clear.
    pub fn bcc(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        branch!(self, !self.if_carry(), ll as i8);
    }

    /// Branch on carry set.
    pub fn bcs(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        branch!(self, self.if_carry(), ll as i8);
    }

    /// Branch on equal (zero set).
    pub fn beq(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        branch!(self, self.if_zero(), ll as i8);
    }

    /// Bit test.
    pub fn bit(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        let src = self.get_memory_value(ll, hh, mode, 0); // TODO: Fix page cross add
        self.set_sign(src as u8);
        self.set_overflow((0x40 & src) != 0);
        self.set_zero(src as u8 & self.ac);
    }

    /// Branch on minus (negative set).
    pub fn bmi(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        branch!(self, self.if_sign(), ll as i8);
    }

    /// Branch on not equal (zero clear).
    pub fn bne(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        branch!(self, !self.if_zero(), ll as i8);
    }
    /// Branch on plus (negative clear).
    pub fn bpl(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        branch!(self, !self.if_sign(), ll as i8);
    }

    /// Interrupt.
    pub fn brk(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        self.pc += 1;
        self.push16(self.pc);
        self.set_break(true);
        let sr = self.sr | 0x30;
        self.push8(sr);
        self.set_interrupt(true);
        self.pc = self.load16(0xFFFE);
    }

    /// Branch on overflow clear.
    pub fn bvc(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        branch!(self, !self.if_overflow(), ll as i8);
    }

    /// Branch on overflow set.
    pub fn bvs(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        branch!(self, self.if_overflow(), ll as i8);
    }

    /// Clear carry.
    pub fn clc(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        self.set_carry(false);
    }

    /// Clear decimal.
    pub fn cld(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        self.set_decimal(false);
    }

    /// Clear interrupt disable.
    pub fn cli(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        self.set_interrupt(false);
    }

    /// Clear overflow.
    pub fn clv(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        self.set_overflow(false);
    }

    /// Compare (with accumulator).
    pub fn cmp(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        let mut src = self.get_memory_value(ll, hh, mode, 0); // TODO: Fix page cross add
        src = unchecked_sub!(self.ac as u16, src);

        self.set_carry(src < 0x100);
        self.set_sign(src as u8);
        self.set_zero(src as u8);
    }

    /// Compare with X.
    pub fn cpx(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        let mut src = self.get_memory_value(ll, hh, mode, 0); // TODO: Fix page cross add
        src = unchecked_sub!(self.xr as u16, src);
        self.set_carry(src < 0x100);
        self.set_sign(src as u8);
        self.set_zero(src as u8);
    }

    /// Compare with Y.
    pub fn cpy(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        let mut src = self.get_memory_value(ll, hh, mode, 0); // TODO: Fix page cross add
        src = unchecked_sub!(self.yr as u16, src);
        self.set_carry(src < 0x100);
        self.set_sign(src as u8);
        self.set_zero(src as u8);
    }

    /// Decrement.
    pub fn dec(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        let mut src = self.get_memory_value(ll, hh, mode, 0); // TODO: Fix page cross add
        src = unchecked_sub!(src, 1);
        self.set_sign(src as u8);
        self.set_zero(src as u8);
        self.set_memory_value(src as u8, ll, hh, mode);
    }

    /// Decrement X.
    pub fn dex(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        self.xr = unchecked_sub!(self.xr, 1);
        self.set_sign(self.xr);
        self.set_zero(self.xr);
    }

    /// Decrement Y.
    pub fn dey(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        self.yr = unchecked_sub!(self.yr, 1);
        self.set_sign(self.yr);
        self.set_zero(self.yr);
    }

    /// Exclusive or (with accumulator).
    pub fn eor(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        let mut src = self.get_memory_value(ll, hh, mode, 0); // TODO: Fix page cross add
        src ^= self.ac as u16;
        self.ac = src as u8;
        self.set_sign(self.ac);
        self.set_zero(self.ac);
    }

    /// Increment.
    pub fn inc(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        let mut src = self.get_memory_value(ll, hh, mode, 0); // TODO: Fix page cross add
        src = unchecked_add!(src, 1);
        self.set_sign(src as u8);
        self.set_zero(src as u8);
        self.set_memory_value(src as u8, ll, hh, mode);
    }

    /// Increment X.
    pub fn inx(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        self.xr = unchecked_add!(self.xr, 1);
        self.set_sign(self.xr);
        self.set_zero(self.xr);
    }

    /// Increment Y.
    pub fn iny(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        self.yr = unchecked_add!(self.yr, 1);
        self.set_sign(self.yr);
        self.set_zero(self.yr);
    }

    /// Jump.
    pub fn jmp(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        match mode {
            AddressMode::Absolute => {
                self.pc = join_bytes!(hh, ll);
            }
            AddressMode::Indirect => {
                let jump_addr_low = self.load8(join_bytes!(hh, ll));
                let jump_addr_high = self.load8(join_bytes!(hh, unchecked_add!(ll, 1)));
                self.pc = join_bytes!(jump_addr_high, jump_addr_low);
            }
            _ => unreachable!(),
        }
    }

    /// Jump subroutine.
    pub fn jsr(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        self.pc -= 1;
        self.push16(self.pc);

        match mode {
            AddressMode::Absolute => {
                self.pc = join_bytes!(hh, ll);
            }
            AddressMode::Indirect => {
                let jump_addr_low = self.load8(join_bytes!(hh, ll));
                let jump_addr_high = self.load8(join_bytes!(hh, 00));;
                self.pc = join_bytes!(jump_addr_high, jump_addr_low);
            }
            _ => unreachable!(),
        }
    }

    /// Load accumulator.
    pub fn lda(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        let src = self.get_memory_value(ll, hh, mode, 0); // TODO: Fix page cross add
        self.ac = src as u8;
        self.set_sign(self.ac);
        self.set_zero(self.ac);
    }

    /// Load X.
    pub fn ldx(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        let src = self.get_memory_value(ll, hh, mode, 0); // TODO: Fix page cross add
        self.xr = src as u8;
        self.set_sign(self.xr);
        self.set_zero(self.xr);
    }

    /// Load Y.
    pub fn ldy(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        let src = self.get_memory_value(ll, hh, mode, 0); // TODO: Fix page cross add
        self.yr = src as u8;
        self.set_sign(self.yr);
        self.set_zero(self.yr);
    }

    /// Logical shift right.
    pub fn lsr(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        let mut src = self.get_memory_value(ll, hh, mode, 0); // TODO: Fix page cross add
        self.set_carry((src & 0x01) != 0);
        src >>= 1;
        self.set_sign(src as u8);
        self.set_zero(src as u8);
        self.set_memory_value(src as u8, ll, hh, mode);
    }

    /// No operation.
    pub fn nop(&mut self, mode: AddressMode, ll: u8, hh: u8) {}

    /// Or with accumulator.
    pub fn ora(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        let src = self.get_memory_value(ll, hh, mode, 0); // TODO: Fix page cross add
        self.ac |= src as u8;
        self.set_sign(self.ac);
        self.set_zero(self.ac);
    }

    /// Push accumulator.
    pub fn pha(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        self.push8(self.ac);
    }

    /// Push processor status (SR).
    pub fn php(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        // https://wiki.nesdev.com/w/index.php/Status_flags#The_B_flag
        let sr = self.sr | 0x30;
        self.push8(sr);
    }

    /// Pull accumulator.
    pub fn pla(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        self.ac = self.pull8();
        self.set_sign(self.ac);
        self.set_zero(self.ac);
    }

    /// Pull processor status (SR).
    pub fn plp(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        let mut value = self.pull8();
        value &= 0b1100_1111;
        value |= self.sr & 0b0011_0000;
        self.set_sr(value);
    }

    /// Rotate left.
    pub fn rol(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        let mut src = self.get_memory_value(ll, hh, mode, 0); // TODO: Fix page cross add
        src <<= 1;
        if self.if_carry() {
            src |= 0x1;
        }
        self.set_carry(src > 0xFF);
        self.set_sign(src as u8);
        self.set_zero(src as u8);
        self.set_memory_value(src as u8, ll, hh, mode);
    }

    /// Rotate right.
    pub fn ror(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        let mut src = self.get_memory_value(ll, hh, mode, 0); // TODO: Fix page cross add
        if self.if_carry() {
            src |= 0x100;
        }
        self.set_carry((src & 0x01) != 0);
        src >>= 1;
        self.set_sign(src as u8);
        self.set_zero(src as u8);
        self.set_memory_value(src as u8, ll, hh, mode);
    }

    /// Return from interrupt.
    pub fn rti(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        let mut value = self.pull8();
        value &= 0b1100_1111;
        value |= self.sr & 0b0011_0000;
        self.set_sr(value);
        self.pc = self.pull16();
    }

    /// Return from subroutine.
    pub fn rts(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        // if (self.sp == 0xFD)
        //     throw new System.Exception();

        self.pc = self.pull16();
        self.pc += 1;
    }

    /// Subtract with carry.
    pub fn sbc(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        let src = self.get_memory_value(ll, hh, mode, 0); // TODO: Fix page cross add
        let temp = unchecked_sub!(self.ac as u16, src, if self.if_carry() { 0 } else { 1 });
        let bb = temp as u8;

        self.set_zero(bb);
        self.set_sign(bb);
        // http://forums.nesdev.com/viewtopic.php?t=6331
        self.set_overflow(((self.ac ^ bb) as u16 & (!src ^ bb as u16) & 0x80) != 0);
        self.set_carry((temp & 0x100) != 0x100);
        self.ac = bb;
    }

    /// Set carry.
    pub fn sec(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        self.set_carry(true);
    }

    /// Set decimal.
    pub fn sed(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        self.set_decimal(true);
    }

    /// Set interrupt disable.
    pub fn sei(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        self.set_interrupt(true);
    }

    /// Store accumulator.
    pub fn sta(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        self.set_memory_value(self.ac, ll, hh, mode);
    }

    /// Store X.
    pub fn stx(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        self.set_memory_value(self.xr, ll, hh, mode);
    }

    /// Store Y.
    pub fn sty(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        self.set_memory_value(self.yr, ll, hh, mode);
    }

    /// Transfer accumulator to X.
    pub fn tax(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        self.xr = self.ac;
        self.set_sign(self.xr);
        self.set_zero(self.xr);
    }

    /// Transfer accumulator to Y.
    pub fn tay(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        self.yr = self.ac;
        self.set_sign(self.yr);
        self.set_zero(self.yr);
    }

    /// Transfer stack pointer to X.
    pub fn tsx(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        self.xr = self.sp;
        self.set_sign(self.xr);
        self.set_zero(self.xr);
    }

    /// Transfer X to accumulator.
    pub fn txa(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        self.ac = self.xr;
        self.set_sign(self.ac);
        self.set_zero(self.ac);
    }

    /// Transfer X to stack pointer.
    pub fn txs(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        self.sp = self.xr;
    }

    /// Transfer Y to accumulator.
    pub fn tya(&mut self, mode: AddressMode, ll: u8, hh: u8) {
        self.ac = self.yr;
        self.set_sign(self.ac);
        self.set_zero(self.ac);
    }
}

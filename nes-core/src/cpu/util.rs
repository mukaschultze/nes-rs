use crate::cpu::CPU6502;

#[derive(Clone, Copy)]
pub enum SRFlag {
    Carry = 1 << 0,
    Zero = 1 << 1,
    InterruptDisable = 1 << 2,
    Decimal = 1 << 3,
    Break = 1 << 4,
    Overflow = 1 << 6,
    Negative = 1 << 7,
}

impl CPU6502 {
    pub fn set_flag(&mut self, flag: SRFlag, set: bool) {
        if set {
            self.sr |= flag as u8;
        } else {
            self.sr &= !(flag as u8);
        }
    }

    pub fn get_flag(&mut self, flag: SRFlag) -> bool {
        (self.sr & flag as u8) != 0
    }

    /// Set the Program Status Register to the value given.
    pub fn set_sr(&mut self, bb: u8) {
        self.sr = bb;
    }

    /// Get the value of the Program Status Register.
    pub fn get_sr(&mut self) -> u8 {
        self.sr
    }

    /// Get a byte from the memory address.
    pub fn load8(&mut self, address: u16) -> u8 {
        self.bus.borrow_mut().read(address)
    }

    /// Store a byte in the memory address.
    pub fn store8(&mut self, address: u16, bb: u8) {
        self.bus.borrow_mut().write(address, bb);
    }

    /// Pull a byte the stack.
    #[allow(clippy::cast_lossless)]
    pub fn pull8(&mut self) -> u8 {
        self.sp = unchecked_add!(self.sp, 1);
        self.load8(0x100 + self.sp as u16)
    }

    /// Push a onto the stack.
    #[allow(clippy::cast_lossless)]
    pub fn push8(&mut self, bb: u8) {
        self.store8(0x100 + self.sp as u16, bb);
        self.sp = unchecked_sub!(self.sp, 1);
    }

    /// Pull a short off the stack.
    pub fn pull16(&mut self) -> u16 {
        let ll = self.pull8();
        let hh = self.pull8();
        join_bytes!(hh, ll)
    }

    /// Push a short onto the stack.
    pub fn push16(&mut self, value: u16) {
        self.push8(high_byte!(value));
        self.push8(low_byte!(value));
    }

    /// Get a short from the memory address.
    pub fn load16(&mut self, address: u16) -> u16 {
        let ll = self.load8(address);
        let hh = self.load8(address + 1);
        return join_bytes!(hh, ll);
    }
}

use crate::cpu::CPU6502;

impl CPU6502<'_> {
    fn set_flag(&mut self, bit: u8, set: bool) {
        if set {
            self.sr |= 1 << bit;
        } else {
            self.sr &= !(1 << bit);
        }
    }

    fn get_flag(&mut self, bit: u8) -> bool {
        (self.sr & 1 << bit) != 0
    }

    /// Set/Reset the sign flag depending on bit 7.
    pub fn set_sign(&mut self, bb: u8) {
        self.set_flag(7, (bb & 0x80) != 0);
    }

    /// Set/Reset the zero flag depending on whether the result is zero or not.
    pub fn set_zero(&mut self, bb: u8) {
        self.set_flag(1, bb == 0);
    }

    /// If the condition is true then the carry flag is set, else it is reset.
    pub fn set_carry(&mut self, condition: bool) {
        self.set_flag(0, condition);
    }

    /// If the condition is true then the overflow flag is set, else it is reset.
    pub fn set_overflow(&mut self, condition: bool) {
        self.set_flag(6, condition);
    }

    /// If the condition is true then the interrupt flag is set, else it is reset.
    pub fn set_interrupt(&mut self, condition: bool) {
        self.set_flag(2, condition);
    }

    /// If the condition is true then the break flag is set, else it is reset.
    pub fn set_break(&mut self, condition: bool) {
        self.set_flag(4, condition);
    }

    /// If the condition is true then the decimal flag is set, else it is reset.
    pub fn set_decimal(&mut self, condition: bool) {
        self.set_flag(3, condition);
    }

    /// Returns the relative address obtained by adding the displacement to the PC.
    #[allow(clippy::cast_lossless)]
    pub fn rel_addr(&mut self, pc: u16, displacement: i8) -> u16 {
        (pc as i32 + displacement as i32) as u16
    }

    /// Set the Program Status Register to the value given.
    pub fn set_sr(&mut self, bb: u8) {
        self.sr = bb;
    }

    /// Get the value of the Program Status Register.
    pub fn get_sr(&mut self) -> u8 {
        self.sr
    }

    /// Pull a byte the stack.
    #[allow(clippy::cast_lossless)]
    pub fn pull8(&mut self) -> u8 {
        self.sp += 1;
        self.load8(0x100 + self.sp as u16)
    }

    /// Pull a short off the stack.
    pub fn pull16(&mut self) -> u16 {
        let ll = self.pull8();
        let hh = self.pull8();
        join_bytes!(hh, ll)
    }

    /// Push a onto the stack.
    #[allow(clippy::cast_lossless)]
    pub fn push8(&mut self, bb: u8) {
        self.store8(0x100 + self.sp as u16, bb);
        self.sp -= 1;
    }

    /// Push a short onto the stack.
    pub fn push16(&mut self, value: u16) {
        self.push8(high_byte!(value));
        self.push8(low_byte!(value));
    }

    /// Get a byte from the memory address.
    pub fn load8(&mut self, address: u16) -> u8 {
        self.bus.read(address)
    }

    /// Get a short from the memory address.
    pub fn load16(&mut self, address: u16) -> u16 {
        let ll = self.load8(address);
        let hh = self.load8(address + 1);
        return join_bytes!(hh, ll);
    }

    /// Store a in: u8 a memory address.
    pub fn store8(&mut self, address: u16, bb: u8) {
        self.bus.write(address, bb);
    }

    /// Returns true if the sign flag is set, otherwise returns false.
    pub fn if_sign(&mut self) -> bool {
        self.get_flag(7)
    }

    /// Returns true if the zero flag is set, otherwise returns false.
    pub fn if_zero(&mut self) -> bool {
        self.get_flag(1)
    }

    /// Returns true if the carry flag is set, otherwise returns false.
    pub fn if_carry(&mut self) -> bool {
        self.get_flag(0)
    }

    /// Returns true if the overflow flag is set, otherwise returns false.
    pub fn if_overflow(&mut self) -> bool {
        self.get_flag(6)
    }

    /// Returns true if the interrupt flag is set, otherwise returns false.
    pub fn if_interrupt(&mut self) -> bool {
        self.get_flag(2)
    }

    /// Returns true if the break flag is set, otherwise returns false.
    pub fn if_break(&mut self) -> bool {
        self.get_flag(4)
    }

    /// Returns true if the decimal flag is set, otherwise returns false.
    pub fn if_decimal(&mut self) -> bool {
        self.get_flag(3)
    }
}

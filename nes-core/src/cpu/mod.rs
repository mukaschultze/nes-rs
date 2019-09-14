pub mod address_mode;
mod execution;
mod instruction;
pub mod instructions_info;
mod util;

use crate::bus::DataBus;
use crate::cpu::instructions_info::Instruction;
use std::u8;

pub struct CPU6502<'a> {
    /// Program counter.
    pub pc: u16,
    /// Accumulator register.
    pub ac: u8,
    /// X register.
    pub xr: u8,
    /// Y register.
    pub yr: u8,
    /// Status register (NV-BDIZC).
    pub sr: u8, // 0x34,
    /// Stack pointer.
    pub sp: u8,
    /// The number of clock cycles since the start of the processor.
    pub ticks: u64,

    irq_requested: bool,
    nmi_requested: bool,

    pub bus: &'a mut DataBus<'a>,
}

impl<'a> CPU6502<'a> {
    pub fn new(bus: &'a mut DataBus<'a>) -> Self {
        CPU6502 {
            pc: 0xC000,
            ac: 0x00,
            xr: 0x00,
            yr: 0x00,
            sr: 0x24,
            sp: 0xFD,
            ticks: 4,
            bus,
            irq_requested: false,
            nmi_requested: false,
        }
    }

    #[allow(clippy::cast_lossless)]
    pub fn process_next_opcode(&mut self) -> (u8, u8, u8) {
        if self.irq_requested {
            self.irq();
            return (0, 0, 0);
        }
        if self.nmi_requested {
            self.nmi();
            return (0, 0, 0);
        }

        let opcode = self.load8(self.pc);
        let inst = Instruction::get_instruction(opcode);

        let ll = if inst.size >= 2 {
            self.load8(self.pc + 1)
        } else {
            u8::min_value()
        };

        let hh = if inst.size >= 3 {
            self.load8(self.pc + 2)
        } else {
            u8::min_value()
        };

        self.ticks += inst.ticks as u64;
        self.pc += inst.size as u16;

        self.process_opcode(opcode, ll, hh);

        (opcode, ll, hh)
    }

    fn nmi(&mut self) {
        self.nmi_requested = false;
        self.push16(self.pc);
        let mut sr = self.sr;
        sr |= 0x10;
        sr &= !0x20;
        self.push8(sr);
        self.pc = self.load16(0xFFFA);
        self.set_interrupt(true);
    }

    fn irq(&mut self) {
        self.irq_requested = false;
        self.push16(self.pc);
        let mut sr = self.sr;
        sr |= 0x10;
        sr &= !0x20;
        self.push8(sr);
        self.pc = self.load16(0xFFFE);
        self.set_interrupt(true);
    }
}

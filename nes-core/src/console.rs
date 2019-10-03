extern crate regex;

use crate::bus::DataBus;
use crate::cpu::address_mode::AddressMode;
use crate::cpu::instructions_info::Instruction;
use crate::cpu::CPU6502;
use crate::ppu::Ppu;
use crate::rom::rom_file::RomFile;
use regex::Regex;
use std::cell::RefCell;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::rc::Rc;
use std::u16;
use std::u8;

pub struct NesConsole {
    pub cpu: Rc<RefCell<CPU6502>>,
    pub bus: Rc<RefCell<DataBus>>,
    pub ppu: Rc<RefCell<Ppu>>,
}

impl NesConsole {
    pub fn new(rom: Rc<RefCell<RomFile>>) -> NesConsole {
        let bus = Rc::new(RefCell::new(DataBus::new(rom.clone())));
        let cpu = Rc::new(RefCell::new(CPU6502::new(bus.clone())));
        let ppu = Rc::new(RefCell::new(Ppu::new(
            cpu.clone(),
            bus.clone(),
            rom.clone(),
        )));

        bus.clone().borrow_mut().ppu = Some(ppu.clone());

        NesConsole { bus, cpu, ppu }
    }

    pub fn tick(&mut self) {
        self.cpu.borrow_mut().process_next_opcode();
        self.ppu.borrow_mut().tick();
        self.ppu.borrow_mut().tick();
        self.ppu.borrow_mut().tick();
    }
}

#[allow(dead_code)]
fn check_instructions(log_path: &Path) {
    let log_file = File::open(log_path).unwrap();
    let log_reader = BufReader::new(log_file);

    let rom_path = Path::new("./test/nestest.nes");
    let rom = Rc::new(RefCell::new(RomFile::new(rom_path)));
    let nes = NesConsole::new(rom);

    // let pc_high = nes.cpu.bus.read(0xFFFD);
    // let pc_low = nes.cpu.bus.read(0xFFFC);

    // nes.cpu.pc = join_bytes!(pc_high, pc_low);

    let mut cpu = nes.cpu.borrow_mut();
    let regex = Regex::new(r"([0-9A-F]{4})  ([0-9A-F]{2}) ([0-9A-F]{2}|\s{2}) ([0-9A-F]{2}|\s{2}) [ \*].{32}A:([0-9A-F]{2}) X:([0-9A-F]{2}) Y:([0-9A-F]{2}) P:([0-9A-F]{2}) SP:([0-9A-F]{2}) PPU:\s*(\d*),\s*(\d*) CYC:(\d+)").unwrap();

    for line in log_reader.lines().map(|l| l.unwrap()) {
        let cap = regex.captures(&line).expect(&line);
        let addr = u16::from_str_radix(&cap[1], 16).unwrap();
        let opcode = u8::from_str_radix(&cap[2], 16).unwrap();
        let byte_lo = if &cap[3] != "  " {
            u8::from_str_radix(&cap[3], 16).unwrap()
        } else {
            u8::min_value()
        };
        let byte_hi = if &cap[4] != "  " {
            u8::from_str_radix(&cap[4], 16).unwrap()
        } else {
            u8::min_value()
        };
        let reg_a = u8::from_str_radix(&cap[5], 16).unwrap();
        let reg_x = u8::from_str_radix(&cap[6], 16).unwrap();
        let reg_y = u8::from_str_radix(&cap[7], 16).unwrap();
        let reg_p = u8::from_str_radix(&cap[8], 16).unwrap();
        let reg_sp = u8::from_str_radix(&cap[9], 16).unwrap();
        // let reg_ppu_x = u16::from_str_radix(&cap[10], 10).unwrap();
        // let reg_ppu_y = u16::from_str_radix(&cap[11], 10).unwrap();
        // let reg_cyc = u16::from_str_radix(&cap[12], 10).unwrap();

        assert_eq!(addr, cpu.pc, "instruction address\n{}\n", line);
        assert_eq!(reg_a, cpu.ac, "ac register\n{}\n", line);
        assert_eq!(reg_x, cpu.xr, "xr register\n{}\n", line);
        assert_eq!(reg_y, cpu.yr, "yr register\n{}\n", line);
        assert_eq!(reg_p, cpu.sr, "sr register\n{}\n", line);
        assert_eq!(reg_sp, cpu.sp, "sp register\n{}\n", line);

        let las_instr = cpu.process_next_opcode();

        assert_eq!(opcode, las_instr.0, "opcode\n{}\n", line);
        assert_eq!(byte_lo, las_instr.1, "low byte\n{}\n", line);
        assert_eq!(byte_hi, las_instr.2, "high byte\n{}\n", line);
        // assert_eq!(reg_ppu_x,  , "ppu x\n{}\n", line);
        // assert_eq!(reg_ppu_y,  , "ppu y\n{}\n", line);
        // assert_eq!(reg_cyc, cpu.ticks , "clock cycles\n{}\n", line);
    }
}

#[test]
fn official_instructions() {
    check_instructions(&Path::new("./test/nestest.log"));
}

#[test]
#[ignore]
fn unofficial_instructions() {
    check_instructions(&Path::new("./test/nestest.full.log"));
}

pub fn format_instruction(opcode: u8, ll: u8, hh: u8) -> String {
    let inst = Instruction::get_instruction(opcode);

    match inst.addressing_mode {
        AddressMode::Implied => format!("{} ", inst.name),
        AddressMode::Accumulator => format!("{} A", inst.name),
        AddressMode::Absolute => format!("{} ${:02X}{:02X}", inst.name, hh, ll),
        AddressMode::AbsoluteX => format!("{} ${:02X}{:02X},X", inst.name, hh, ll),
        AddressMode::AbsoluteY => format!("{} ${:02X}{:02X},Y", inst.name, hh, ll),
        AddressMode::Immediate => format!("{} #${:02X}", inst.name, ll),
        AddressMode::Indirect => format!("{} (${:02X}{:02X})", inst.name, hh, ll),
        AddressMode::IndirectX => format!("{} (${:02X},X)", inst.name, ll),
        AddressMode::IndirectY => format!("{} (${:02X},Y)", inst.name, ll),
        AddressMode::Relative => format!("{} ${:02X}", inst.name, ll),
        AddressMode::Zeropage => format!("{} ${:02X}", inst.name, ll),
        AddressMode::ZeropageX => format!("{} ${:02X},X", inst.name, ll),
        AddressMode::ZeropageY => format!("{} ${:02X},Y", inst.name, ll),
        _ => format!("{} ", inst.name),
    }
}

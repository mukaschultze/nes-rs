extern crate regex;

use crate::bus::DataBus;
use crate::cpu::address_mode::AddressMode;
use crate::cpu::instructions_info::Instruction;
use crate::cpu::CPU6502;
use crate::rom::rom_file::RomFile;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::u16;
use std::u8;

pub struct NesConsole<'a> {
    cpu: &'a mut CPU6502<'a>,
    // bus: &'a mut DataBus<'a>,
}

#[test]
fn nestest() {
    should_work();
}

pub fn should_work() {
    let log_path = Path::new("./test/nestest.log");
    let log_file = File::open(&log_path).unwrap();
    let log_reader = BufReader::new(log_file);

    let rom_path = Path::new("./test/nestest.nes");
    let mut rom = RomFile::new(rom_path);

    let mut mapper = match rom.get_mapper() {
        Some(mapper) => mapper,
        None => panic!("No mapper in ROM"),
    };
    let mut bus = DataBus::new(&mut mapper);
    let mut cpu = CPU6502::new(&mut bus);
    let nes = NesConsole { cpu: &mut cpu };

    // let pc_high = nes.cpu.bus.read(0xFFFD);
    // let pc_low = nes.cpu.bus.read(0xFFFC);

    // nes.cpu.pc = join_bytes!(pc_high, pc_low);

    let regex = Regex::new(r"([0-9A-F]{4})  ([0-9A-F]{2}) ([0-9A-F]{2}|\s{2}) ([0-9A-F]{2}|\s{2})  .{32}A:([0-9A-F]{2}) X:([0-9A-F]{2}) Y:([0-9A-F]{2}) P:([0-9A-F]{2}) SP:([0-9A-F]{2}) PPU:\s*(\d*),\s*(\d*) CYC:(\d+)").unwrap();
    for line in log_reader.lines().map(|l| l.unwrap()) {
        let cap = regex.captures(&line).unwrap();
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

        assert_eq!(addr, nes.cpu.pc, "instruction address\n{}\n", line);
        assert_eq!(reg_a, nes.cpu.ac, "ac register\n{}\n", line);
        assert_eq!(reg_x, nes.cpu.xr, "xr register\n{}\n", line);
        assert_eq!(reg_y, nes.cpu.yr, "yr register\n{}\n", line);
        assert_eq!(reg_p, nes.cpu.sr, "sr register\n{}\n", line);
        assert_eq!(reg_sp, nes.cpu.sp, "sp register\n{}\n", line);

        let las_instr = nes.cpu.process_next_opcode();

        assert_eq!(opcode, las_instr.0, "opcode\n{}\n", line);
        assert_eq!(byte_lo, las_instr.1, "low byte\n{}\n", line);
        assert_eq!(byte_hi, las_instr.2, "high byte\n{}\n", line);
        // assert_eq!(reg_ppu_x,  , "ppu x\n{}\n", line);
        // assert_eq!(reg_ppu_y,  , "ppu y\n{}\n", line);
        // assert_eq!(reg_cyc, nes.cpu.ticks , "clock cycles\n{}\n", line);
    }
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

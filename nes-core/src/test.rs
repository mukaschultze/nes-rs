extern crate regex;
// extern crate test;

use crate::console::NesConsole;
use crate::palette;
use crate::rom::rom_file::RomFile;
use regex::Regex;
use std::cell::RefCell;
use std::fs::File;
use std::path::Path;
use std::rc::Rc;
use std::u16;
// use test::Bencher;

const LOG_FILE: &str = include_str!("../test/nestest.log"); // "../test/nestest.full.log"
const LOG_REGEX_PATTERN : &str = r"([0-9A-F]{4})  ([0-9A-F]{2}) ([0-9A-F]{2}|\s{2}) ([0-9A-F]{2}|\s{2}) [ \*].{32}A:([0-9A-F]{2}) X:([0-9A-F]{2}) Y:([0-9A-F]{2}) P:([0-9A-F]{2}) SP:([0-9A-F]{2}) PPU:\s*(\d*),\s*(\d*) CYC:(\d+)";

const ROM_NESTEST: &[u8] = include_bytes!("../test/nestest.nes");
const ROM_DONKEY_KONG: &[u8] = include_bytes!("../test/Donkey Kong (World) (Rev A).nes");

fn nes_with_rom(rom_bytes: &[u8], start_addr: u16) -> NesConsole {
    let rom = Rc::new(RefCell::new(RomFile::from_bytes(rom_bytes)));
    let nes = NesConsole::new(rom);

    {
        let mut cpu = nes.cpu.borrow_mut();
        cpu.pc = start_addr;
    }
    nes
}

#[test]
fn cpu_instructions() {
    let regex = Regex::new(LOG_REGEX_PATTERN).unwrap();
    let nes = nes_with_rom(ROM_NESTEST, 0xC000);
    let mut cpu = nes.cpu.borrow_mut();

    for line in LOG_FILE.lines() {
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

        assert_eq!(addr, cpu.pc, "instruction address\n{}\n", line);
        assert_eq!(reg_a, cpu.ac, "ac register\n{}\n", line);
        assert_eq!(reg_x, cpu.xr, "xr register\n{}\n", line);
        assert_eq!(reg_y, cpu.yr, "yr register\n{}\n", line);
        assert_eq!(reg_p, cpu.sr, "sr register\n{}\n", line);
        assert_eq!(reg_sp, cpu.sp, "sp register\n{}\n", line);

        let (op, ll, hh) = cpu.process_next_opcode();

        assert_eq!(opcode, op, "opcode\n{}\n", line);
        assert_eq!(byte_lo, ll, "low byte\n{}\n", line);
        assert_eq!(byte_hi, hh, "high byte\n{}\n", line);
    }
}

#[test]
#[ignore]
fn cpu_timings() {
    let regex = Regex::new(LOG_REGEX_PATTERN).unwrap();
    let mut nes = nes_with_rom(ROM_NESTEST, 0xC000);

    for line in LOG_FILE.lines() {
        let cap = regex.captures(&line).expect(&line);
        let cyc = u64::from_str_radix(&cap[12], 10).unwrap();

        nes.tick();

        let cycles = {
            let cpu = nes.cpu.borrow();
            cpu.ticks
        };

        assert_eq!(cyc, cycles, "clock cycles\n{}\n", line);
    }
}

#[test]
#[ignore]
fn ppu_timings() {
    let regex = Regex::new(LOG_REGEX_PATTERN).unwrap();
    let mut nes = nes_with_rom(ROM_NESTEST, 0xC000);

    for line in LOG_FILE.lines() {
        let cap = regex.captures(&line).expect(&line);
        let ppu_x = u16::from_str_radix(&cap[10], 10).unwrap();
        let ppu_y = u16::from_str_radix(&cap[11], 10).unwrap();

        let (dot, scanline) = {
            let ppu = nes.ppu.borrow();
            (ppu.dot, ppu.scanline)
        };

        nes.tick();

        assert_eq!(ppu_x, dot, "ppu x\n{}\n", line);
        assert_eq!(ppu_y, scanline, "ppu y\n{}\n", line);
    }
}

#[test]
fn screenshot() {
    const ROM_DONKEY_KONG: &[u8] = include_bytes!("../test/Donkey Kong (World) (Rev A).nes");
    let rom = Rc::new(RefCell::new(RomFile::from_bytes(ROM_DONKEY_KONG)));
    let mut nes = NesConsole::new(rom);

    for _ in 0..15 {
        nes.render_full_frame();
    }

    nes.screenshot(&Path::new("screenshot.png"));

    let output = nes.ppu.borrow().output;
    let decoder = png::Decoder::new(File::open("screenshot.png").unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut buf = vec![0; info.buffer_size()];

    reader.next_frame(&mut buf).unwrap();

    for i in 0..output.len() {
        let output = palette::get_rgb_color_split(output[i]);
        let screenshot = (buf[i * 3 + 0], buf[i * 3 + 1], buf[i * 3 + 2]);

        assert_eq!(output, screenshot, "Screenshot and output don't match");
    }
}

#[test]
fn gif() {
    // const ROM_DONKEY_KONG: &[u8] = include_bytes!("../test/Donkey Kong (World) (Rev A).nes");
    const ROM_DONKEY_KONG: &[u8] = include_bytes!("../../roms/Testing/NEStress.NES");
    let rom = Rc::new(RefCell::new(RomFile::from_bytes(ROM_DONKEY_KONG)));
    let mut nes = NesConsole::new(rom);
    let mut encoder = nes.get_gif_encoder(Path::new("test.gif"));

    for _ in 0..250 {
        nes.render_full_frame();
        nes.render_full_frame();
        nes.frame_to_gif(&mut encoder);
    }
}

// #[bench]
// fn render_frame(b: &mut Bencher) {
//     let rom = Rc::new(RefCell::new(RomFile::from_bytes(ROM_DONKEY_KONG)));
//     let mut nes = NesConsole::new(rom);

//     b.iter(|| nes.render_full_frame());
// }

// #[bench]
// fn nes_speed(b: &mut Bencher) {
//     let rom = Rc::new(RefCell::new(RomFile::from_bytes(ROM_DONKEY_KONG)));
//     let mut nes = NesConsole::new(rom);

//     b.iter(|| nes.tick());
// }

// #[bench]
// fn cpu_speed(b: &mut Bencher) {
//     let rom = Rc::new(RefCell::new(RomFile::from_bytes(ROM_DONKEY_KONG)));
//     let nes = NesConsole::new(rom);
//     let mut cpu = nes.cpu.borrow_mut();

//     b.iter(|| cpu.process_next_opcode());
// }

// #[bench]
// fn ppu_speed(b: &mut Bencher) {
//     let rom = Rc::new(RefCell::new(RomFile::from_bytes(ROM_DONKEY_KONG)));
//     let nes = NesConsole::new(rom);
//     let mut ppu = nes.ppu.borrow_mut();

//     b.iter(|| ppu.tick());
// }

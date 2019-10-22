#![feature(test)]
#![cfg(test)]

extern crate nes_core;
extern crate test;

use nes_core::console::NesConsole;
use nes_core::rom::rom_file::RomFile;

use test::Bencher;

#[bench]
fn render_frame(b: &mut Bencher) {
    const ROM_SRC: &[u8] = include_bytes!("../roms/Testing/NEStress.NES");
    let mut rom = RomFile::from_bytes(ROM_SRC);
    let mut nes = NesConsole::new();

    nes.bus.borrow_mut().connect_cartridge(&mut rom);
    nes.reset();

    b.iter(|| nes.render_full_frame());
}

#[bench]
fn nes_speed(b: &mut Bencher) {
    const ROM_SRC: &[u8] = include_bytes!("../roms/Testing/NEStress.NES");
    let mut rom = RomFile::from_bytes(ROM_SRC);
    let mut nes = NesConsole::new();

    nes.bus.borrow_mut().connect_cartridge(&mut rom);
    nes.reset();

    b.iter(|| nes.tick());
}

extern crate sha1;

use nes_core::console::NesConsole;
use nes_core::rom::rom_file::RomFile;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

#[cfg(test)]
fn nes_with_rom(rom_bytes: &[u8]) -> NesConsole {
    let rom = Rc::new(RefCell::new(RomFile::from_bytes(rom_bytes)));

    NesConsole::new(rom)
}

#[cfg(test)]
fn run_for_frames_and_screenshot(rom_bytes: &[u8], frames: u32, screenshot_name: &str) {
    let mut nes = nes_with_rom(rom_bytes);

    for _ in 0..frames {
        nes.render_full_frame();
    }

    nes.screenshot(Path::new(screenshot_name));
}

#[cfg(test)]
fn run_for_frames_and_return_hash(rom_bytes: &[u8], frames: u32) -> String {
    let mut nes = nes_with_rom(rom_bytes);

    for _ in 0..frames {
        nes.render_full_frame();
    }

    let ppu = nes.ppu.borrow();
    let output = &ppu.output;
    let mut m = sha1::Sha1::new();

    m.update(output);
    m.digest().to_string()
}

#[test]
fn palette_ram() {
    let hash = run_for_frames_and_return_hash(
        include_bytes!("roms/blargg_ppu_tests/palette_ram.nes"),
        120,
    );

    assert_eq!(hash, "6f6b9c5048bace6cbcf8402fab94328992e83ebc");
}

#[test]
fn power_up_palette() {
    let hash = run_for_frames_and_return_hash(
        include_bytes!("roms/blargg_ppu_tests/power_up_palette.nes"),
        120,
    );

    assert_eq!(hash, "6f6b9c5048bace6cbcf8402fab94328992e83ebc");
}

#[test]
fn sprite_ram() {
    let hash =
        run_for_frames_and_return_hash(include_bytes!("roms/blargg_ppu_tests/sprite_ram.nes"), 120);

    assert_eq!(hash, "6f6b9c5048bace6cbcf8402fab94328992e83ebc");
}

#[test]
fn vbl_clear_time() {
    let hash = run_for_frames_and_return_hash(
        include_bytes!("roms/blargg_ppu_tests/vbl_clear_time.nes"),
        120,
    );

    assert_eq!(hash, "6f6b9c5048bace6cbcf8402fab94328992e83ebc");
}

#[test]
fn vram_access() {
    let hash = run_for_frames_and_return_hash(
        include_bytes!("roms/blargg_ppu_tests/vram_access.nes"),
        120,
    );

    assert_eq!(hash, "6f6b9c5048bace6cbcf8402fab94328992e83ebc");
}

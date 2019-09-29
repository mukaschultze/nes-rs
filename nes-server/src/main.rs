extern crate nes_core;
extern crate sdl2;

use std::cell::RefCell;
use std::rc::Rc;
use std::path::Path;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use nes_core::rom::rom_file::RomFile;
use nes_core::console::NesConsole;

fn main() {
    // let rom_path = Path::new("../nes-core/test/nestest.nes");
    let rom_path = Path::new("D:/Repos/nes/Assets/StreamingAssets/Roms/Mario Bros (E).nes");
    let rom = Rc::new(RefCell::new(RomFile::new(rom_path)));
    let mut nes = NesConsole::new(rom);

    // http://nercury.github.io/rust/opengl/tutorial/2018/02/08/opengl-in-rust-from-scratch-01-window.html
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();

    let width = 256;
    let height = 240;

    let window = video_subsystem
        .window("NES", width, height)
        .build()
        .unwrap( );

    let mut canvas : Canvas<Window> = window.into_canvas()
        .present_vsync() // this means the screen cannot render faster than your display rate (usually 60Hz or 144Hz)
        .build()
        .unwrap();

    loop {
        for _ in 0..1790000 / 60 {
            nes.tick();
        }

        for evt in event_pump.poll_iter() {
            
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear(); 

        let output = &nes.ppu.borrow().output;

        for y in 0..height {
            for x in 0..width {
                let pixel = output[(width * y + x) as usize];
                let color = Color::RGB(pixel, pixel, pixel);
                let point = Point::new(x as i32, y as i32);

                canvas.set_draw_color(color);
                canvas.draw_point(point);
            }
        }

        // However the canvas has not been updated to the window yet,
        // everything has been processed to an internal buffer,
        // but if we want our buffer to be displayed on the window,
        // we need to call `present`. We need to call this everytime
        // we want to render a new frame on the window.
        canvas.present();
        // present does not "clear" the buffer, that means that
        // you have to clear it yourself before rendering again,
        // otherwise leftovers of what you've renderer before might
        // show up on the window !
        //
        // A good rule of thumb is to `clear()`, draw every texture
        // needed, and then `present()`; repeat this every new frame.

    }
}

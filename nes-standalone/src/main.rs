#![windows_subsystem = "windows"]

extern crate nes_core;
extern crate sdl2;
extern crate stopwatch;

use stopwatch::Stopwatch;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::cell::RefCell;
use std::env;
use std::path::Path;
use std::rc::Rc;

use nes_core::console::NesConsole;
use nes_core::controller::Controller;
use nes_core::controller::ControllerDataLine;
use nes_core::palette;
use nes_core::rom::rom_file::RomFile;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 240;

const KEYMAPS: &[(Keycode, ControllerDataLine)] = &[
    (Keycode::A, ControllerDataLine::A),
    (Keycode::S, ControllerDataLine::B),
    (Keycode::Return, ControllerDataLine::SELECT),
    (Keycode::Space, ControllerDataLine::START),
    (Keycode::Up, ControllerDataLine::UP),
    (Keycode::Down, ControllerDataLine::DOWN),
    (Keycode::Left, ControllerDataLine::LEFT),
    (Keycode::Right, ControllerDataLine::RIGHT),
];

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("No ROM file specified");
    }

    println!("Loading ROM from {}", args[1]);
    let rom_path = Path::new(&args[1]);
    let rom = Rc::new(RefCell::new(RomFile::from_file(rom_path)));
    let mut nes = NesConsole::new(rom);

    {
        let mut bus = nes.bus.borrow_mut();
        let controller = Controller::new();
        bus.controller0 = Some(controller);
    }

    // http://nercury.github.io/rust/opengl/tutorial/2018/02/08/opengl-in-rust-from-scratch-01-window.html
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();

    let window = video_subsystem
        .window("NES", WIDTH, HEIGHT)
        .resizable()
        .opengl()
        .build()
        .unwrap();

    let mut canvas: Canvas<Window> = window
        .into_canvas()
        .present_vsync() // this means the screen cannot render faster than your display rate (usually 60Hz or 144Hz)
        .build()
        .unwrap();

    canvas.set_logical_size(WIDTH, HEIGHT).unwrap();

    'main: loop {
        nes.render_full_frame();

        let mut bus = nes.bus.borrow_mut();
        let controller = bus.controller0.as_mut().expect("No controller 0 connected");
        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit { .. } => break 'main,

                Event::KeyDown { keycode, .. } => {
                    for (src, dst) in KEYMAPS {
                        if keycode == Some(*src) {
                            controller.data.insert(*dst);
                        }
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    for (src, dst) in KEYMAPS {
                        if keycode == Some(*src) {
                            controller.data.remove(*dst);
                        }
                    }
                }

                // evt => println!("Event received: {:?}", evt),
                _ => {}
            }
        }

        let mut sw = Stopwatch::start_new();
        let clear_color_idx = nes.ppu.borrow().palette_vram[0];
        let (r, g, b) = palette::get_rgb_color_split(clear_color_idx);
        canvas.set_draw_color(Color::RGB(r, g, b));
        canvas.clear();

        let output = &nes.ppu.borrow().output;

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let color_idx = output[(WIDTH * y + x) as usize];

                if color_idx != clear_color_idx {
                    let (r, g, b) = palette::get_rgb_color_split(color_idx);
                    let color = Color::RGB(r, g, b);
                    let point = Point::new(x as i32, y as i32);

                    canvas.set_draw_color(color);
                    canvas.draw_point(point).unwrap();
                }
            }
        }

        canvas.present();
        sw.stop();
        // println!(
        //     "Rendering took {}ms, {:.2} FPS",
        //     sw.elapsed_ms(),
        //     1000f64 / sw.elapsed_ms() as f64
        // );
    }
}

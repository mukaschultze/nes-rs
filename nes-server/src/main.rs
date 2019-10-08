extern crate nes_core;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::cell::RefCell;
use std::env;
use std::path::Path;
use std::rc::Rc;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;

use nes_core::console::NesConsole;
use nes_core::controller::Controller;
use nes_core::controller::ControllerDataLine;
use nes_core::palette;
use nes_core::rom::rom_file::RomFile;
use sdl2::rect::Point;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 240;

const KEYMAPS: [(Keycode, ControllerDataLine); 8] = [
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

    let (input_tx, input_rx) = channel::<ControllerDataLine>();
    let (vsync_tx, vsync_rx) = channel::<Arc<[u8; (WIDTH * HEIGHT) as usize]>>();

    let tx = vsync_tx.clone();

    thread::spawn(move || {
        println!("Loading ROM from {}", args[1]);
        let rom_path = Path::new(&args[1]);
        let rom = Rc::new(RefCell::new(RomFile::from_file(rom_path)));
        let mut nes = NesConsole::new(rom);
        static mut RENDER_REQUEST: bool = false;

        {
            let mut bus = nes.bus.borrow_mut();
            let controller = Controller::new();
            bus.controller0 = Some(controller);
        }

        {
            let mut ppu = nes.ppu.borrow_mut();

            ppu.v_blank_callback = Box::new(|| unsafe {
                RENDER_REQUEST = true;
            });
        }

        loop {
            nes.tick();

            unsafe {
                if RENDER_REQUEST {
                    let mut bus = nes.bus.borrow_mut();
                    let controller = bus.controller0.as_mut().expect("No controller 0 connected");
                    controller.data = input_rx.recv().unwrap();

                    let arc = Arc::from(nes.ppu.borrow().output);
                    tx.send(arc).unwrap();

                    RENDER_REQUEST = false;
                }
            }
        }
    });

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

    let mut input = ControllerDataLine::empty();

    'main: loop {
        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit { .. } => break 'main,

                Event::KeyDown { keycode, .. } => {
                    for (src, dst) in &KEYMAPS {
                        if keycode == Some(*src) {
                            input.insert(*dst);
                        }
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    for (src, dst) in &KEYMAPS {
                        if keycode == Some(*src) {
                            input.remove(*dst);
                        }
                    }
                }

                // evt => println!("Event received: {:?}", evt),
                _ => {}
            }
        }

        input_tx.send(input).unwrap();

        // let clear_rgb = palette::get_rgb_color(nes.ppu.borrow().paletteRAM[0]);
        // let r = ((clear_rgb >> 16) & 0xFF) as u8;
        // let g = ((clear_rgb >> 8) & 0xFF) as u8;
        // let b = (clear_rgb & 0xFF) as u8;

        // canvas.set_draw_color(Color::RGB(r, g, b));
        // canvas.clear();

        // let nes = vsync_rx.recv().unwrap();
        // let output = &nes.ppu.borrow().output;
        let output = vsync_rx.recv().unwrap();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let color_idx = output[(WIDTH * y + x) as usize];

                let rgb = palette::get_rgb_color(color_idx);
                let r = ((rgb >> 16) & 0xFF) as u8;
                let g = ((rgb >> 8) & 0xFF) as u8;
                let b = (rgb & 0xFF) as u8;

                let color = Color::RGB(r, g, b);
                let point = Point::new(x as i32, y as i32);

                canvas.set_draw_color(color);
                canvas.draw_point(point).unwrap();
            }
        }

        canvas.present();
    }
}

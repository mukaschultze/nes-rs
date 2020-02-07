#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate gl;
extern crate nes_core;
extern crate nfd;
extern crate png;
extern crate sdl2;
extern crate stopwatch;

use stopwatch::Stopwatch;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::surface::Surface;
use sdl2::video::Window;

use std::env;
use std::path::Path;
use std::process::exit;

use nfd::Response;

use nes_core::console::NesConsole;
use nes_core::controller::Controller;
use nes_core::controller::ControllerDataLine;
use nes_core::palette;
use nes_core::rom::rom_file::RomFile;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 240;

const KEYMAPS: &[(Keycode, ControllerDataLine)] = &[
    (Keycode::Z, ControllerDataLine::A),
    (Keycode::X, ControllerDataLine::B),
    (Keycode::Return, ControllerDataLine::SELECT),
    (Keycode::Space, ControllerDataLine::START),
    (Keycode::Up, ControllerDataLine::UP),
    (Keycode::Down, ControllerDataLine::DOWN),
    (Keycode::Left, ControllerDataLine::LEFT),
    (Keycode::Right, ControllerDataLine::RIGHT),
];

fn main() -> ! {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        let result = nfd::open_file_dialog(Some("nes"), None).unwrap_or_else(|e| {
            panic!(e);
        });

        match result {
            Response::Okay(file_path) => start(Path::new(&file_path)),
            Response::OkayMultiple(_files) => unreachable!(),
            Response::Cancel => panic!("No ROM file specified"),
        }
    } else {
        start(Path::new(&args[1]));
    }
}

fn start(rom_path: &Path) -> ! {
    println!("Loading ROM from {}", rom_path.display());
    let mut rom = RomFile::from_file(rom_path);
    let mut nes = NesConsole::new();

    let mut output_buffer = vec![0; (WIDTH * HEIGHT) as usize];
    let (mut output_buffer_scaled, scaled_width, scaled_height) =
        nes_core::xbr::get_buffer_for_size(WIDTH, HEIGHT);

    nes.bus.borrow_mut().connect_cartridge(&mut rom);

    {
        let mut bus = nes.bus.borrow_mut();
        let controller = Controller::new();
        bus.controller0 = Some(controller);
    }

    // http://nercury.github.io/rust/opengl/tutorial/2018/02/08/opengl-in-rust-from-scratch-01-window.html
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();

    let mut window = video_subsystem
        .window("NES", scaled_width, scaled_height)
        .resizable()
        .opengl()
        .build()
        .unwrap();

    set_icon(&mut window);

    let mut canvas: Canvas<Window> = window
        .into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .present_vsync() // this means the screen cannot render faster than your display rate (usually 60Hz or 144Hz)
        .accelerated()
        .build()
        .unwrap();

    canvas
        .set_logical_size(scaled_width, scaled_height)
        .unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    canvas.window().gl_set_context_to_current().unwrap();

    nes.reset();

    loop {
        nes.render_full_frame();

        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit { .. } => exit(0),

                Event::KeyDown { keycode, .. } => {
                    if let Some(controller) = nes.bus.borrow_mut().controller0.as_mut() {
                        for (src, dst) in KEYMAPS {
                            if keycode == Some(*src) {
                                controller.data.insert(*dst);
                            }
                        }
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(controller) = nes.bus.borrow_mut().controller0.as_mut() {
                        for (src, dst) in KEYMAPS {
                            if keycode == Some(*src) {
                                controller.data.remove(*dst);
                            }
                        }
                    }

                    match keycode {
                        Some(Keycode::R) => nes.reset(),
                        Some(Keycode::T) => nes.screenshot("nes_screenshot.png"),
                        _ => {}
                    };
                }

                // evt => println!("Event received: {:?}", evt),
                _ => {}
            }
        }

        let mut sw = Stopwatch::start_new();
        let clear_color_idx = nes.ppu.borrow().palette_vram[0];
        let clear_color: Color = palette::get_rgb_color_split(clear_color_idx).into();
        canvas.set_draw_color(clear_color);
        canvas.clear();

        nes.get_output_rgb_u32(&mut output_buffer);
        nes_core::xbr::apply(&mut output_buffer_scaled, &output_buffer, WIDTH, HEIGHT);

        // TODO: Use frame buffer, migrade from SDL2
        for y in 0..scaled_height {
            for x in 0..scaled_width {
                let idx = (scaled_width * y + x) as usize;
                let color = Color::RGB(
                    ((output_buffer_scaled[idx] >> 16) & 0xFF) as u8,
                    ((output_buffer_scaled[idx] >> 8) & 0xFF) as u8,
                    (output_buffer_scaled[idx] & 0xFF) as u8,
                );
                let point = Point::new(x as i32, y as i32);

                if color.rgb() != clear_color.rgb() {
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

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

fn set_icon(window: &mut Window) {
    const ICON_SRC: &[u8] = include_bytes!("./icon.png");
    let decoder = png::Decoder::new(ICON_SRC);
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    let icon = Surface::from_data(
        &mut buf,
        info.width,
        info.height,
        info.line_size as u32,
        PixelFormatEnum::RGBA32,
    )
    .unwrap();

    window.set_icon(icon);
}

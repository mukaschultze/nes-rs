#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate gl;
extern crate nes_core;
extern crate nfd;
extern crate png;
extern crate stopwatch;
extern crate winit;

use stopwatch::Stopwatch;

use std::env;
use std::path::Path;

use pixels::wgpu::TextureFormat;
use pixels::PixelsBuilder;
use pixels::SurfaceTexture;

use winit::dpi::LogicalSize;
use winit::event::Event;
use winit::event::VirtualKeyCode;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::Fullscreen;
use winit::window::Icon;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use nfd::Response;

use nes_core::console::NesConsole;
use nes_core::console::NES_HEIGHT;
use nes_core::console::NES_WIDTH;
use nes_core::input::joypad::Joypad;
use nes_core::input::joypad::JoypadDataLine;
use nes_core::input::InputType;
use nes_core::rom::rom_file::RomFile;

const TARGET_FRAMERATE: i64 = 60;
const HIGH_QUALITY: bool = false;

const KEYMAPS: &[(VirtualKeyCode, JoypadDataLine)] = &[
    (VirtualKeyCode::Z, JoypadDataLine::A),
    (VirtualKeyCode::X, JoypadDataLine::B),
    (VirtualKeyCode::Return, JoypadDataLine::SELECT),
    (VirtualKeyCode::Space, JoypadDataLine::START),
    (VirtualKeyCode::Up, JoypadDataLine::UP),
    (VirtualKeyCode::Down, JoypadDataLine::DOWN),
    (VirtualKeyCode::Left, JoypadDataLine::LEFT),
    (VirtualKeyCode::Right, JoypadDataLine::RIGHT),
];

fn main() -> ! {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        let result = nfd::open_file_dialog(Some("nes"), None).unwrap_or_else(|e| {
            panic!("{}", e);
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

fn load_nes(rom_path: &Path) -> NesConsole {
    println!("Loading ROM from {}", rom_path.display());
    let rom = RomFile::from_file(rom_path);
    let mut nes = NesConsole::new();

    nes.bus.borrow_mut().connect_cartridge(rom);

    {
        let mut bus = nes.bus.borrow_mut();
        let joypad = Joypad::new();
        bus.input0 = InputType::Joypad(joypad);
    }

    nes.reset();
    nes
}

fn start(rom_path: &Path) -> ! {
    let mut nes = load_nes(rom_path);

    // Generate output buffers
    let mut output_buffer = vec![0; (NES_WIDTH * NES_HEIGHT) as usize];
    let (mut output_buffer_scaled, scaled_width, scaled_height) =
        nes_core::xbr::get_buffer_for_size(NES_WIDTH, NES_HEIGHT);

    let (width, height) = if HIGH_QUALITY {
        (scaled_width, scaled_height)
    } else {
        (NES_WIDTH, NES_HEIGHT)
    };

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(width as f64, height as f64);
        WindowBuilder::new()
            .with_title("NES Emulator")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_window_icon(Some(get_icon()))
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let surface_texture = SurfaceTexture::new(width, height, &window);

        PixelsBuilder::new(width, height, surface_texture)
            .texture_format(TextureFormat::Bgra8UnormSrgb)
            .build()
            .unwrap()
    };

    let mut sw = Stopwatch::start_new();
    let mut sync = Stopwatch::start_new();
    let mut frames = 0;
    let mut rendered_frames = 0;

    event_loop.run(move |event, _, control_flow| {
        match &event {
            Event::RedrawRequested { .. } => {
                rendered_frames += 1;
                nes.get_output_rgb_u32(&mut output_buffer);

                let src = if HIGH_QUALITY {
                    nes_core::xbr::apply(
                        &mut output_buffer_scaled,
                        &output_buffer,
                        NES_WIDTH,
                        NES_HEIGHT,
                    );
                    &output_buffer_scaled
                } else {
                    &output_buffer
                };

                unsafe {
                    let pixels_buf = pixels.get_frame();
                    std::ptr::copy_nonoverlapping(
                        src.as_ptr() as *mut u8,
                        pixels_buf.as_mut_ptr(),
                        pixels_buf.len(),
                    );
                }
                pixels.render().expect("failed to render");
            }
            Event::WindowEvent { event, .. } => {
                if let Some(size) = match event {
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        Some(**new_inner_size)
                    }
                    WindowEvent::Resized(size) => Some(*size),
                    _ => None,
                } {
                    println!("Resized to {}x{}", size.width, size.height);
                    pixels.resize_surface(size.width, size.height);
                }
            }
            _ => {}
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
            }
            if input.key_released(VirtualKeyCode::R) {
                nes.reset();
            }
            if input.key_released(VirtualKeyCode::T) {
                nes.screenshot("nes_screenshot.png");
                println!("Screenshot taken");
            }
            if input.key_released(VirtualKeyCode::F) {
                let fs = window.fullscreen();
                let new_fs = if let Some(_) = fs {
                    None
                } else {
                    Some(Fullscreen::Borderless(window.current_monitor()))
                };

                window.set_fullscreen(new_fs);
                println!("Fullscreen changed");
            }

            if let InputType::Joypad(joypad) = &mut nes.bus.borrow_mut().input0 {
                for (src, dst) in KEYMAPS {
                    if input.key_pressed(*src) {
                        joypad.data.insert(*dst);
                    }
                    if input.key_released(*src) {
                        joypad.data.remove(*dst);
                    }
                }
            }
        }

        if sw.elapsed_ms() > 1000 {
            sw.restart();
            println!("FPS: {}, rendered {}", frames, rendered_frames);
            frames = 0;
            rendered_frames = 0;
        }

        if sync.elapsed_ms() >= (1000 / TARGET_FRAMERATE) {
            sync.restart();
            frames += 1;
            nes.render_full_frame();
            window.request_redraw();
        }
    });
}

fn get_icon() -> Icon {
    const ICON_SRC: &[u8] = include_bytes!("./icon.png");
    let decoder = png::Decoder::new(ICON_SRC);
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    Icon::from_rgba(buf, info.width, info.height).unwrap()
}

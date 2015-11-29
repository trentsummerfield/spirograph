extern crate sdl2;

mod sim_interface;
mod sim;

use std::iter::FromIterator;

use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sim_interface::Bitmap;
use sim_interface::Input;

fn blit(bitmap: &Bitmap, texture: &mut sdl2::render::Texture, renderer: &mut sdl2::render::Renderer) {
    let texture_query = texture.query();
    let width = texture_query.width as usize;
    let height = texture_query.height as usize;
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        for y in (0..height) {
            for x in (0..width) {
                let offset = y * pitch + x * 4;
                let i = y * bitmap.pitch as usize + x * 4;
                buffer[offset + 0] = bitmap.memory[i + 0];
                buffer[offset + 1] = bitmap.memory[i + 1];
                buffer[offset + 2] = bitmap.memory[i + 2];
                buffer[offset + 3] = bitmap.memory[i + 3];
            }
        }
    }).unwrap();

    renderer.clear();
    renderer.copy(&texture, None, None);
    renderer.present();
}

fn main() {
    let window_name = "Spirograph";
    let width = 960;
    let height = 540;
    let target_ms_frame = 1000.0 / 60.0;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let timers = sdl_context.timer().unwrap();

    let window = video_subsystem
        .window(window_name, width, height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().present_vsync().build().unwrap();
    let mut texture = renderer.create_texture_streaming(PixelFormatEnum::ARGB8888, (width, height)).unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut bitmap = Bitmap::new(width as usize, height as usize);

    let perf_freq = timers.performance_frequency() as f64;

    let mut start_time = timers.performance_counter();
    let mut running = true;
    let mut input = Input::new(Vec::from_iter(std::env::args()));
    let mut sim_state = None;
    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    running = false;
                },
                Event::KeyDown { keycode: Some(Keycode::W), ..} => {
                    input.keyboard.up.ended_down = true;
                },
                Event::KeyDown { keycode: Some(Keycode::A), ..} => {
                    input.keyboard.left.ended_down = true;
                },
                Event::KeyDown { keycode: Some(Keycode::S), ..} => {
                    input.keyboard.down.ended_down = true;
                },
                Event::KeyDown { keycode: Some(Keycode::D), ..} => {
                    input.keyboard.right.ended_down = true;
                },
                Event::KeyUp { keycode: Some(Keycode::W), ..} => {
                    input.keyboard.up.ended_down = false;
                },
                Event::KeyUp { keycode: Some(Keycode::A), ..} => {
                    input.keyboard.left.ended_down = false;
                },
                Event::KeyUp { keycode: Some(Keycode::S), ..} => {
                    input.keyboard.down.ended_down = false;
                },
                Event::KeyUp { keycode: Some(Keycode::D), ..} => {
                    input.keyboard.right.ended_down = false;
                },
                _ => {}
            }
        }
        sim_state = Some(sim::update_and_render(&mut bitmap, &input, sim_state));
        blit(&bitmap, &mut texture, &mut renderer);

        let mut elapsed = (1000 * (timers.performance_counter() - start_time)) as f64 / perf_freq;
        output_elapsed_time(elapsed);
        while target_ms_frame - elapsed > 0.0 {
            elapsed = (1000 * (timers.performance_counter() - start_time)) as f64 / perf_freq;
        }

        output_elapsed_time(elapsed);
        start_time = timers.performance_counter();
    }
}

#[cfg(feature = "internal")]
fn output_elapsed_time(elapsed: f64) {
    println!("{:3.4} ms/f", elapsed);
}

#[cfg(not(feature = "internal"))]
fn output_elapsed_time(_: f64) {
}

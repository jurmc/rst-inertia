extern crate sdl2;
extern crate cairo;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface as SDLSurface;

use cairo::{ ImageSurface, Format, Context };

use std::f64::consts::PI;
use std::time::Duration;

pub fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let video_subsystem = sdl_ctx.video().unwrap();

    let x_width: u32 = 300;
    let y_height: u32 = 300;

    let window = video_subsystem.window("rust-sdl2-cairo-example", x_width, y_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.clear();

    let masks = PixelFormatEnum::BGRA32.into_masks().unwrap();
    let sdl_surface = SDLSurface::from_pixelmasks(x_width, y_height, &masks).unwrap();

    let width = sdl_surface.width() as i32;
    let height = sdl_surface.height() as i32;
    let pitch = sdl_surface.pitch() as i32;

    let dest_surface: ImageSurface;
    unsafe {
        dest_surface = ImageSurface::create_for_data_unsafe((*(sdl_surface.raw())).pixels as *mut u8, Format::ARgb32, width, height, pitch)
            .expect("Couldn't create Cairo surface (using pixels from SDL surface)");
    };
    let ctx = Context::new(&dest_surface).unwrap();

    //SDL_SetRenderDrawColor(renderer, 0, 0, 0, 0); // TODO: do we have anythign useful from it?
    //SDL_RenderClear(renderer); // TODO: do we have anythign useful from it?

    // Lines
    ctx.set_source_rgb(1.0, 1.0, 1.0);
    ctx.paint();

    ctx.set_source_rgba(0.1, 0.1, 7.0, 0.95);
    ctx.move_to(150.0 - 75.0, 150.0);
    ctx.line_to(150.0, 150.0);
    ctx.line_to(150.0, 150.0 - 75.0);

    ctx.move_to(150.0 + 10.0, 150.0 + 10.0);
    ctx.line_to(150.0 + 75.0, 150.0 + 75.0);

    ctx.move_to(150.0, 150.0 - 75.0);
    ctx.new_sub_path();
    ctx.arc(150.0, 150.0, 75.0, 180.0 * (PI / 180.0), 270.0 * (PI / 180.0));

    ctx.set_line_width(10.0);
    ctx.stroke().unwrap();

    ctx.set_source_rgba(1.0, 0.2, 0.2, 0.5);
    ctx.new_sub_path();
    ctx.arc(150.0, 150.0 - 75.0, 20.0, 0.0, 2.0 * PI);
    ctx.new_sub_path();
    ctx.arc(150.0 - 75.0, 150.0, 20.0, 0.0, 2.0 * PI);
    ctx.fill();

    let texture_creator = canvas.texture_creator();
    let new_texture = texture_creator.create_texture_from_surface(sdl_surface).unwrap();
    canvas.copy(&new_texture,None,None).unwrap();
    canvas.present();

    let mut event_pump = sdl_ctx.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

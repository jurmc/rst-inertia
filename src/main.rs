extern crate sdl2;
extern crate cairo;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::TextureAccess;

use cairo::ImageSurface;
use cairo::Format;
use cairo::Context;
use cairo::Error;

use std::f64::consts::PI;
use std::time::Duration;
use std::mem;
//use rand::prelude::*;
use rand::thread_rng;
use rand::Rng;

const ACC: f64 = 0.05;

struct Particle {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
}

impl Particle {
    fn new(ball: &Ball) -> Particle {
        Particle {
            x: ball.x,
            y: ball.y, // TODO: position in the midlle of screen
            vx: ball.vx + thread_rng().gen_range(-1.5..1.5),
            vy: ball.vy + thread_rng().gen_range(-3.0..-1.5),
        }
    }

    fn tick(&mut self) {
        self.vy += ACC;

        self.x += self.vx;
        self.y += self.vy;
    }

    fn draw(&self, surface: &ImageSurface) -> Result<(), Error> {
        let ctx = Context::new(&surface).unwrap();
        ctx.translate(self.x, self.y);

        ctx.set_source_rgba(0.9, 0.9, 0.9, 1.0);
        ctx.arc(0.0, 0.0, 1.0, 0.0, 2.0 * PI);
        ctx.fill()?;

        Ok(())
    }
}

struct Blow {
    particles: Vec<Particle>,
    cntr: u32,
}

impl Blow {
    fn new(ball: &Ball) -> Blow {
        let mut particles: Vec<Particle> = Vec::new();
        for _ in 0..80 {
            particles.push(Particle::new(ball))
        }
        Blow {
            particles: particles,
            cntr: 0,
        }
    }

    fn tick(&mut self) {
        self.particles.iter_mut().for_each(|p| p.tick());
        self.particles.retain(|p| p.y < 480.0);
    }

    fn draw(&self, surface: &ImageSurface) -> Result<(), Error> {
        self.particles.iter().for_each(|p| p.draw(surface).unwrap());
        Ok(())
    }
}

#[derive(Clone, PartialEq)]
struct Ball {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
}

impl Ball {
    fn new() -> Ball {
        Ball {
            x: thread_rng().gen_range(0.0..640.0),
            y: 0.0,
            vx: thread_rng().gen_range(-2.0..2.0),
            vy: 0.0
        }
    }

    fn draw(&self, surface: &ImageSurface) -> Result<(), Error> {
        let ctx = Context::new(&surface).unwrap();
        ctx.translate(self.x, self.y);

        ctx.set_source_rgba(1.0, 0.5, 0.5, 1.0);
        ctx.arc(0.0, 0.0, 5.0, 0.0, 2.0 * PI);
        ctx.fill()?;

        Ok(())
    }

    fn tick(&mut self) {
        self.vy += ACC;   // TODO: consider t_diff
        self.y += self.vy; // TODO: consider t_diff

        if self.y > 480.0 {
            self.vy = -self.vy;
        }

        self.x += self.vx;
        if self.x < 0.0 || self.x > 640.0 {
            self.vx = -self.vx
        }
    }
}

fn draw_player(x: f64, y: f64, rotation: f64,       // Placement
              scale: f64, radius: f64, angle: f64, // Shape geometry
              surface: &ImageSurface) -> Result<(), Error> {
    let ctx = Context::new(&surface).unwrap();
    ctx.translate(x, y);
    ctx.scale(scale, scale);
    ctx.rotate(rotation * PI/180.0);

    // Arc
    let (xc, yc) = (0.0, 0.0);
    let a1 = PI/180.0;
    let a2 = (a1 + angle) * (PI/180.0);

    ctx.set_source_rgba(0.90, 0.85, 0.25, 1.0);
    ctx.set_line_width(5.0);
    ctx.arc(xc, yc, radius, a1, a2);
    ctx.stroke()?;

    // Dot
    ctx.set_source_rgba(1.0, 0.2, 0.2, 0.6);
    ctx.set_line_width(3.0);
    ctx.arc(xc, yc, 5.0, 0.0, 2.0 * PI);
    ctx.fill()?;

    // Lines
    ctx.arc(xc, yc, radius, a1, a1);
    ctx.line_to(xc, yc);
    ctx.arc(xc, yc, radius, a2, a2);
    ctx.line_to(xc, yc);
    ctx.stroke()?;

    Ok(())
}

pub fn main() -> Result<(), Error> {
    let sdl_ctx = sdl2::init().unwrap();
    let video_subsystem = sdl_ctx.video().unwrap();

    let window = video_subsystem.window("rust-sdl2-cairo-example", 640, 480)
        .position_centered()
        .fullscreen()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.clear();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture(PixelFormatEnum::BGRA32, TextureAccess::Streaming, 640, 480).unwrap();

    let mut pixels = [0u8; 640 * 480 * 4];
    let cairo_surface: ImageSurface;
    unsafe {
        cairo_surface = ImageSurface::create_for_data_unsafe(pixels[..].as_mut_ptr(), Format::ARgb32, 640, 480, (640 * 4 * mem::size_of::<u8>()) as i32)
            .expect("Couldn't create Cairo surface (using pixels from SDL surface)");
    }

    let (mut x, mut y) = (640.0 / 2.0, 480.0 / 2.0);
    let mut rotation = 0;
    let mut radius = 100.0;
    let mut angle = 135.0;
    let mut event_pump = sdl_ctx.event_pump().unwrap();

    let mut balls: Vec<Ball> = Vec::new();
    let mut blows: Vec<Blow> = Vec::new();

    'running: loop {

        if rotation as u32 % 30 == 0 {
            if balls.len() < 25 {
                balls.push(Ball::new());
            }
            blows.retain(|b| b.particles.len() > 0);
        }

        rotation = (rotation + 3) % 360;

        // Background
        let ctx = Context::new(&cairo_surface).unwrap();
        ctx.set_source_rgba(0.2, 0.2, 0.2, 1.0);
        ctx.paint()?;

        //draw_player(x, y, rotation as f64, 1.0, radius, angle, &cairo_surface)?;

        balls.iter_mut().for_each(|ball| {
            ball.draw(&cairo_surface);
            ball.tick();
        });
        blows.iter_mut().for_each(|blow| {
            blow.draw(&cairo_surface);
            blow.tick();
        });
        cairo_surface.flush();

        // Collision detection
        let balls_clone = balls.clone();
        let mut balls_collided: Vec<Ball> = Vec::new();
        balls.retain(|b1| {
            let mut result = true;
            balls_clone.iter().for_each(|b2| {
                if b2 != b1 {
                    let len = ((b1.x - b2.x).powi(2) + (b1.y - b2.y).powi(2)).sqrt();
                    if len < 10.0 {
                        result = false;
                        balls_collided.push(b2.clone());
                    }
                }
            });
            result
        });
        balls_collided.iter().for_each(|b| blows.push(Blow::new(b)));

        texture.update(None, &mut pixels[..], 640 * 4 * mem::size_of::<u8>()).unwrap();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    x = x - 4.0;
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    x = x + 4.0;
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    y = y - 4.0;
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    y = y + 4.0;
                },
                Event::KeyDown { keycode: Some(Keycode::UP), .. } => {
                    radius = radius + 2.0;
                },
                Event::KeyDown { keycode: Some(Keycode::DOWN), .. } => {
                    radius = radius - 2.0;
                },
                Event::KeyDown { keycode: Some(Keycode::LEFT), .. } => {
                    angle = angle + 4.0;
                },
                Event::KeyDown { keycode: Some(Keycode::RIGHT), .. } => {
                    angle = angle - 4.0;
                },
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 500_000_000u32 / 60));
    }

    Ok(())
}

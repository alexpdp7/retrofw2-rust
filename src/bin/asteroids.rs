extern crate sdl;

use std::num::Wrapping;

use modulo::Mod;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use sdl::event::Event;

use retrofw2_rust::controls::*;
use retrofw2_rust::geom::Painter;

struct Asteroids {
    screen: sdl::video::Surface,
    video_info: sdl::video::VideoInfo,
    height: isize,
    prop_width: f32,
    pressed_keys: PressedKeys,
    ship: Ship,
    sin_cos_lut: [(f32, f32); 256],
    frames: usize,
    start: std::time::Instant,
    last_frame: std::time::Instant,
    stars: [Star; 100],
}

impl Asteroids {
    fn new() -> Asteroids {
        let (screen, video_info) = retrofw2_rust::gfx::init();
        let width = video_info.width;
        let height = video_info.height;
        let prop_width = width as f32 / height as f32;
        Asteroids {
            screen,
            video_info,
            height,
            prop_width,
            pressed_keys: PressedKeys::default(),
            ship: Ship {
                x: prop_width / 2.0,
                y: 0.5,
                rot: Wrapping(0u8),
                dx: 0.0,
                dy: 0.0,
            },
            sin_cos_lut: retrofw2_rust::geom::get_sin_cos_lut(),
            frames: 0,
            start: std::time::Instant::now(),
            last_frame: std::time::Instant::now(),
            stars: make_stars(prop_width),
        }
    }

    fn main(mut self) {
        self.start = std::time::Instant::now();
        'main: loop {
            self.last_frame = std::time::Instant::now();
            'event: loop {
                let event = sdl::event::poll_event();
                self.pressed_keys.process_key(&event);
                match event {
                    Event::Quit => break 'main,
                    Event::None => break 'event,
                    _ => {}
                }
            }

            if self.pressed_keys.is_pressed(CONTROL_SELECT) {
                break;
            }

            if self.pressed_keys.is_pressed(CONTROL_LEFT) {
                self.ship.rot += std::num::Wrapping(4);
            }

            if self.pressed_keys.is_pressed(CONTROL_RIGHT) {
                self.ship.rot -= std::num::Wrapping(4);
            }

            if self.pressed_keys.is_pressed(CONTROL_B) {
                let d = self.sin_cos_lut[self.ship.rot.0 as usize];
                self.ship.dx += d.0;
                self.ship.dy += d.1;
            }

            self.ship.dx *= 0.9;
            self.ship.dy *= 0.9;

            for mut star in self.stars.iter_mut() {
                star.x -= self.ship.dx * (star.depth as f32) / 500_000.0;
                star.y -= self.ship.dy * (star.depth as f32) / 500_000.0;
                star.x = star.x.modulo(self.prop_width);
                star.y = star.y.modulo(1.0);
            }

            self.screen.fill(sdl::video::Color::RGB(0, 0, 0));
            self.frames += 1;

            let _draw = |pixels: &mut [u8]| -> bool {
                let mut painter = Painter {
                    video_info: &self.video_info,
                    pixels: std::boxed::Box::new(pixels),
                };
                self.draw_ship(&mut painter);
                self.draw_stars(&mut painter);
                true
            };
            self.screen.with_lock(_draw);

            self.screen.flip();
            if cfg!(feature = "throttle") {
                let now = std::time::Instant::now();
                let frame_diff = now.duration_since(self.last_frame);
                if frame_diff < std::time::Duration::from_millis(16) {
                    std::thread::sleep(std::time::Duration::from_millis(16) - frame_diff);
                }
            }
        }

        sdl::quit();
        println!(
            "{}",
            1000.0 * self.frames as f32 / self.start.elapsed().as_millis() as f32
        );
    }

    fn draw_ship(&self, painter: &mut Painter) {
        self.draw_polar_line(
            painter,
            self.ship.x,
            self.ship.y,
            self.ship.rot,
            0.02,
            self.ship.rot + Wrapping(40u8),
            -0.02,
        );
        self.draw_polar_line(
            painter,
            self.ship.x,
            self.ship.y,
            self.ship.rot,
            0.02,
            self.ship.rot - Wrapping(40u8),
            -0.02,
        );
        self.draw_polar_line(
            painter,
            self.ship.x,
            self.ship.y,
            self.ship.rot,
            0.0,
            self.ship.rot + Wrapping(40u8),
            -0.02,
        );
        self.draw_polar_line(
            painter,
            self.ship.x,
            self.ship.y,
            self.ship.rot,
            0.0,
            self.ship.rot - Wrapping(40u8),
            -0.02,
        );
    }

    fn draw_stars(&self, painter: &mut Painter) {
        for star in self.stars.iter() {
            painter.put_pixel(
                (star.x * self.height as f32) as isize,
                (star.y * self.height as f32) as isize,
                star.depth,
                star.depth,
                star.depth,
            );
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn draw_polar_line(
        &self,
        painter: &mut Painter,
        cx: f32,
        cy: f32,
        rot1: Wrapping<u8>,
        rad1: f32,
        rot2: Wrapping<u8>,
        rad2: f32,
    ) {
        let (sx1, sy1) = self.sin_cos_lut[rot1.0 as usize];
        let (sx2, sy2) = self.sin_cos_lut[rot2.0 as usize];
        painter.draw_line(
            ((cx + rad1 * sx1) * self.height as f32) as isize,
            ((cy + rad1 * sy1) * self.height as f32) as isize,
            ((cx + rad2 * sx2) * self.height as f32) as isize,
            ((cy + rad2 * sy2) * self.height as f32) as isize,
            255,
            255,
            255,
        );
    }
}

struct Ship {
    x: f32,
    y: f32,
    rot: Wrapping<u8>,
    dx: f32,
    dy: f32,
}

#[derive(Default, Copy, Clone)]
struct Star {
    x: f32,
    y: f32,
    depth: u8,
}

fn make_stars(prop_width: f32) -> [Star; 100] {
    let mut stars = [Star::default(); 100];
    let mut rng = SmallRng::from_entropy();
    for mut star in stars.iter_mut() {
        star.x = rng.gen_range(0.0, prop_width);
        star.y = rng.gen_range(0.0, 1.0);
        star.depth = rng.gen_range(0, 255);
    }
    stars
}

fn main() {
    Asteroids::new().main();
}

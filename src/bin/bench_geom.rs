extern crate sdl;

use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};

use sdl::event::Event;

use retrofw2_rust::controls::*;
use retrofw2_rust::geom::Painter;

fn main() {
    let (screen, video_info) = retrofw2_rust::gfx::init();
    let mut pressed_keys =  PressedKeys::default();
    let start = std::time::Instant::now();
    let mut frames: usize = 0;
    let sin_cos_lut = retrofw2_rust::geom::get_sin_cos_lut();
    'main: loop {
        'event: loop {
            let event = sdl::event::poll_event();
            pressed_keys.process_key(&event);
            match event {
                Event::Quit => break 'main,
                Event::None => break 'event,
                _ => {}
            }
        }

        if pressed_keys.is_pressed(CONTROL_SELECT) {
            break;
        }

        screen.fill(sdl::video::Color::RGB(0, 0, 0));
        frames += 1;

        let _draw = |pixels: &mut [u8]| -> bool {
            let mut painter = Painter {
                video_info: &video_info,
                pixels: std::boxed::Box::new(pixels),
            };
            let mut rng = SmallRng::from_entropy();
            for _ in 0..10 {
                let (dx, dy) = sin_cos_lut[frames % 256];
                painter.draw_line(160, 120, 160 + (dx*10.0) as isize, 120 + (dy*10.0) as isize, rng.b(), rng.b(), rng.b());
            }
            true
        };
        screen.with_lock(_draw);
        screen.flip();
    }
    sdl::quit();
    println!("{}", 1000.0 * frames as f32 / start.elapsed().as_millis() as f32);
}

trait Foo {
    fn x(&mut self) -> isize;
    fn y(&mut self) -> isize;
    fn b(&mut self) -> u8;
}

impl Foo for SmallRng {
    fn x(&mut self) -> isize { (self.next_u32() % 320) as isize}
    fn y(&mut self) -> isize { (self.next_u32() % 240) as isize }
    fn b(&mut self) -> u8 { self.next_u32() as u8}

}
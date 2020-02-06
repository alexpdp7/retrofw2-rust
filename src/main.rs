extern crate rand;
extern crate sdl;

use rand::Rng;

use sdl::video::{SurfaceFlag, VideoFlag};
use sdl::event::{Event, Key};

fn main() {
    sdl::init(&[sdl::InitFlag::Video]);
    sdl::wm::set_caption("rust-sdl demo - video", "rust-sdl");

    let mut rng = rand::thread_rng();
    let screen = match sdl::video::set_video_mode(320, 480, 16,
                                                  &[SurfaceFlag::SWSurface],
                                                  &[VideoFlag::DoubleBuf]) {
        Ok(screen) => screen,
        Err(err) => panic!("failed to set video mode: {}", err)
    };

    // Note: You'll want to put this and the flip call inside the main loop
    // but we don't as to not startle epileptics
    for i in 0usize..10 {
        for j in 0usize..10 {
            screen.fill_rect(Some(sdl::Rect {
                x: (i as i16) * 320 / 10,
                y: (j as i16) * 480 / 10,
                w: 320 / 10,
                h: 480 / 10
            }), rng.gen::<sdl::video::Color>());
        }
    }

    screen.flip();

    'main : loop {
        'event : loop {
            match sdl::event::poll_event() {
                Event::Quit => break 'main,
                Event::None => break 'event,
                Event::Key(k, _, _, _)
                    if k == Key::Escape
                        => break 'main,
                _ => {}
            }
        }
    }

    sdl::quit();
}

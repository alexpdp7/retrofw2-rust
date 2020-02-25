extern crate rand;
extern crate sdl;

use rand::Rng;

use sdl::event::{Event, Key};
use sdl::video::{SurfaceFlag, VideoFlag};

const SIZE_X: i16 = 320;
const SIZE_Y: i16 = 240;

// const CONTROL_UP: Key = Key::Up;
// const CONTROL_RIGHT: Key = Key::Right;
// const CONTROL_DOWN: Key = Key::Down;
// const CONTROL_LEFT: Key = Key::Left;
// const CONTROL_A: Key = Key::LCtrl;
// const CONTROL_B: Key = Key::LAlt;
// const CONTROL_X: Key = Key::Space;
// const CONTROL_Y: Key = Key::LShift;
// const CONTROL_TRIGGER_LEFT: Key = Key::Tab;
// const CONTROL_TRIGGER_RIGHT: Key = Key::Backspace;
const CONTROL_SELECT: Key = Key::Escape;
// const CONTROL_START: Key = Key::Return;

fn main() {
    sdl::init(&[sdl::InitFlag::Video]);
    sdl::wm::set_caption("rust-sdl demo - video", "rust-sdl");

    let mut rng = rand::thread_rng();
    let screen = match sdl::video::set_video_mode(
        SIZE_X as isize,
        SIZE_Y as isize,
        16,
        &[SurfaceFlag::SWSurface],
        &[VideoFlag::DoubleBuf],
    ) {
        Ok(screen) => screen,
        Err(err) => panic!("failed to set video mode: {}", err),
    };

    'main: loop {
        'event: loop {
            match sdl::event::poll_event() {
                Event::Quit => break 'main,
                Event::None => break 'event,
                // Key(_, false, _) means key up, true key down
                Event::Key(CONTROL_SELECT, false, _, _) => break 'main,
                _ => {}
            }
        }

        for i in 0..SIZE_X {
            for j in 0..SIZE_Y {
                screen.fill_rect(
                    Some(sdl::Rect {
                        x: i,
                        y: j,
                        w: 1,
                        h: 1,
                    }),
                    rng.gen::<sdl::video::Color>(),
                );
            }
        }
        screen.flip();
    }

    sdl::quit();
}

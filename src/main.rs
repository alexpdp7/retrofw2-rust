extern crate rand;
extern crate sdl;

use std::collections::HashMap;

use sdl::event::{Event, Key};
use sdl::video::{Color, Surface, SurfaceFlag, VideoFlag};

const SIZE_X: i16 = 320;
const SIZE_Y: i16 = 240;

const CONTROL_UP: Key = Key::Up;
const CONTROL_RIGHT: Key = Key::Right;
const CONTROL_DOWN: Key = Key::Down;
const CONTROL_LEFT: Key = Key::Left;
const CONTROL_A: Key = Key::LCtrl;
const CONTROL_B: Key = Key::LAlt;
const CONTROL_X: Key = Key::Space;
const CONTROL_Y: Key = Key::LShift;
const CONTROL_TRIGGER_LEFT: Key = Key::Tab;
const CONTROL_TRIGGER_RIGHT: Key = Key::Backspace;
const CONTROL_SELECT: Key = Key::Escape;
const CONTROL_START: Key = Key::Return;

fn main() {
    sdl::init(&[sdl::InitFlag::Video]);
    sdl::wm::set_caption("rust-sdl demo - video", "rust-sdl");

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

    let mut mode = 0;
    let mut pressed_keys = HashMap::new();
    let mut frame: i16 = 0;

    'main: loop {
        'event: loop {
            match sdl::event::poll_event() {
                Event::Quit => break 'main,
                Event::None => break 'event,
                // Key(_, false, _) means key up, true key down
                Event::Key(CONTROL_SELECT, false, _, _) => break 'main,
                Event::Key(CONTROL_START, false, _, _) => mode += 1,
                Event::Key(k, pressed, _, _) => drop(pressed_keys.insert(k as isize, pressed)),
                _ => {}
            }
        }

        match mode % 3 {
            0 => draw_colors(&screen, frame),
            1 => draw_controls(&screen, &pressed_keys),
            2 => draw_alternating(&screen, frame),
            _ => panic!("bad mode"),
        }

        screen.flip();
        frame += 1;
    }

    sdl::quit();
}

fn draw_colors(screen: &Surface, frame: i16) {
    for i in 0..SIZE_X {
        for j in 0..SIZE_Y {
            screen.fill_rect(
                Some(sdl::Rect {
                    x: i,
                    y: j,
                    w: 1,
                    h: 1,
                }),
                Color::RGB(
                    (3 * i + 2 * j - frame) as u8,
                    (4 * i - 3 * j + 2 * frame) as u8,
                    (-5 * i + j + frame) as u8,
                ),
            );
        }
    }
}

fn draw_alternating(screen: &Surface, frame: i16) {
    for i in 0..SIZE_X {
        for j in 0..SIZE_Y {
            let v = if (i + j + frame) % 2 == 0 { 255 } else { 0 };
            screen.fill_rect(
                Some(sdl::Rect {
                    x: i,
                    y: j,
                    w: 1,
                    h: 1,
                }),
                Color::RGB(v, v, v),
            );
        }
    }
}

fn draw_controls(screen: &Surface, pressed_keys: &HashMap<isize, bool>) {
    screen.fill_rect(
        Some(sdl::Rect {
            x: 0,
            y: 0,
            w: SIZE_X as u16,
            h: SIZE_Y as u16,
        }),
        sdl::video::Color::RGB(0, 0, 0),
    );
    for (k, pressed) in pressed_keys {
        if *pressed {
            let (x, y) = if *k == CONTROL_UP as isize {
                (1, 2)
            } else if *k == CONTROL_LEFT as isize {
                (0, 3)
            } else if *k == CONTROL_DOWN as isize {
                (1, 4)
            } else if *k == CONTROL_RIGHT as isize {
                (2, 3)
            } else if *k == CONTROL_TRIGGER_LEFT as isize {
                (1, 0)
            } else if *k == CONTROL_TRIGGER_RIGHT as isize {
                (5, 0)
            } else if *k == CONTROL_A as isize {
                (6, 3)
            } else if *k == CONTROL_B as isize {
                (5, 4)
            } else if *k == CONTROL_X as isize {
                (5, 2)
            } else if *k == CONTROL_Y as isize {
                (4, 3)
            } else if *k == CONTROL_SELECT as isize {
                (2, 6)
            } else if *k == CONTROL_START as isize {
                (4, 6)
            } else {
                panic!("unknown key");
            };
            screen.fill_rect(
                Some(sdl::Rect {
                    x: x * (SIZE_X / 7),
                    y: y * (SIZE_Y / 7),
                    w: (SIZE_X / 7) as u16,
                    h: (SIZE_Y / 7) as u16,
                }),
                sdl::video::Color::RGB(255, 255, 255),
            );
        }
    }
}

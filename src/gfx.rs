use sdl::video::{SurfaceFlag, VideoFlag};

pub fn init() -> (sdl::video::Surface, sdl::video::VideoInfo) {
    sdl::init(&[sdl::InitFlag::Video]);

    let best = sdl::video::get_video_info();

    let screen = match sdl::video::set_video_mode(
        best.width,
        best.height,
        best.format.bpp as isize,
        &[SurfaceFlag::SWSurface],
        &[VideoFlag::DoubleBuf],
    ) {
        Ok(screen) => screen,
        Err(err) => panic!("failed to set video mode: {}", err),
    };

    (screen, sdl::video::get_video_info())
}

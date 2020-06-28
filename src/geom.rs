use sdl::video::VideoInfo;

pub struct Painter<'a> {
    pub video_info: &'a VideoInfo,
    pub pixels: std::boxed::Box<&'a mut [u8]>,
}

impl<'a> Painter<'a> {
    #[allow(clippy::many_single_char_names)]
    #[allow(clippy::too_many_arguments)]
    pub fn draw_line(&mut self, x0: isize, y0: isize, x1: isize, y1: isize, r: u8, g: u8, b: u8) {
        if x0 == x1 {
            for y in std::cmp::min(y0, y1)..=std::cmp::max(y0, y1) {
                self.put_pixel(x0, y, r, g, b);
            }
        }
        if y0 == y1 {
            for x in std::cmp::min(x0, x1)..=std::cmp::max(x0, x1) {
                self.put_pixel(x, y0, r, g, b);
            }
        }
        let minx = std::cmp::min(x0, x1);
        let maxx = std::cmp::max(x0, x1);
        let miny = std::cmp::min(y0, y1);
        let maxy = std::cmp::max(y0, y1);
        if maxx - minx > maxy - miny {
            for x in minx..=maxx {
                let f = ((x - x0) as f32) / (x1 - x0) as f32;
                let y = y0 as f32 * (1.0 - f) + y1 as f32 * f;
                self.put_pixel(x, y as isize, r, g, b);
            }
        } else {
            for y in miny..=maxy {
                let f = ((y - y0) as f32) / (y1 - y0) as f32;
                let x = x0 as f32 * (1.0 - f) + x1 as f32 * f;
                self.put_pixel(x as isize, y, r, g, b);
            }
        }
    }

    #[allow(clippy::many_single_char_names)]
    pub fn put_pixel(&mut self, x: isize, y: isize, r: u8, g: u8, b: u8) {
        let mut pixel_pos =
            ((y * self.video_info.width + x) * (self.video_info.format.bpp >> 3) as isize) as usize;

        let mut coloru32: u32 = ((r as u32 >> self.video_info.format.r_loss)
            << self.video_info.format.r_shift)
            + ((g as u32 >> self.video_info.format.g_loss) << self.video_info.format.g_shift)
            + ((b as u32 >> self.video_info.format.b_loss) << self.video_info.format.b_shift);

        for _ in 0..(self.video_info.format.bpp >> 3) {
            self.pixels[pixel_pos as usize] = (coloru32 % 256) as u8;
            pixel_pos += 1;
            coloru32 >>= 8;
        }
    }
}

pub fn get_sin_cos_lut() -> [(f32, f32); 256] {
    let mut sin_cos_lut = [(0.0 as f32, 0.0 as f32); 256];
    std::ops::Range { start: 0, end: 256 }
        .map(|b| b as f32 * 2.0 * std::f32::consts::PI / 256.0)
        .map(f32::sin_cos)
        .zip(sin_cos_lut.iter_mut())
        .for_each(|(sc, a)| *a = sc);
    sin_cos_lut
}

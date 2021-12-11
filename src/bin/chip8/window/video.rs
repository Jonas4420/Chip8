use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;

use super::error::WindowError;

pub struct VideoEngine {
    canvas: WindowCanvas,
    width: usize,
    height: usize,
    bg: Color,
    fg: Color,
    buffer: Vec<bool>,
}

impl VideoEngine {
    pub fn new(
        sdl: &Sdl,
        title: &str,
        (width, height): (usize, usize),
        scale: u8,
        bg: Color,
        fg: Color,
    ) -> Result<Self, WindowError> {
        let video = sdl.video()?;

        let window = video
            .window(title, Self::scale_dim(width, scale)?, Self::scale_dim(height, scale)?)
            .position_centered()
            .build()?;

        let mut canvas = window.into_canvas().build()?;
        canvas.set_scale(scale.into(), scale.into())?;

        // TODO: safe multiplication
        let video_size = (width * height) as usize;

        Ok(Self {
            canvas,
            width,
            height,
            bg,
            fg,
            buffer: vec![false; video_size],
        })
    }

    pub fn scale_dim(x: usize, scale: u8) -> Result<u32, WindowError> {
        // TODO: safe
        Ok((x * (scale as usize)) as u32)
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(self.bg);
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn render(&mut self) -> Result<(), WindowError> {
        self.canvas.set_draw_color(self.bg);
        self.canvas.clear();

        self.canvas.set_draw_color(self.fg);

        for y in 0..self.height {
            for x in 0..self.width {
                if self.buffer[(y * self.width) + x] {
                    self.canvas.draw_point((x as i32, y as i32))?;
                }
            }
        }

        self.canvas.present();

        Ok(())
    }

    pub fn get_buffer(&mut self) -> &mut [bool] {
        &mut self.buffer
    }
}

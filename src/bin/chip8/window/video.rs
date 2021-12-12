use std::time::{Duration, Instant};

use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;

use crate::error;

pub struct VideoEngine {
    canvas: WindowCanvas,
    buffer: Vec<bool>,
    width: usize,
    height: usize,
    fps: Duration,
    last: Option<Instant>,
    bg: Color,
    fg: Color,
}

impl VideoEngine {
    pub fn new(
        sdl: &Sdl,
        title: &str,
        (width, height): (usize, usize),
        scale: u8,
        fps: u32,
        bg: Color,
        fg: Color,
    ) -> Result<Self, error::Error> {
        let video = sdl.video()?;

        let mut canvas = video
            .window(title, Self::scale_size(width, scale)?, Self::scale_size(height, scale)?)
            .position_centered()
            .build()?
            .into_canvas()
            .build()?;

        canvas.set_scale(scale.into(), scale.into())?;

        let buffer_sz = width
            .checked_mul(height)
            .ok_or(error::Error::InvalidScreenSize((width, height)))?;

        let fps = Duration::from_secs(1)
            .checked_div(fps)
            .ok_or(error::Error::InvalidFramerate(fps))?;

        Ok(Self {
            canvas,
            buffer: vec![false; buffer_sz],
            width,
            height,
            fps,
            last: None,
            bg,
            fg,
        })
    }

    pub fn render(&mut self, now: Instant) -> Result<(), error::Error> {
        let render = match self.last {
            Some(prev) => now.duration_since(prev) >= self.fps,
            None => true,
        };

        if render {
            self.update()?;
            self.last = Some(now);
        }

        Ok(())
    }

    pub fn get_memory(&mut self) -> &mut [bool] {
        &mut self.buffer
    }

    fn update(&mut self) -> Result<(), error::Error> {
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

    fn scale_size(x: usize, scale: u8) -> Result<u32, error::Error> {
        x.checked_mul(scale.into())
            .and_then(|x_scaled| x_scaled.try_into().ok())
            .ok_or(error::Error::InvalidScale(x, scale))
    }
}

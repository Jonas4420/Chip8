use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::error;
use crate::options;

use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

const WINDOW_TITLE: &str = "CHIP-8 Emulator";
const WINDOW_SCALE: u8 = 8;
const WINDOW_FRAMERATE: u32 = 30;
const WINDOW_BACKGROUND: Color = Color::RGB(0x28, 0x28, 0x28);
const WINDOW_FOREGROUND: Color = Color::RGB(0xd5, 0xc4, 0xa1);

pub struct WindowBuilder {
    window: Window,
}

pub struct Window {
    canvas: WindowCanvas,
    events: EventPump,
    audio: AudioDevice<SquareWave>,
    width: usize,
    height: usize,
    bg: Color,
    fg: Color,
    frame_period: Duration,
    frame_last: Option<Instant>,
    is_open: bool,
    is_playing: bool,
    mappings: HashMap<Scancode, usize>,
    video: Vec<bool>,
    keyboard: Vec<bool>,
}

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl WindowBuilder {
    pub fn new(
        (width, height): (usize, usize),
        mappings: &[char],
        options: &options::Options,
    ) -> Result<Self, error::Error> {
        let sdl = sdl2::init().map_err(error::Error::SdlStr)?;

        let video = sdl.video().map_err(error::Error::SdlStr)?;
        let audio = sdl.audio().map_err(error::Error::SdlStr)?;
        let events = sdl.event_pump().map_err(error::Error::SdlStr)?;

        let fps = options.fps.unwrap_or(WINDOW_FRAMERATE);
        // TODO
        let frame_period = Duration::from_secs(1).checked_div(fps).unwrap();

        let scale = options.scale.unwrap_or(WINDOW_SCALE);

        let window = video
            .window(
                WINDOW_TITLE,
                Self::scale_dim(width, scale)?,
                Self::scale_dim(height, scale)?,
            )
            .position_centered()
            .build()?;

        let mut canvas = window.into_canvas().build()?;
        canvas
            .set_scale(scale.into(), scale.into())
            .map_err(error::Error::SdlStr)?;

        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: None,
        };

        let device = audio
            .open_playback(None, &desired_spec, |spec| SquareWave {
                phase_inc: 440.0 / (spec.freq as f32),
                phase: 0.0,
                volume: 0.1,
            })
            .map_err(error::Error::SdlStr)?;

        let mappings = mappings
            .iter()
            .enumerate()
            .map(|(i, c)| get_scancode(*c).map(|key| (key, i)))
            .collect::<Result<HashMap<_, _>, _>>()?;

        // TODO: safe multiplication
        let video_size = width * height;

        let window = Window {
            canvas,
            events,
            audio: device,
            video: vec![false; video_size],
            width,
            height,
            bg: options.bg.unwrap_or(WINDOW_BACKGROUND),
            fg: options.fg.unwrap_or(WINDOW_FOREGROUND),
            frame_period,
            frame_last: None,
            is_open: false,
            is_playing: false,
            keyboard: vec![false; mappings.len()],
            mappings,
        };

        Ok(Self { window })
    }

    pub fn present(self) -> Window {
        let mut window = self.window;

        window.canvas.set_draw_color(window.bg);
        window.canvas.clear();
        window.canvas.present();

        for event in window.events.wait_iter() {
            if let Event::Window {
                win_event: WindowEvent::FocusGained { .. },
                ..
            } = event
            {
                break;
            }
        }

        window.is_open = true;

        window
    }

    pub fn scale_dim(x: usize, scale: u8) -> Result<u32, error::Error> {
        // TODO: safe
        Ok((x * (scale as usize)) as u32)
    }
}

impl Window {
    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn process_events(&mut self) -> Result<(), error::Error> {
        for event in self.events.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    self.is_open = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    self.is_open = false;
                }
                Event::KeyDown {
                    scancode: Some(key), ..
                } => {
                    if let Some(idx) = self.mappings.get(&key) {
                        self.keyboard[*idx] = true;
                    }
                }
                Event::KeyUp {
                    scancode: Some(key), ..
                } => {
                    if let Some(idx) = self.mappings.get(&key) {
                        self.keyboard[*idx] = false;
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub fn render(&mut self, sound_timer: u8) -> Result<(), error::Error> {
        let now = Instant::now();

        let render = match self.frame_last {
            Some(prev) => now.duration_since(prev) >= self.frame_period,
            None => true,
        };

        // Video
        if render {
            self.frame_last = Some(now);

            self.canvas.set_draw_color(self.bg);
            self.canvas.clear();

            self.canvas.set_draw_color(self.fg);

            for y in 0..self.height {
                for x in 0..self.width {
                    if self.video[(y * self.width) + x] {
                        self.canvas
                            .draw_point((x as i32, y as i32))
                            .map_err(error::Error::SdlStr)?;
                    }
                }
            }

            self.canvas.present();
        }

        // Audio
        if sound_timer != 0 {
            if !self.is_playing {
                self.audio.resume();
                self.is_playing = true;
            }
        } else {
            if self.is_playing {
                self.audio.pause();
                self.is_playing = false;
            }
        }

        Ok(())
    }

    pub fn get_io(&mut self) -> (&[bool], &mut [bool]) {
        (&self.keyboard, &mut self.video)
    }
}

fn get_scancode(c: char) -> Result<Scancode, error::Error> {
    match c.to_ascii_lowercase() {
        '1' => Ok(Scancode::Num1),
        '2' => Ok(Scancode::Num2),
        '3' => Ok(Scancode::Num3),
        '4' => Ok(Scancode::Num4),
        '5' => Ok(Scancode::Num5),
        '6' => Ok(Scancode::Num6),
        '7' => Ok(Scancode::Num7),
        '8' => Ok(Scancode::Num8),
        '9' => Ok(Scancode::Num9),
        '0' => Ok(Scancode::Num0),
        'a' => Ok(Scancode::A),
        'b' => Ok(Scancode::B),
        'c' => Ok(Scancode::C),
        'd' => Ok(Scancode::D),
        'e' => Ok(Scancode::E),
        'f' => Ok(Scancode::F),
        'g' => Ok(Scancode::G),
        'h' => Ok(Scancode::H),
        'i' => Ok(Scancode::I),
        'j' => Ok(Scancode::J),
        'k' => Ok(Scancode::K),
        'l' => Ok(Scancode::L),
        'm' => Ok(Scancode::M),
        'n' => Ok(Scancode::N),
        'o' => Ok(Scancode::O),
        'p' => Ok(Scancode::P),
        'q' => Ok(Scancode::Q),
        'r' => Ok(Scancode::R),
        's' => Ok(Scancode::S),
        't' => Ok(Scancode::T),
        'u' => Ok(Scancode::U),
        'v' => Ok(Scancode::V),
        'w' => Ok(Scancode::W),
        'x' => Ok(Scancode::X),
        'y' => Ok(Scancode::Y),
        'z' => Ok(Scancode::Z),
        _ => Err(error::Error::UnknownMapping(c)),
    }
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 { self.volume } else { -self.volume };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

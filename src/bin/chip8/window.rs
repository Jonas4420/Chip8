use std::time::{Duration, Instant};

use crate::options;
use error::WindowError;

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::EventPump;

mod audio;
mod error;
mod keyboard;
mod video;

const WINDOW_TITLE: &str = "CHIP-8 Emulator";
const WINDOW_SCALE: u8 = 8;
const WINDOW_FRAMERATE: u32 = 30;
const WINDOW_BACKGROUND: Color = Color::RGB(0x28, 0x28, 0x28);
const WINDOW_FOREGROUND: Color = Color::RGB(0xd5, 0xc4, 0xa1);

pub struct WindowBuilder {
    window: Window,
}

pub struct Window {
    video: video::VideoEngine,
    audio: audio::AudioEngine,
    keyboard: keyboard::KeyboardEngine,
    events: EventPump,
    frame_period: Duration,
    frame_last: Option<Instant>,
    is_open: bool,
}

impl WindowBuilder {
    pub fn new(dimensions: (usize, usize), keys: &[char], options: &options::Options) -> Result<Self, WindowError> {
        let sdl = sdl2::init()?;

        // Get options
        let fps = options.fps.unwrap_or(WINDOW_FRAMERATE);
        let scale = options.scale.unwrap_or(WINDOW_SCALE);
        let bg = options.bg.unwrap_or(WINDOW_BACKGROUND);
        let fg = options.fg.unwrap_or(WINDOW_FOREGROUND);

        // Initialize engines
        let video = video::VideoEngine::new(&sdl, WINDOW_TITLE, dimensions, scale, bg, fg)?;
        let audio = audio::AudioEngine::new(&sdl)?;
        let keyboard = keyboard::KeyboardEngine::new(keys)?;

        let events = sdl.event_pump()?;
        let frame_period = Duration::from_secs(1).checked_div(fps).unwrap();

        let window = Window {
            video,
            audio,
            keyboard,
            events,
            frame_period,
            frame_last: None,
            is_open: false,
        };

        Ok(Self { window })
    }

    pub fn present(self) -> Window {
        let mut window = self.window;

        window.video.clear();

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
}

impl Window {
    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn process_events(&mut self) -> Result<(), WindowError> {
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
                    self.keyboard.key_down(&key);
                }
                Event::KeyUp {
                    scancode: Some(key), ..
                } => {
                    self.keyboard.key_up(&key);
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub fn render(&mut self, sound_timer: u8) -> Result<(), WindowError> {
        let now = Instant::now();

        let render = match self.frame_last {
            Some(prev) => now.duration_since(prev) >= self.frame_period,
            None => true,
        };

        if render {
            self.video.render()?;
            self.frame_last = Some(now);
        }

        if sound_timer > 0 {
            self.audio.play();
        } else {
            self.audio.pause();
        }

        Ok(())
    }

    pub fn get_io(&mut self) -> (&[bool], &mut [bool]) {
        (self.keyboard.get_buffer(), self.video.get_buffer())
    }
}

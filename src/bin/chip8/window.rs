use std::time::Instant;

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::EventPump;

use crate::options;
use error::WindowError;

mod audio;
mod error;
mod keyboard;
mod video;

const WINDOW_TITLE: &str = "CHIP-8 Emulator";
const WINDOW_SCALE: u8 = 8;
const WINDOW_FRAMERATE: u32 = 30;
const WINDOW_BACKGROUND: Color = Color::RGB(0x28, 0x28, 0x28);
const WINDOW_FOREGROUND: Color = Color::RGB(0xd5, 0xc4, 0xa1);

pub struct Window {
    video: video::VideoEngine,
    audio: audio::AudioEngine,
    keyboard: keyboard::KeyboardEngine,
    events: EventPump,
}

impl Window {
    pub fn new(dimensions: (usize, usize), keys: &[char], options: &options::Options) -> Result<Self, WindowError> {
        let sdl = sdl2::init()?;

        // Get options
        let fps = options.fps.unwrap_or(WINDOW_FRAMERATE);
        let scale = options.scale.unwrap_or(WINDOW_SCALE);
        let bg = options.bg.unwrap_or(WINDOW_BACKGROUND);
        let fg = options.fg.unwrap_or(WINDOW_FOREGROUND);

        // Initialize engines
        let video = video::VideoEngine::new(&sdl, WINDOW_TITLE, dimensions, scale, fps, bg, fg)?;
        let audio = audio::AudioEngine::new(&sdl)?;
        let keyboard = keyboard::KeyboardEngine::new(keys)?;
        let events = sdl.event_pump()?;

        Ok(Self {
            video,
            audio,
            keyboard,
            events,
        })
    }

    pub fn run<F>(&mut self, mut f: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnMut(&[bool], &mut [bool], &mut bool) -> Result<(), Box<dyn std::error::Error>>,
    {
        self.display()?;

        while self.process_events() {
            f(
                self.keyboard.get_memory(),
                self.video.get_memory(),
                self.audio.get_memory(),
            )?;

            self.audio.render()?;
            self.video.render(Instant::now())?;

            std::thread::sleep(std::time::Duration::from_millis(1));
        }

        Ok(())
    }

    fn display(&mut self) -> Result<(), WindowError> {
        self.video.render(Instant::now())?;

        for event in self.events.wait_iter() {
            if let Event::Window {
                win_event: WindowEvent::FocusGained { .. },
                ..
            } = event
            {
                break;
            }
        }

        Ok(())
    }

    fn process_events(&mut self) -> bool {
        for event in self.events.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    return false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return false;
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

        true
    }
}

use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use sdl2::Sdl;

use super::error::WindowError;

pub struct AudioEngine {
    audio: AudioDevice<SquareWave>,
    is_playing: bool,
}

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioEngine {
    pub fn new(sdl: &Sdl) -> Result<Self, WindowError> {
        let audio = sdl.audio()?;

        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: None,
        };

        let device = audio.open_playback(None, &desired_spec, |spec| SquareWave {
            phase_inc: 440.0 / (spec.freq as f32),
            phase: 0.0,
            volume: 0.1,
        })?;

        Ok(AudioEngine {
            audio: device,
            is_playing: false,
        })
    }

    pub fn play(&mut self) {
        if !self.is_playing {
            self.audio.resume();
            self.is_playing = true;
        }
    }

    pub fn pause(&mut self) {
        if self.is_playing {
            self.audio.pause();
            self.is_playing = false;
        }
    }
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 { self.volume } else { -self.volume };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

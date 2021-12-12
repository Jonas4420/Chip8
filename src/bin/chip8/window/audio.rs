use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired, AudioStatus};
use sdl2::Sdl;

use super::error::WindowError;

pub const NOTE_PITCH: f32 = 440.0;

pub struct AudioEngine {
    audio: AudioDevice<SquareWave>,
    beep: bool,
}

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioEngine {
    pub fn new(sdl: &Sdl) -> Result<Self, WindowError> {
        let audio = sdl.audio()?;

        let spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: None,
        };

        let audio = audio.open_playback(None, &spec, |spec| SquareWave {
            phase_inc: NOTE_PITCH / (spec.freq as f32),
            phase: 0.0,
            volume: 0.1,
        })?;

        Ok(AudioEngine { audio, beep: false })
    }

    pub fn render(&mut self) -> Result<(), WindowError> {
        if self.beep {
            self.play();
        } else {
            self.pause();
        }

        Ok(())
    }

    pub fn get_memory(&mut self) -> &mut bool {
        &mut self.beep
    }

    fn play(&mut self) {
        if let AudioStatus::Paused = self.audio.status() {
            self.audio.resume();
        }
    }

    fn pause(&mut self) {
        if let AudioStatus::Playing = self.audio.status() {
            self.audio.pause();
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

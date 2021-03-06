use sdl2::audio;
use sdl2::Sdl;

use crate::error;

pub const NOTE_PITCH: f32 = 440.0;

pub struct AudioEngine {
    audio: audio::AudioDevice<SquareWave>,
    beep: bool,
}

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioEngine {
    pub fn new(sdl: &Sdl) -> Result<Self, error::Error> {
        let audio = sdl.audio()?;

        let spec = audio::AudioSpecDesired {
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

    pub fn render(&mut self) -> Result<(), error::Error> {
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
        if let audio::AudioStatus::Paused = self.audio.status() {
            self.audio.resume();
        }
    }

    fn pause(&mut self) {
        if let audio::AudioStatus::Playing = self.audio.status() {
            self.audio.pause();
        }
    }
}

impl audio::AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = self.volume * if self.phase <= 0.5 { 1.0 } else { -1.0 };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

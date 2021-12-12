use std::collections::HashMap;

use sdl2::keyboard::Scancode;

use super::error::WindowError;

pub struct KeyboardEngine {
    map: HashMap<Scancode, usize>,
    buffer: Vec<bool>,
}

impl KeyboardEngine {
    pub fn new(keys: &[char]) -> Result<Self, WindowError> {
        Ok(Self {
            map: keys
                .iter()
                .enumerate()
                .map(|(i, c)| Self::get_scancode(*c).map(|key| (key, i)))
                .collect::<Result<_, _>>()?,
            buffer: vec![false; keys.len()],
        })
    }

    pub fn key_down(&mut self, key: &Scancode) {
        if let Some(idx) = self.map.get(key) {
            self.buffer[*idx] = true;
        }
    }

    pub fn key_up(&mut self, key: &Scancode) {
        if let Some(idx) = self.map.get(key) {
            self.buffer[*idx] = false;
        }
    }

    pub fn get_memory(&self) -> &[bool] {
        &self.buffer
    }

    fn get_scancode(c: char) -> Result<Scancode, WindowError> {
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
            _ => Err(WindowError::UnknownMapping(c)),
        }
    }
}

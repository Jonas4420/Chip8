#[derive(Debug, Default)]
pub struct Timer {
    value: u8,
}

impl Timer {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn clock(&mut self) {
        self.set(self.value.saturating_sub(1));
    }

    pub fn get(&self) -> u8 {
        self.value
    }

    pub fn set(&mut self, value: u8) {
        self.value = value;
    }
}

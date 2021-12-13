use std::time;

use crate::error::Error;

#[derive(Debug)]
pub struct Clock {
    freq: time::Duration,
    last: Option<time::Instant>,
}

impl Clock {
    pub fn new(freq: time::Duration) -> Self {
        Self { freq, last: None }
    }

    pub fn tick<F>(&mut self, now: time::Instant, mut f: F) -> Result<(), Error>
    where
        F: FnMut() -> Result<(), Error>,
    {
        self.last = if let Some(mut last) = self.last {
            while now.saturating_duration_since(last) >= self.freq {
                f()?;
                last += self.freq;
            }
            Some(last)
        } else {
            f()?;
            Some(now)
        };

        Ok(())
    }
}

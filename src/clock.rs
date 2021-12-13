use std::time;

#[derive(Debug)]
pub struct Clock {
    freq: time::Duration,
    last: Option<time::Instant>,
}

impl Clock {
    pub fn new(freq: time::Duration) -> Self {
        Self { freq, last: None }
    }

    pub fn tick<F>(&mut self, now: time::Instant, mut f: F)
    where
        F: FnMut() -> (),
    {
        if let Some(mut last) = self.last {
            while now.saturating_duration_since(last) >= self.freq {
                f();
                last += self.freq;
            }
            self.last = Some(last);
        } else {
            f();
            self.last = Some(now);
        }
    }
}

pub struct Phase {
    phase: f64,
    delta: f64,
}

impl Phase {
    pub fn new(delta: f64) -> Option<Self> {
        if !delta.is_normal() || delta <= 0. {
            None
        } else {
            Some(Self { phase: 0., delta })
        }
    }

    pub fn with_freq(freq: f64, sample_rate: f64) -> Option<Self> {
        Self::new(freq / sample_rate)
    }
}

impl crate::sound::Sound for Phase {
    fn next(&mut self) -> Option<f64> {
        self.phase += self.delta;
        // if self.phase > 1. {
        //     self.phase -= 2.;
        // }
        Some(self.phase)
    }

    fn reset(&mut self) {
        self.phase = 0.;
    }

    fn duration(&self) -> f64 {
        f64::INFINITY
    }
}

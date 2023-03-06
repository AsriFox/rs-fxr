pub struct Phase {
    phase: f64,
    delta: f64,
}

impl Phase {
    pub fn new(delta: f64) -> Option<Self> {
        if delta.is_subnormal() || delta <= 0. { None }
        else { Some(Self { phase: 0., delta }) }
    }
    
    pub fn with_freq(freq: f64, sample_rate: f64) -> Option<Self> {
        Self::new(freq / sample_rate)
    }
}

impl Iterator for Phase {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        self.phase += self.delta;
        // if self.phase > 1. {
        //     self.phase -= 2.;
        // }
        Some(self.phase)
    }
}
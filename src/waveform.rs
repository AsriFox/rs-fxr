use crate::sound::Sound;
use rand::{
    distributions::{DistIter, Uniform},
    prelude::*,
    rngs::OsRng,
};
use std::f64::consts::PI;

pub struct Sine<S: Sound> {
    phase: S,
}
impl<S: Sound> Sine<S> {
    pub fn new(phase: S) -> Self {
        Self { phase }
    }
}
impl<S: Sound> Sound for Sine<S> {
    fn next(&mut self) -> Option<f64> {
        Some((self.phase.next()? * PI).sin())
    }
    fn reset(&mut self) {
        self.phase.reset();
    }
    fn duration(&self) -> f64 {
        self.phase.duration()
    }
}

pub struct Triangle<S: Sound> {
    phase: S,
}
impl<S: Sound> Triangle<S> {
    pub fn new(phase: S) -> Self {
        Self { phase }
    }
}
impl<S: Sound> Sound for Triangle<S> {
    fn next(&mut self) -> Option<f64> {
        let v = self.phase.next()?.fract();
        Some(if v < 0.25 {
            4. * v
        } else if v < 0.75 {
            2. - 4. * v
        } else {
            -4. + 4. * v
        })
    }
    fn reset(&mut self) {
        self.phase.reset();
    }
    fn duration(&self) -> f64 {
        self.phase.duration()
    }
}

pub struct Sawtooth<S: Sound> {
    phase: S,
}
impl<S: Sound> Sawtooth<S> {
    pub fn new(phase: S) -> Self {
        Self { phase }
    }
}
impl<S: Sound> Sound for Sawtooth<S> {
    fn next(&mut self) -> Option<f64> {
        Some((self.phase.next()? * 2.).fract() * 2. - 1.)
    }
    fn reset(&mut self) {
        self.phase.reset();
    }
    fn duration(&self) -> f64 {
        self.phase.duration()
    }
}

pub struct Breaker<S: Sound> {
    phase: S,
}
impl<S: Sound> Breaker<S> {
    pub fn new(phase: S) -> Self {
        Self { phase }
    }
}
impl<S: Sound> Sound for Breaker<S> {
    fn next(&mut self) -> Option<f64> {
        const BREAKER_OFFSET: f64 = 0.86602540378443864676372317075294; // f64::sqrt(0.75);
        let v = (self.phase.next()? + BREAKER_OFFSET).fract();
        Some(-1. + 2. * (1. - 2. * v * v).abs())
    }
    fn reset(&mut self) {
        self.phase.reset();
    }
    fn duration(&self) -> f64 {
        self.phase.duration()
    }
}

pub struct Tangent<S: Sound> {
    phase: S,
    cutoff: f64,
}
impl<S: Sound> Tangent<S> {
    pub fn new(phase: S, cutoff: f64) -> Option<Self> {
        if !cutoff.is_normal() || cutoff <= 0. {
            None
        } else {
            Some(Self { phase, cutoff })
        }
    }
    pub fn default(phase: S) -> Self {
        Self {
            phase,
            cutoff: 0.15,
        }
    }
}
impl<S: Sound> Sound for Tangent<S> {
    fn next(&mut self) -> Option<f64> {
        Some(((self.phase.next()? * PI).tan() / self.cutoff).clamp(-1., 1.))
    }
    fn reset(&mut self) {
        self.phase.reset();
    }
    fn duration(&self) -> f64 {
        self.phase.duration()
    }
}

pub struct Square<S: Sound> {
    phase: S,
    square_duty: f64,
}
impl<S: Sound> Square<S> {
    pub fn new(phase: S, square_duty: f64) -> Option<Self> {
        if !square_duty.is_normal() || square_duty <= 0. || square_duty >= 1. {
            None
        } else {
            Some(Self { phase, square_duty })
        }
    }
    pub fn default(phase: S) -> Self {
        Self {
            phase,
            square_duty: 0.5,
        }
    }
}
impl<S: Sound> Sound for Square<S> {
    fn next(&mut self) -> Option<f64> {
        let v = self.phase.next()?.fract();
        Some(if v < self.square_duty { 1. } else { -1. })
    }
    fn reset(&mut self) {
        self.phase.reset();
    }
    fn duration(&self) -> f64 {
        self.phase.duration()
    }
}

pub struct WhiteNoise<S: Sound> {
    // interpolated
    rng: DistIter<Uniform<f64>, OsRng, f64>,
    phase: S,
    prev_phase: f64,
    prev_random: f64,
    curr_random: f64,
    // interpolate: bool,
}
impl<S: Sound> WhiteNoise<S> {
    pub fn new(phase: S) -> Self {
        Self {
            rng: new_random(),
            phase,
            prev_phase: 0.,
            prev_random: 0.,
            curr_random: 0.,
        }
    }
}
impl<S: Sound> Sound for WhiteNoise<S> {
    fn next(&mut self) -> Option<f64> {
        let p = (self.phase.next()? * 2.).fract();
        if p < self.prev_phase {
            self.prev_random = self.curr_random;
            self.curr_random = self.rng.next()?;
        }
        self.prev_phase = p;
        Some(lerp(self.prev_random, self.curr_random, p))
    }
    fn reset(&mut self) {
        self.rng = new_random();
        self.phase.reset();
    }
    fn duration(&self) -> f64 {
        self.phase.duration()
    }
}

pub struct PinkNoise<S: Sound> {
    // interpolated
    rng: DistIter<Uniform<f64>, OsRng, f64>,
    phase: S,
    prev_phase: f64,
    prev_random: f64,
    curr_random: f64,
    b: [f64; 7],
    // interpolate: bool,
}
impl<S: Sound> PinkNoise<S> {
    pub fn new(phase: S) -> Self {
        Self {
            rng: new_random(),
            phase,
            prev_phase: 0.,
            prev_random: 0.,
            curr_random: 0.,
            b: [0.; 7],
        }
    }
}
impl<S: Sound> Sound for PinkNoise<S> {
    fn next(&mut self) -> Option<f64> {
        let p = (self.phase.next()? * 2.).fract();
        if p < self.prev_phase {
            self.prev_random = self.curr_random;
            let white = self.rng.next()?;
            self.b[0] = 0.99886 * self.b[0] + white * 0.0555179;
            self.b[1] = 0.99332 * self.b[1] + white * 0.0750759;
            self.b[2] = 0.96900 * self.b[2] + white * 0.1538520;
            self.b[3] = 0.86650 * self.b[3] + white * 0.3104856;
            self.b[4] = 0.55000 * self.b[4] + white * 0.5329522;
            self.b[5] = -0.7616 * self.b[5] + white * 0.0168980;
            self.curr_random = (self.b[0]
                + self.b[1]
                + self.b[2]
                + self.b[3]
                + self.b[4]
                + self.b[5]
                + self.b[6]
                + white * 0.5362)
                / 7.;
            self.b[6] = white * 0.115926;
        }
        self.prev_phase = p;
        Some(lerp(self.prev_random, self.curr_random, p))
    }
    fn reset(&mut self) {
        self.rng = new_random();
        self.phase.reset();
    }
    fn duration(&self) -> f64 {
        self.phase.duration()
    }
}

pub struct BrownNoise<S: Sound> {
    // interpolated
    rng: DistIter<Uniform<f64>, OsRng, f64>,
    phase: S,
    prev_phase: f64,
    prev_random: f64,
    curr_random: f64,
    rolloff: f64,
    // interpolate: bool,
}
impl<S: Sound> BrownNoise<S> {
    pub fn new(phase: S, rolloff: f64) -> Option<Self> {
        if !rolloff.is_normal() || rolloff <= 0. || rolloff >= 1. {
            None
        } else {
            Some(Self {
                rng: OsRng.sample_iter(Uniform::new(-1., 1.)),
                phase,
                prev_phase: 0.,
                prev_random: 0.,
                curr_random: 0.,
                rolloff,
            })
        }
    }
    pub fn default(phase: S) -> Self {
        Self {
            rng: new_random(),
            phase,
            prev_phase: 0.,
            prev_random: 0.,
            curr_random: 0.,
            rolloff: 0.1,
        }
    }
}
impl<S: Sound> Sound for BrownNoise<S> {
    fn next(&mut self) -> Option<f64> {
        let p = (self.phase.next()? * 2.).fract();
        if p < self.prev_phase {
            self.prev_random = self.curr_random;
            self.curr_random = (self.curr_random + self.rolloff * self.rng.next()?).clamp(-1., 1.);
        }
        self.prev_phase = p;
        Some(lerp(self.prev_random, self.curr_random, p))
    }
    fn reset(&mut self) {
        self.rng = new_random();
        self.phase.reset();
    }
    fn duration(&self) -> f64 {
        self.phase.duration()
    }
}

#[inline]
fn new_random() -> DistIter<Uniform<f64>, OsRng, f64> {
    OsRng.sample_iter(rand::distributions::Uniform::new(-1., 1.))
}

#[inline]
fn lerp(prev: f64, curr: f64, p: f64) -> f64 {
    (1. - p) * prev + p * curr
}

pub trait IntoWaveform<S: Sound> {
    fn sine(self) -> Sine<S>;
    fn triangle(self) -> Triangle<S>;
    fn sawtooth(self) -> Sawtooth<S>;
    fn square(self) -> Square<S>;
    fn tangent(self) -> Tangent<S>;
    fn breaker(self) -> Breaker<S>;
    fn white_noise(self) -> WhiteNoise<S>;
    fn pink_noise(self) -> PinkNoise<S>;
    fn brown_noise(self) -> BrownNoise<S>;
    fn tangent_custom(self, cutoff: f64) -> Option<Tangent<S>>;
    fn square_custom(self, square_duty: f64) -> Option<Square<S>>;
    fn brown_noise_custom(self, rolloff: f64) -> Option<BrownNoise<S>>;
}

impl<S: Sound> IntoWaveform<S> for S {
    fn sine(self) -> Sine<S> {
        Sine::new(self)
    }
    fn triangle(self) -> Triangle<S> {
        Triangle::new(self)
    }
    fn sawtooth(self) -> Sawtooth<S> {
        Sawtooth::new(self)
    }
    fn square(self) -> Square<S> {
        Square::default(self)
    }
    fn tangent(self) -> Tangent<S> {
        Tangent::default(self)
    }
    fn breaker(self) -> Breaker<S> {
        Breaker::new(self)
    }
    fn white_noise(self) -> WhiteNoise<S> {
        WhiteNoise::new(self)
    }
    fn pink_noise(self) -> PinkNoise<S> {
        PinkNoise::new(self)
    }
    fn brown_noise(self) -> BrownNoise<S> {
        BrownNoise::default(self)
    }
    fn tangent_custom(self, cutoff: f64) -> Option<Tangent<S>> {
        Tangent::new(self, cutoff)
    }
    fn square_custom(self, square_duty: f64) -> Option<Square<S>> {
        Square::new(self, square_duty)
    }
    fn brown_noise_custom(self, rolloff: f64) -> Option<BrownNoise<S>> {
        BrownNoise::new(self, rolloff)
    }
}

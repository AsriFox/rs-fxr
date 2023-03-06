use rand::{prelude::*, distributions::{DistIter, Uniform}, rngs::OsRng};

pub enum Waveform<I: Iterator<Item = f64>> {
    Sine { phase: I },
    Triangle { phase: I },
    Sawtooth { phase: I },
    Tangent { 
        phase: I,
        cutoff: f64,
    },
    Breaker { phase: I },
    Square { 
        phase: I, 
        square_duty: f64,
    },
    WhiteNoise { // interpolated
        rng: DistIter<Uniform<f64>, OsRng, f64>,
        phase: I, 
        prev_phase: f64,
        prev_random: f64,
        curr_random: f64,
        // interpolate: bool,
    },
    PinkNoise { // interpolated
        rng: DistIter<Uniform<f64>, OsRng, f64>,
        phase: I, 
        prev_phase: f64,
        prev_random: f64,
        curr_random: f64,
        b: [f64; 7],
        // interpolate: bool,
    },
    BrownNoise { // interpolated
        rng: DistIter<Uniform<f64>, OsRng, f64>,
        phase: I, 
        prev_phase: f64,
        prev_random: f64,
        curr_random: f64,
        // interpolate: bool,
    },
}

impl<I: Iterator<Item = f64>> Iterator for Waveform<I> {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        use std::f64::consts::PI;
        Some(match self {
            Self::Sine { phase } => { 
                (phase.next()? * PI).sin()
            }
            Self::Tangent { phase, cutoff } => {
                ((phase.next()? * PI).tan() / *cutoff).clamp(-1., 1.)
            }
            Self::Triangle { phase } => {
                let v = phase.next()?.fract();
                if v < 0.25 { 4. * v }
                else if v < 0.75 { 2. - 4. * v }
                else { -4. + 4. * v }
            }
            Self::Sawtooth { phase } => {
                (phase.next()? * 2.).fract() * 2. - 1.
            }
            Self::Breaker { phase } => {
                const BREAKER_OFFSET: f64 = 0.86602540378443864676372317075294; // f64::sqrt(0.75);
                let v = (phase.next()? + BREAKER_OFFSET).fract();
                -1. + 2. * (1. - 2. * v * v).abs()
            }
            Self::Square { phase, square_duty } => {
                let v = phase.next()?.fract();
                if v < *square_duty { 1. } else { -1. }
            }
            Self::WhiteNoise { 
                rng, 
                phase, 
                prev_phase, 
                prev_random, 
                curr_random ,
            } => {
                let p = (phase.next()? * 2.).fract();
                if p < *prev_phase {
                    *prev_random = *curr_random;
                    *curr_random = rng.next()?;
                }
                *prev_phase = p;
                lerp(*prev_random, *curr_random, p)
            }
            Self::PinkNoise { 
                rng, 
                phase,
                prev_phase, 
                prev_random, 
                curr_random ,
                b,
            } => {
                let p = (phase.next()? * 2.).fract();
                if p < *prev_phase {
                    *prev_random = *curr_random;
                    let white = rng.next()?;
                    b[0] = 0.99886 * b[0] + white * 0.0555179;
                    b[1] = 0.99332 * b[1] + white * 0.0750759;
                    b[2] = 0.96900 * b[2] + white * 0.1538520;
                    b[3] = 0.86650 * b[3] + white * 0.3104856;
                    b[4] = 0.55000 * b[4] + white * 0.5329522;
                    b[5] = -0.7616 * b[5] + white * 0.0168980;
                    *curr_random = (b[0] + b[1] + b[2] + b[3] + b[4] + b[5] + b[6] + white * 0.5362) / 7.;
                    b[6] = white * 0.115926;
                }
                *prev_phase = p;
                lerp(*prev_random, *curr_random, p)
            }
            Self::BrownNoise { 
                rng, 
                phase,
                prev_phase, 
                prev_random, 
                curr_random ,
            } => {
                let p = (phase.next()? * 2.).fract();
                if p < *prev_phase {
                    *prev_random = *curr_random;
                    *curr_random = (*curr_random + 0.1 * rng.next()?).clamp(-1., 1.);
                }
                *prev_phase = p;
                lerp(*prev_random, *curr_random, p)
            }
        })
    }
}

fn lerp(prev: f64, curr: f64, p: f64) -> f64 {
    (1. - p) * prev + p * curr
}

impl<I: Iterator<Item = f64>> Waveform<I> {
    pub fn new_sine(phase: I) -> Self {
        Self::Sine { phase }
    }
    
    pub fn new_triangle(phase: I) -> Self {
        Self::Triangle { phase }
    }
    
    pub fn new_sawtooth(phase: I) -> Self {
        Self::Sawtooth { phase }
    }
    
    pub fn new_breaker(phase: I) -> Self {
        Self::Breaker { phase }
    }
    
    pub fn new_tangent(phase: I, cutoff: f64) -> Option<Self> {
        if cutoff.is_subnormal() || cutoff <= 0. { None }
        else { Some(Self::Tangent { phase, cutoff }) }
    }
    
    pub fn new_square(phase: I, square_duty: f64) -> Option<Self> {
        if square_duty < 0. || square_duty > 1. { None }
        else { Some(Self::Square { phase, square_duty }) }
    }
    
    pub fn new_white_noise(phase: I) -> Self {
        Self::WhiteNoise { 
            rng: OsRng.sample_iter(rand::distributions::Uniform::new(-1., 1.)), 
            phase, 
            prev_phase: 0., 
            prev_random: 0., 
            curr_random: 0.,
        }
    }
    
    pub fn new_pink_noise(phase: I) -> Self {
        Self::PinkNoise { 
            rng: OsRng.sample_iter(rand::distributions::Uniform::new(-1., 1.)), 
            phase, 
            prev_phase: 0., 
            prev_random: 0., 
            curr_random: 0.,
            b: [0.; 7],
        }
    }
    
    pub fn new_brown_noise(phase: I) -> Self {
        Self::BrownNoise { 
            rng: OsRng.sample_iter(rand::distributions::Uniform::new(-1., 1.)), 
            phase, 
            prev_phase: 0., 
            prev_random: 0., 
            curr_random: 0.,
        }
    }
}

pub trait IntoWaveform<I: Iterator<Item = f64>> {
    fn sine(self) -> Waveform<I>;
    fn triangle(self) -> Waveform<I>;
    fn sawtooth(self) -> Waveform<I>;
    fn breaker(self) -> Waveform<I>;
    fn tangent(self, cutoff: f64) -> Option<Waveform<I>>;
    fn square(self, square_duty: f64) -> Option<Waveform<I>>;
    fn white_noise(self) -> Waveform<I>;
    fn pink_noise(self) -> Waveform<I>;
    fn brown_noise(self) -> Waveform<I>;
}

impl<I: Iterator<Item = f64>> IntoWaveform<I> for I {
    fn sine(self) -> Waveform<I> {
        Waveform::new_sine(self)
    }
    fn triangle(self) -> Waveform<I> {
        Waveform::new_triangle(self)
    }
    fn sawtooth(self) -> Waveform<I> {
        Waveform::new_sawtooth(self)
    }
    fn breaker(self) -> Waveform<I> {
        Waveform::new_breaker(self)
    }
    fn tangent(self, cutoff: f64) -> Option<Waveform<I>> {
        Waveform::new_tangent(self, cutoff)
    }
    fn square(self, square_duty: f64) -> Option<Waveform<I>> {
        Waveform::new_square(self, square_duty)
    }
    fn white_noise(self) -> Waveform<I> {
        Waveform::new_white_noise(self)
    }
    fn pink_noise(self) -> Waveform<I> {
        Waveform::new_pink_noise(self)
    }
    fn brown_noise(self) -> Waveform<I> {
        Waveform::new_brown_noise(self)
    }
}
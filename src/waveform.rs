use crate::sound::Proc;
use rand::{
    distributions::{DistIter, Uniform},
    prelude::*,
    rngs::OsRng,
};

pub struct Sine {
    freq: f64,
}
impl Sine {
    pub fn new(freq: f64) -> Option<Self> {
        if !freq.is_normal() || freq <= 0. {
            None
        } else {
            Some(Self { freq })
        }
    }
}
impl Proc for Sine {
    fn value(&self, t: f64) -> f64 {
        (std::f64::consts::TAU * self.freq * t).sin()
    }
}

pub struct Triangle {
    freq: f64,
}
impl Triangle {
    pub fn new(freq: f64) -> Option<Self> {
        if !freq.is_normal() || freq <= 0. {
            None
        } else {
            Some(Self { freq })
        }
    }
}
impl Proc for Triangle {
    fn value(&self, t: f64) -> f64 {
        let v = (self.freq * t).fract();
        if v < 0.25 {
            4. * v
        } else if v < 0.75 {
            2. - 4. * v
        } else {
            -4. + 4. * v
        }
    }
}

pub struct Sawtooth {
    freq: f64,
}
impl Sawtooth {
    pub fn new(freq: f64) -> Option<Self> {
        if !freq.is_normal() || freq <= 0. {
            None
        } else {
            Some(Self { freq })
        }
    }
}
impl Proc for Sawtooth {
    fn value(&self, t: f64) -> f64 {
        (self.freq * t).fract() * 2. - 1.
    }
}

pub struct Breaker {
    freq: f64,
}
impl Breaker {
    pub fn new(freq: f64) -> Option<Self> {
        if !freq.is_normal() || freq <= 0. {
            None
        } else {
            Some(Self { freq })
        }
    }
}
impl Proc for Breaker {
    fn value(&self, t: f64) -> f64 {
        const BREAKER_OFFSET: f64 = 0.86602540378443864676372317075294; // f64::sqrt(0.75);
        let v = (self.freq * t + BREAKER_OFFSET).fract();
        -1. + 2. * (1. - 2. * v * v).abs()
    }
}

pub struct Tangent {
    freq: f64,
    cutoff: f64,
}
impl Tangent {
    pub fn new(freq: f64, cutoff: f64) -> Option<Self> {
        if !freq.is_normal() || freq <= 0. || !cutoff.is_normal() || cutoff <= 0. {
            None
        } else {
            Some(Self { freq, cutoff })
        }
    }
    pub fn default(freq: f64) -> Option<Self> {
        Self::new(freq, 0.15)
    }
}
impl Proc for Tangent {
    fn value(&self, t: f64) -> f64 {
        ((std::f64::consts::PI * self.freq * t).tan() / self.cutoff).clamp(-1., 1.)
    }
}

pub struct Square {
    freq: f64,
    square_duty: f64,
}
impl Square {
    pub fn new(freq: f64, square_duty: f64) -> Option<Self> {
        if !freq.is_normal()
            || freq <= 0.
            || !square_duty.is_normal()
            || square_duty <= 0.
            || square_duty >= 1.
        {
            None
        } else {
            Some(Self { freq, square_duty })
        }
    }
    pub fn default(freq: f64) -> Option<Self> {
        Self::new(freq, 0.5)
    }
}
impl Proc for Square {
    fn value(&self, t: f64) -> f64 {
        let v = (self.freq * t).fract();
        if v < self.square_duty {
            1.
        } else {
            -1.
        }
    }
}

pub struct WhiteNoise {
    // interpolated
    rng: DistIter<Uniform<f64>, OsRng, f64>,
    freq: f64,
    prev_phase: f64,
    prev_random: f64,
    curr_random: f64,
    // interpolate: bool,
}
impl WhiteNoise {
    pub fn new(freq: f64) -> Option<Self> {
        if !freq.is_normal() || freq <= 0. {
            None
        } else {
            Some(Self {
                rng: new_random(),
                freq,
                prev_phase: 0.,
                prev_random: 0.,
                curr_random: 0.,
            })
        }
    }
}
// impl Proc for WhiteNoise {
//     fn value(&self, t: f64) -> f64 {
//         let p = (TAU * self.freq * t * 2.).fract();
//         if p < self.prev_phase {
//             self.prev_random = self.curr_random;
//             self.curr_random = self.rng.next().unwrap_or(0.);
//         }
//         self.prev_phase = p;
//         lerp(self.prev_random, self.curr_random, p)
//     }
// }

pub struct PinkNoise {
    // interpolated
    rng: DistIter<Uniform<f64>, OsRng, f64>,
    freq: f64,
    prev_phase: f64,
    prev_random: f64,
    curr_random: f64,
    b: [f64; 7],
    // interpolate: bool,
}
impl PinkNoise {
    pub fn new(freq: f64) -> Option<Self> {
        if !freq.is_normal() || freq <= 0. {
            None
        } else {
            Some(Self {
                rng: new_random(),
                freq,
                prev_phase: 0.,
                prev_random: 0.,
                curr_random: 0.,
                b: [0.; 7],
            })
        }
    }
}
// impl Proc for PinkNoise {
//     fn value(&self, t: f64) -> f64 {
//         let p = (TAU * self.freq * t * 2.).fract();
//         if p < self.prev_phase {
//             self.prev_random = self.curr_random;
//             let white = self.rng.next().unwrap_or(0.);
//             self.b[0] = 0.99886 * self.b[0] + white * 0.0555179;
//             self.b[1] = 0.99332 * self.b[1] + white * 0.0750759;
//             self.b[2] = 0.96900 * self.b[2] + white * 0.1538520;
//             self.b[3] = 0.86650 * self.b[3] + white * 0.3104856;
//             self.b[4] = 0.55000 * self.b[4] + white * 0.5329522;
//             self.b[5] = -0.7616 * self.b[5] + white * 0.0168980;
//             self.curr_random = (self.b[0]
//                 + self.b[1]
//                 + self.b[2]
//                 + self.b[3]
//                 + self.b[4]
//                 + self.b[5]
//                 + self.b[6]
//                 + white * 0.5362)
//                 / 7.;
//             self.b[6] = white * 0.115926;
//         }
//         self.prev_phase = p;
//         lerp(self.prev_random, self.curr_random, p)
//     }
// }

pub struct BrownNoise {
    // interpolated
    rng: DistIter<Uniform<f64>, OsRng, f64>,
    freq: f64,
    prev_phase: f64,
    prev_random: f64,
    curr_random: f64,
    rolloff: f64,
    // interpolate: bool,
}
impl BrownNoise {
    pub fn new(freq: f64, rolloff: f64) -> Option<Self> {
        if !freq.is_normal() || freq <= 0. || !rolloff.is_normal() || rolloff <= 0. || rolloff >= 1.
        {
            None
        } else {
            Some(Self {
                rng: new_random(),
                freq,
                prev_phase: 0.,
                prev_random: 0.,
                curr_random: 0.,
                rolloff,
            })
        }
    }
    pub fn default(freq: f64) -> Option<Self> {
        Self::new(freq, 0.1)
    }
}
// impl Proc for BrownNoise {
//     fn value(&self, t: f64) -> f64 {
//         let p = (TAU * self.freq * t * 2.).fract();
//         if p < self.prev_phase {
//             self.prev_random = self.curr_random;
//             let white = self.rng.next().unwrap_or(0.);
//             self.curr_random = (self.curr_random + self.rolloff * white).clamp(-1., 1.);
//         }
//         self.prev_phase = p;
//         lerp(self.prev_random, self.curr_random, p)
//     }
// }

#[inline]
fn new_random() -> DistIter<Uniform<f64>, OsRng, f64> {
    OsRng.sample_iter(rand::distributions::Uniform::new(-1., 1.))
}

#[inline]
fn lerp(prev: f64, curr: f64, p: f64) -> f64 {
    (1. - p) * prev + p * curr
}

// pub trait IntoWaveform<S: Sound> {
//     fn sine(self) -> Sine<S>;
//     fn triangle(self) -> Triangle<S>;
//     fn sawtooth(self) -> Sawtooth<S>;
//     fn square(self) -> Square<S>;
//     fn tangent(self) -> Tangent<S>;
//     fn breaker(self) -> Breaker<S>;
//     fn white_noise(self) -> WhiteNoise<S>;
//     fn pink_noise(self) -> PinkNoise<S>;
//     fn brown_noise(self) -> BrownNoise<S>;
//     fn tangent_custom(self, cutoff: f64) -> Option<Tangent<S>>;
//     fn square_custom(self, square_duty: f64) -> Option<Square<S>>;
//     fn brown_noise_custom(self, rolloff: f64) -> Option<BrownNoise<S>>;
// }

// impl<S: Sound> IntoWaveform<S> for S {
//     fn sine(self) -> Sine<S> {
//         Sine::new(self)
//     }
//     fn triangle(self) -> Triangle<S> {
//         Triangle::new(self)
//     }
//     fn sawtooth(self) -> Sawtooth<S> {
//         Sawtooth::new(self)
//     }
//     fn square(self) -> Square<S> {
//         Square::default(self)
//     }
//     fn tangent(self) -> Tangent<S> {
//         Tangent::default(self)
//     }
//     fn breaker(self) -> Breaker<S> {
//         Breaker::new(self)
//     }
//     fn white_noise(self) -> WhiteNoise<S> {
//         WhiteNoise::new(self)
//     }
//     fn pink_noise(self) -> PinkNoise<S> {
//         PinkNoise::new(self)
//     }
//     fn brown_noise(self) -> BrownNoise<S> {
//         BrownNoise::default(self)
//     }
//     fn tangent_custom(self, cutoff: f64) -> Option<Tangent<S>> {
//         Tangent::new(self, cutoff)
//     }
//     fn square_custom(self, square_duty: f64) -> Option<Square<S>> {
//         Square::new(self, square_duty)
//     }
//     fn brown_noise_custom(self, rolloff: f64) -> Option<BrownNoise<S>> {
//         BrownNoise::new(self, rolloff)
//     }
// }

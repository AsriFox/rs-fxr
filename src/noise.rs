use std::f64::consts::TAU;

use crate::lerp;
use crate::traits::{Duration, Proc, ProcState};
use rand::{
    distributions::{DistIter, Uniform},
    prelude::*,
    rngs::OsRng,
};

pub struct Noise<W, E>
where
    W: ProcState,
    E: Proc + Duration,
{
    sample_rate: u32,
    t: f64,
    dt: f64,
    duration: f64,
    waveform: W,
    envelope: E,
}

impl<W, E> Noise<W, E>
where
    W: ProcState,
    E: Proc + Duration,
{
    pub fn new(sample_rate: u32, waveform: W, envelope: E) -> Option<Self> {
        if sample_rate == 0 {
            None
        } else {
            Some(Self {
                sample_rate,
                t: 0.,
                dt: 1. / sample_rate as f64,
                duration: envelope.duration(),
                waveform,
                envelope,
            })
        }
    }

    pub fn render_64<T>(self) -> crate::Samples<T>
    where
        T: From<f64>,
    {
        crate::Samples::<T> {
            sample_rate: self.sample_rate,
            samples: self.map(|s| s.into()).collect(),
        }
    }

    pub fn render_32<T>(self) -> crate::Samples<T>
    where
        T: From<f32>,
    {
        crate::Samples::<T> {
            sample_rate: self.sample_rate,
            samples: self.map(|s| (s as f32).into()).collect(),
        }
    }

    pub fn render_16<T>(self) -> crate::Samples<T>
    where
        T: From<i16>,
    {
        crate::Samples::<T> {
            sample_rate: self.sample_rate,
            samples: self
                .map(|s| ((s * i16::MAX as f64) as i16).into())
                .collect(),
        }
    }
}

impl<W, E> crate::traits::Synth for Noise<W, E>
where
    W: ProcState,
    E: Proc + Duration,
{
}

unsafe impl<W, E> Send for Noise<W, E>
where
    W: ProcState,
    E: Proc + Duration,
{
}

impl<W, E> Duration for Noise<W, E>
where
    W: ProcState,
    E: Proc + Duration,
{
    fn duration(&self) -> f64 {
        self.duration
    }
}

impl<W, E> Iterator for Noise<W, E>
where
    W: ProcState,
    E: Proc + Duration,
{
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.t >= self.duration {
            return None;
        }
        let w = self.waveform.next_value(self.t);
        let w = w * self.envelope.value(self.t);
        self.t += self.dt;
        Some(w)
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
impl ProcState for WhiteNoise {
    fn next_value(&mut self, t: f64) -> f64 {
        let p = (TAU * self.freq * t * 2.).fract();
        if p < self.prev_phase {
            self.prev_random = self.curr_random;
            self.curr_random = self.rng.next().unwrap_or(0.);
        }
        self.prev_phase = p;
        lerp(self.prev_random, self.curr_random, p)
    }
}

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
impl ProcState for PinkNoise {
    fn next_value(&mut self, t: f64) -> f64 {
        let p = (TAU * self.freq * t * 2.).fract();
        if p < self.prev_phase {
            self.prev_random = self.curr_random;
            let white = self.rng.next().unwrap_or(0.);
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
        lerp(self.prev_random, self.curr_random, p)
    }
}

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
impl ProcState for BrownNoise {
    fn next_value(&mut self, t: f64) -> f64 {
        let p = (TAU * self.freq * t * 2.).fract();
        if p < self.prev_phase {
            self.prev_random = self.curr_random;
            let white = self.rng.next().unwrap_or(0.);
            self.curr_random = (self.curr_random + self.rolloff * white).clamp(-1., 1.);
        }
        self.prev_phase = p;
        lerp(self.prev_random, self.curr_random, p)
    }
}

#[inline]
fn new_random() -> DistIter<Uniform<f64>, OsRng, f64> {
    OsRng.sample_iter(rand::distributions::Uniform::new(-1., 1.))
}

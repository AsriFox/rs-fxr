use super::{BrownNoise, PinkNoise, WhiteNoise};
use crate::{
    envelope::Envelope,
    lerp,
    traits::{Duration, Proc, Synth},
};
use rand::{
    distributions::{DistIter, Uniform},
    prelude::*,
    rngs::OsRng,
};
use std::f32::consts::TAU;

#[inline]
fn new_random() -> DistIter<Uniform<f32>, OsRng, f32> {
    OsRng.sample_iter(Uniform::new(-1., 1.))
}

impl<S> WhiteNoise<S>
where
    S: Synth,
{
    pub fn new(samples: S, freq: Envelope) -> Self {
        let duration = samples.duration().min(freq.duration());
        Self {
            samples,
            freq,
            duration,
            rng: new_random(),
            prev_phase: 0.,
            prev_random: 0.,
            curr_random: 0.,
        }
    }
}
impl<S> Synth for WhiteNoise<S> where S: Synth {}
unsafe impl<S> Send for WhiteNoise<S> where S: Synth {}
impl<S> Duration for WhiteNoise<S>
where
    S: Synth,
{
    fn duration(&self) -> f32 {
        todo!()
    }
}
impl<S> Iterator for WhiteNoise<S>
where
    S: Synth,
{
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let t = self.samples.next()?;
        if t >= self.duration {
            return None;
        }
        let p = (TAU * self.freq.value(t) * t * 2.).fract();
        if p < self.prev_phase {
            self.prev_random = self.curr_random;
            self.curr_random = self.rng.next().unwrap_or(0.);
        }
        self.prev_phase = p;
        Some(lerp(self.prev_random, self.curr_random, p))
    }
}

impl<S> PinkNoise<S>
where
    S: Synth,
{
    pub fn new(samples: S, freq: Envelope) -> Self {
        let duration = samples.duration().min(freq.duration());
        Self {
            samples,
            freq,
            duration,
            rng: new_random(),
            prev_phase: 0.,
            prev_random: 0.,
            curr_random: 0.,
            b: [0.; 7],
        }
    }
}
impl<S> Synth for PinkNoise<S> where S: Synth {}
unsafe impl<S> Send for PinkNoise<S> where S: Synth {}
impl<S> Duration for PinkNoise<S>
where
    S: Synth,
{
    fn duration(&self) -> f32 {
        todo!()
    }
}
impl<S> Iterator for PinkNoise<S>
where
    S: Synth,
{
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let t = self.samples.next()?;
        if t >= self.duration {
            return None;
        }
        let p = (TAU * self.freq.value(t) * t * 2.).fract();
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
        Some(lerp(self.prev_random, self.curr_random, p))
    }
}

impl<S> BrownNoise<S>
where
    S: Synth,
{
    pub fn new(samples: S, freq: Envelope, rolloff: f32) -> Option<Self> {
        if !rolloff.is_normal() || rolloff <= 0. || rolloff >= 1. {
            None
        } else {
            let duration = samples.duration().min(freq.duration());
            Some(Self {
                samples,
                freq,
                duration,
                rng: new_random(),
                prev_phase: 0.,
                prev_random: 0.,
                curr_random: 0.,
                rolloff,
            })
        }
    }
    pub fn default(samples: S, freq: Envelope) -> Self {
        let duration = samples.duration().min(freq.duration());
        Self {
            samples,
            freq,
            duration,
            rng: new_random(),
            prev_phase: 0.,
            prev_random: 0.,
            curr_random: 0.,
            rolloff: 0.1,
        }
    }
}
impl<S> Synth for BrownNoise<S> where S: Synth {}
unsafe impl<S> Send for BrownNoise<S> where S: Synth {}
impl<S> Duration for BrownNoise<S>
where
    S: Synth,
{
    fn duration(&self) -> f32 {
        todo!()
    }
}
impl<S> Iterator for BrownNoise<S>
where
    S: Synth,
{
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let t = self.samples.next()?;
        if t >= self.duration {
            return None;
        }
        let p = (TAU * self.freq.value(t) * t * 2.).fract();
        if p < self.prev_phase {
            self.prev_random = self.curr_random;
            let white = self.rng.next().unwrap_or(0.);
            self.curr_random = (self.curr_random + self.rolloff * white).clamp(-1., 1.);
        }
        self.prev_phase = p;
        Some(lerp(self.prev_random, self.curr_random, p))
    }
}

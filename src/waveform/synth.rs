use super::{Breaker, Sawtooth, Sine, Square, Tangent, Triangle};
use crate::{
    envelope::Envelope,
    traits::{Duration, Proc, Synth},
};

impl<S> Sine<S>
where
    S: Synth,
{
    pub fn new(samples: S, freq: Envelope) -> Self {
        let duration = samples.duration().min(freq.duration());
        Self {
            samples,
            freq,
            duration,
        }
    }
}
impl<S> Synth for Sine<S> where S: Synth {}
unsafe impl<S> Send for Sine<S> where S: Synth {}
impl<S> Duration for Sine<S>
where
    S: Synth,
{
    fn duration(&self) -> f32 {
        self.duration
    }
}
impl<S> Iterator for Sine<S>
where
    S: Synth,
{
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let t = self.samples.next()?;
        Some((std::f32::consts::TAU * self.freq.value(t) * t).sin())
    }
}

impl<S> Triangle<S>
where
    S: Synth,
{
    pub fn new(samples: S, freq: Envelope) -> Self {
        let duration = samples.duration().min(freq.duration());
        Self {
            samples,
            freq,
            duration,
        }
    }
}
impl<S> Synth for Triangle<S> where S: Synth {}
unsafe impl<S> Send for Triangle<S> where S: Synth {}
impl<S> Duration for Triangle<S>
where
    S: Synth,
{
    fn duration(&self) -> f32 {
        self.duration
    }
}
impl<S> Iterator for Triangle<S>
where
    S: Synth,
{
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let t = self.samples.next()?;
        if t >= self.duration {
            return None;
        }
        let v = (self.freq.value(t) * t).fract();
        Some(if v < 0.25 {
            4. * v
        } else if v < 0.75 {
            2. - 4. * v
        } else {
            -4. + 4. * v
        })
    }
}

impl<S> Sawtooth<S>
where
    S: Synth,
{
    pub fn new(samples: S, freq: Envelope) -> Self {
        let duration = samples.duration().min(freq.duration());
        Self {
            samples,
            freq,
            duration,
        }
    }
}
impl<S> Synth for Sawtooth<S> where S: Synth {}
unsafe impl<S> Send for Sawtooth<S> where S: Synth {}
impl<S> Duration for Sawtooth<S>
where
    S: Synth,
{
    fn duration(&self) -> f32 {
        self.duration
    }
}
impl<S> Iterator for Sawtooth<S>
where
    S: Synth,
{
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let t = self.samples.next()?;
        if t >= self.duration {
            return None;
        }
        Some((self.freq.value(t) * t).fract() * 2. - 1.)
    }
}

impl<S> Breaker<S>
where
    S: Synth,
{
    pub fn new(samples: S, freq: Envelope) -> Self {
        let duration = samples.duration().min(freq.duration());
        Self {
            samples,
            freq,
            duration,
        }
    }
}
impl<S> Synth for Breaker<S> where S: Synth {}
unsafe impl<S> Send for Breaker<S> where S: Synth {}
impl<S> Duration for Breaker<S>
where
    S: Synth,
{
    fn duration(&self) -> f32 {
        self.duration
    }
}
impl<S> Iterator for Breaker<S>
where
    S: Synth,
{
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let t = self.samples.next()?;
        if t >= self.duration {
            return None;
        }
        const BREAKER_OFFSET: f32 = 0.8660254; // f32::sqrt(0.75);
        let v = (self.freq.value(t) * t + BREAKER_OFFSET).fract();
        Some(-1. + 2. * (1. - 2. * v * v).abs())
    }
}

impl<S> Tangent<S>
where
    S: Synth,
{
    pub fn new(samples: S, freq: Envelope, cutoff: f32) -> Option<Self> {
        if !cutoff.is_normal() || cutoff <= 0. {
            None
        } else {
            let duration = samples.duration().min(freq.duration());
            Some(Self {
                samples,
                freq,
                cutoff,
                duration,
            })
        }
    }
    pub fn default(samples: S, freq: Envelope) -> Self {
        let duration = samples.duration().min(freq.duration());
        Self {
            samples,
            freq,
            cutoff: 0.15,
            duration,
        }
    }
}
impl<S> Synth for Tangent<S> where S: Synth {}
unsafe impl<S> Send for Tangent<S> where S: Synth {}
impl<S> Duration for Tangent<S>
where
    S: Synth,
{
    fn duration(&self) -> f32 {
        self.duration
    }
}
impl<S> Iterator for Tangent<S>
where
    S: Synth,
{
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let t = self.samples.next()?;
        if t >= self.duration {
            return None;
        }
        let tan = (std::f32::consts::PI * self.freq.value(t) * t).tan();
        Some((tan / self.cutoff).clamp(-1., 1.))
    }
}

impl<S> Square<S>
where
    S: Synth,
{
    pub fn new(samples: S, freq: Envelope, square_duty: Envelope) -> Self {
        let duration = samples
            .duration()
            .min(freq.duration())
            .min(square_duty.duration());
        Self {
            samples,
            freq,
            square_duty,
            duration,
        }
    }
    pub fn default(samples: S, freq: Envelope) -> Self {
        let duration = samples.duration().min(freq.duration());
        Self {
            samples,
            freq,
            square_duty: Envelope::new_simple(f32::INFINITY, 0.5).unwrap(),
            duration,
        }
    }
}
impl<S> Synth for Square<S> where S: Synth {}
unsafe impl<S> Send for Square<S> where S: Synth {}
impl<S> Duration for Square<S>
where
    S: Synth,
{
    fn duration(&self) -> f32 {
        self.duration
    }
}
impl<S> Iterator for Square<S>
where
    S: Synth,
{
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let t = self.samples.next()?;
        if t >= self.duration {
            return None;
        }
        let v = (self.freq.value(t) * t).fract();
        Some(if v < self.square_duty.value(t) {
            1.
        } else {
            -1.
        })
    }
}

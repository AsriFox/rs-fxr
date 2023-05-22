use crate::traits::{Duration, Proc};

pub struct Synth<W, E>
where
    W: Proc,
    E: Proc + Duration,
{
    sample_rate: u32,
    t: f64,
    dt: f64,
    duration: f64,
    waveform: W,
    envelope: E,
}

impl<W, E> Synth<W, E>
where
    W: Proc,
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

impl<W, E> crate::traits::Synth for Synth<W, E>
where
    W: Proc,
    E: Proc + Duration,
{
}

unsafe impl<W, E> Send for Synth<W, E>
where
    W: Proc,
    E: Proc + Duration,
{
}

impl<W, E> Duration for Synth<W, E>
where
    W: Proc,
    E: Proc + Duration,
{
    fn duration(&self) -> f64 {
        self.duration
    }
}

impl<W, E> Iterator for Synth<W, E>
where
    W: Proc,
    E: Proc + Duration,
{
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.t >= self.duration {
            return None;
        }
        let w = self.waveform.value(self.t);
        let w = w * self.envelope.value(self.t);
        self.t += self.dt;
        Some(w)
    }
}

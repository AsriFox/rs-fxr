use crate::traits::{Duration, Proc};

pub struct Synth<W, E>
where
    W: Proc,
    E: Proc + Duration,
{
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
    pub fn new(sample_rate: f64, waveform: W, envelope: E) -> Option<Self> {
        if !sample_rate.is_normal() || sample_rate <= 0. {
            None
        } else {
            Some(Self {
                t: 0.,
                dt: 1. / sample_rate,
                duration: envelope.duration(),
                waveform,
                envelope,
            })
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

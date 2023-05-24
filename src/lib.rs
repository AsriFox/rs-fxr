pub mod bit_crush;
pub mod envelope;
pub mod passband;
pub mod traits;
pub mod waveform;

#[cfg(feature = "serde")]
pub mod serde;

#[cfg(test)]
mod tests;

pub struct Samples<T> {
    pub sample_rate: u32,
    pub samples: Vec<T>,
}

#[inline]
pub(crate) fn lerp(prev: f32, curr: f32, p: f32) -> f32 {
    (1. - p) * prev + p * curr
}

pub struct Clock {
    sample_rate: u32,
    t: f32,
    dt: f32,
}
impl Clock {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate,
            t: 0.,
            dt: 1. / sample_rate as f32,
        }
    }
}
impl traits::Synth for Clock {}
unsafe impl Send for Clock {}
impl traits::Duration for Clock {
    fn duration(&self) -> f32 {
        f32::INFINITY
    }
}
impl Iterator for Clock {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let t = self.t;
        self.t += self.dt;
        Some(t)
    }
}

pub mod envelope;
pub mod noise;
pub mod passband;
pub mod synth;
pub mod traits;
pub mod waveform;

#[cfg(feature = "json")]
pub mod json;

pub struct Samples<T> {
    pub sample_rate: u32,
    pub samples: Vec<T>,
}

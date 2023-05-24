mod noise;
mod synth;

use crate::{envelope::Envelope, traits::Synth};

#[cfg(feature = "serde")]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "waveform")]
#[serde(rename_all = "lowercase")]
pub enum WaveformType {
    Sine,
    Triangle,
    Sawtooth,
    Breaker,
    Tangent,
    #[serde(rename_all = "camelCase")]
    Square {
        square_duty: f32,
    },
    WhiteNoise,
    PinkNoise,
    BrownNoise,
}
#[cfg(not(feature = "serde"))]
pub enum WaveformType {
    Sine,
    Triangle,
    Sawtooth,
    Breaker,
    Tangent,
    Square { square_duty: f32 },
    WhiteNoise,
    PinkNoise,
    BrownNoise,
}

impl WaveformType {
    pub fn build<'a, S>(self, samples: S, freq: Envelope) -> Box<dyn Synth + 'a>
    where
        S: Synth + 'a,
    {
        match self {
            Self::Sine => Box::new(Sine::new(samples, freq)),
            Self::Triangle => Box::new(Triangle::new(samples, freq)),
            Self::Sawtooth => Box::new(Sawtooth::new(samples, freq)),
            Self::Breaker => Box::new(Breaker::new(samples, freq)),
            Self::Tangent => Box::new(Tangent::default(samples, freq)),
            Self::Square { square_duty } => {
                let square_duty = Envelope::new_simple(f32::INFINITY, square_duty).unwrap();
                Box::new(Square::new(samples, freq, square_duty))
            }
            Self::WhiteNoise => Box::new(WhiteNoise::new(samples, freq)),
            Self::PinkNoise => Box::new(PinkNoise::new(samples, freq)),
            Self::BrownNoise => Box::new(BrownNoise::default(samples, freq)),
        }
    }
}

pub struct Sine<S>
where
    S: Synth,
{
    samples: S,
    freq: Envelope,
    duration: f32,
}

pub struct Triangle<S>
where
    S: Synth,
{
    samples: S,
    freq: Envelope,
    duration: f32,
}

pub struct Sawtooth<S>
where
    S: Synth,
{
    samples: S,
    freq: Envelope,
    duration: f32,
}

pub struct Breaker<S>
where
    S: Synth,
{
    samples: S,
    freq: Envelope,
    duration: f32,
}

pub struct Tangent<S>
where
    S: Synth,
{
    samples: S,
    freq: Envelope,
    cutoff: f32,
    duration: f32,
}

pub struct Square<S>
where
    S: Synth,
{
    samples: S,
    freq: Envelope,
    square_duty: Envelope,
    duration: f32,
}

use rand::{
    distributions::{DistIter, Uniform},
    rngs::OsRng,
};

pub struct WhiteNoise<S>
where
    S: Synth,
{
    samples: S,
    freq: Envelope,
    duration: f32,

    // interpolated
    rng: DistIter<Uniform<f32>, OsRng, f32>,
    prev_phase: f32,
    prev_random: f32,
    curr_random: f32,
    // interpolate: bool,
}

pub struct PinkNoise<S>
where
    S: Synth,
{
    samples: S,
    freq: Envelope,
    duration: f32,

    // interpolated
    rng: DistIter<Uniform<f32>, OsRng, f32>,
    prev_phase: f32,
    prev_random: f32,
    curr_random: f32,
    b: [f32; 7],
    // interpolate: bool,
}

pub struct BrownNoise<S>
where
    S: Synth,
{
    samples: S,
    freq: Envelope,
    duration: f32,

    // interpolated
    rng: DistIter<Uniform<f32>, OsRng, f32>,
    prev_phase: f32,
    prev_random: f32,
    curr_random: f32,
    rolloff: f32,
    // interpolate: bool,
}

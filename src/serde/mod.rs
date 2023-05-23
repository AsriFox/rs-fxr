#[cfg(feature = "json")]
pub mod json;

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError, ValidationErrors};

use crate::{
    envelope::Envelope,
    noise::{BrownNoise, Noise, PinkNoise, WhiteNoise},
    synth::Synth,
    waveform::{Breaker, Sawtooth, Sine, Square, Tangent, Triangle},
};

#[derive(Debug, Serialize, Deserialize)]
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
        square_duty: f64,
    },
    WhiteNoise,
    PinkNoise,
    BrownNoise,
}

impl Validate for WaveformType {
    fn validate(&self) -> Result<(), ValidationErrors> {
        match self {
            WaveformType::Square { square_duty } => {
                if *square_duty < 0. || *square_duty > 100. {
                    let mut errors = ValidationErrors::new();
                    errors.add(
                        "square_duty",
                        ValidationError::new(
                            "'square_duty' must be a percentage value between 0 and 100",
                        ),
                    );
                    Err(errors)
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    }
}

impl WaveformType {
    pub fn build(
        self,
        sample_rate: u32,
        frequency: f64,
        envelope: Envelope,
    ) -> Box<dyn crate::traits::Synth> {
        match self {
            Self::Sine => {
                let waveform = Sine::new_simple(frequency).unwrap();
                let synth = Synth::new(sample_rate, waveform, envelope).unwrap();
                Box::new(synth)
            }
            Self::Triangle => {
                let waveform = Triangle::new_simple(frequency).unwrap();
                let synth = Synth::new(sample_rate, waveform, envelope).unwrap();
                Box::new(synth)
            }
            Self::Sawtooth => {
                let waveform = Sawtooth::new_simple(frequency).unwrap();
                let synth = Synth::new(sample_rate, waveform, envelope).unwrap();
                Box::new(synth)
            }
            Self::Breaker => {
                let waveform = Breaker::new_simple(frequency).unwrap();
                let synth = Synth::new(sample_rate, waveform, envelope).unwrap();
                Box::new(synth)
            }
            Self::Tangent => {
                let waveform = Tangent::default_simple(frequency).unwrap();
                let synth = Synth::new(sample_rate, waveform, envelope).unwrap();
                Box::new(synth)
            }
            Self::Square { square_duty } => {
                let waveform = Square::new_simple(frequency, square_duty).unwrap();
                let synth = Synth::new(sample_rate, waveform, envelope).unwrap();
                Box::new(synth)
            }
            Self::WhiteNoise => {
                let waveform = WhiteNoise::new_simple(frequency).unwrap();
                let synth = Noise::new(sample_rate, waveform, envelope).unwrap();
                Box::new(synth)
            }
            Self::PinkNoise => {
                let waveform = PinkNoise::new_simple(frequency).unwrap();
                let synth = Noise::new(sample_rate, waveform, envelope).unwrap();
                Box::new(synth)
            }
            Self::BrownNoise => {
                let waveform = BrownNoise::default_simple(frequency).unwrap();
                let synth = Noise::new(sample_rate, waveform, envelope).unwrap();
                Box::new(synth)
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Description {
    #[serde(rename = "_version")]
    #[validate(custom(function = "Description::validate_fxr_version"))]
    pub fxr_version: i32,

    #[serde(rename = "_name")]
    pub fxr_name: String,

    #[serde(default = "Description::sample_rate_default")]
    #[validate(range(min = 1))]
    pub sample_rate: u32,

    #[serde(default)]
    #[validate(range(min = 0.))]
    pub attack: f64,

    #[serde(default)]
    #[validate(range(min = 0.))]
    pub sustain: f64,

    #[serde(default)]
    #[validate(range(min = 0.))]
    pub decay: f64,

    #[serde(default)]
    #[validate(range(min = 0., max = 100.))]
    pub sustain_punch: f64,

    #[serde(default = "Description::amplification_default")]
    #[validate(range(min = 0.))]
    pub amplification: f64,

    #[validate(range(min = 0.))]
    pub frequency: f64,

    #[serde(flatten)]
    #[validate]
    pub waveform: WaveformType,
}

impl Description {
    pub fn build(self) -> Result<Box<dyn crate::traits::Synth>, ValidationErrors> {
        let mut errors = if let Err(errors) = self.validate() {
            errors
        } else {
            ValidationErrors::new()
        };
        if self.attack + self.sustain + self.decay == 0. {
            errors.add("duration", ValidationError::new("Sound duration must be positive; consider setting 'attack', 'sustain' and/or 'decay' values."));
        }
        if !errors.is_empty() {
            return Err(errors);
        }

        let envelope = Envelope::from_duration(
            self.amplification,
            self.attack,
            self.sustain,
            self.decay,
            self.sustain_punch,
            None,
        )
        .unwrap();

        Ok(self
            .waveform
            .build(self.sample_rate, self.frequency, envelope))
    }

    #[inline]
    fn validate_fxr_version(value: i32) -> Result<(), ValidationError> {
        match value {
            1 => Ok(()),
            _ => {
                let mut error = ValidationError::new("JFXR version mismatch");
                error.add_param::<i32>(std::borrow::Cow::Borrowed("_version"), &value);
                Err(error)
            }
        }
    }

    #[inline]
    fn sample_rate_default() -> u32 {
        44100
    }

    #[inline]
    fn amplification_default() -> f64 {
        100.
    }
}

#[cfg(feature = "json")]
pub mod json;

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError, ValidationErrors};

use crate::{envelope::Envelope, waveform::WaveformType};

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
    pub attack: f32,

    #[serde(default)]
    #[validate(range(min = 0.))]
    pub sustain: f32,

    #[serde(default)]
    #[validate(range(min = 0.))]
    pub decay: f32,

    #[serde(default)]
    #[validate(range(min = 0., max = 100.))]
    pub sustain_punch: f32,

    #[serde(default = "Description::amplification_default")]
    #[validate(range(min = 0.))]
    pub amplification: f32,

    #[validate(range(min = 0.))]
    pub frequency: f32,

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

        let samples = crate::Clock::new(self.sample_rate);

        let freq = Envelope::new_simple(f32::INFINITY, self.frequency).unwrap();

        Ok(self.waveform.build(samples, freq))
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
    fn amplification_default() -> f32 {
        100.
    }
}

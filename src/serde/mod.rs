#[cfg(feature = "json")]
pub mod json;

use crate::{
    envelope::Envelope,
    noise::{BrownNoise, Noise, PinkNoise, WhiteNoise},
    synth::Synth,
    waveform::{Breaker, Sawtooth, Sine, Square, Tangent, Triangle},
};

pub enum WaveformType {
    Sine,
    Triangle,
    Sawtooth,
    Breaker,
    Tangent,
    Square(f64),
    WhiteNoise,
    PinkNoise,
    BrownNoise,
}

pub struct Description {
    sample_rate: u32,

    attack: f64,
    sustain: f64,
    decay: f64,

    sustain_punch: f64,
    amp: f64,

    freq: f64,
    waveform: WaveformType,
}

impl Description {
    pub fn errors(&self) -> Vec<String> {
        let mut errors = vec![];
        if self.sample_rate <= 0 {
            errors.push(format!(
                "'sample_rate' must be positive: {}",
                self.sample_rate
            ));
        }
        if self.attack < 0. {
            errors.push(format!(
                "'attack' duration must not be negative: {}",
                self.attack
            ));
        }
        if self.sustain < 0. {
            errors.push(format!(
                "'sustain' duration must not be negative: {}",
                self.sustain
            ));
        }
        if self.decay < 0. {
            errors.push(format!(
                "'decay' duration must not be negative: {}",
                self.decay
            ));
        }
        if self.attack + self.sustain + self.decay <= 0. {
            errors.push(format!(
                "Sound duration must be positive; assign 'attack', 'sustain' and/or 'decay' values"
            ));
        }
        if self.amp < 0. {
            errors.push(format!(
                "'amplification' value must not be negative: {}",
                self.amp
            ));
        }
        if self.freq < 0. {
            errors.push(format!("'frequency' must not be negative: {}", self.freq));
        }
        match self.waveform {
            WaveformType::Square(square_duty) => {
                if square_duty < 0. || square_duty > 1. {
                    errors.push(format!("'square_duty' value must be between 0 and 1"));
                }
            }
            _ => {}
        }
        return errors;
    }

    pub fn build(self) -> Result<Box<dyn crate::traits::Synth>, Vec<String>> {
        let errors = self.errors();
        if !errors.is_empty() {
            return Err(errors);
        }

        let envelope = Envelope::from_duration(
            self.amp,
            self.attack,
            self.sustain,
            self.decay,
            self.sustain_punch,
            None,
        )
        .unwrap();

        match self.waveform {
            WaveformType::Sine => {
                if let Some(waveform) = Sine::new_simple(self.freq) {
                    let synth = Synth::new(self.sample_rate, waveform, envelope).unwrap();
                    Ok(Box::new(synth))
                } else {
                    Err(vec!["Failed to create 'Sine' waveform".to_string()])
                }
            }
            WaveformType::Triangle => {
                if let Some(waveform) = Triangle::new_simple(self.freq) {
                    let synth = Synth::new(self.sample_rate, waveform, envelope).unwrap();
                    Ok(Box::new(synth))
                } else {
                    Err(vec!["Failed to create 'Triangle' waveform".to_string()])
                }
            }
            WaveformType::Sawtooth => {
                if let Some(waveform) = Sawtooth::new_simple(self.freq) {
                    let synth = Synth::new(self.sample_rate, waveform, envelope).unwrap();
                    Ok(Box::new(synth))
                } else {
                    Err(vec!["Failed to create 'Sawtooth' waveform".to_string()])
                }
            }
            WaveformType::Breaker => {
                if let Some(waveform) = Breaker::new_simple(self.freq) {
                    let synth = Synth::new(self.sample_rate, waveform, envelope).unwrap();
                    Ok(Box::new(synth))
                } else {
                    Err(vec!["Failed to create 'Breaker' waveform".to_string()])
                }
            }
            WaveformType::Tangent => {
                if let Some(waveform) = Tangent::default_simple(self.freq) {
                    let synth = Synth::new(self.sample_rate, waveform, envelope).unwrap();
                    Ok(Box::new(synth))
                } else {
                    Err(vec!["Failed to create 'Tangent' waveform".to_string()])
                }
            }
            WaveformType::Square(square_duty) => {
                if let Some(waveform) = Square::new_simple(self.freq, square_duty) {
                    let synth = Synth::new(self.sample_rate, waveform, envelope).unwrap();
                    Ok(Box::new(synth))
                } else {
                    Err(vec!["Failed to create 'Square' waveform".to_string()])
                }
            }
            WaveformType::WhiteNoise => {
                if let Some(waveform) = WhiteNoise::new_simple(self.freq) {
                    let synth = Noise::new(self.sample_rate, waveform, envelope).unwrap();
                    Ok(Box::new(synth))
                } else {
                    Err(vec!["Failed to create 'WhiteNoise' waveform".to_string()])
                }
            }
            WaveformType::PinkNoise => {
                if let Some(waveform) = PinkNoise::new_simple(self.freq) {
                    let synth = Noise::new(self.sample_rate, waveform, envelope).unwrap();
                    Ok(Box::new(synth))
                } else {
                    Err(vec!["Failed to create 'PinkNoise' waveform".to_string()])
                }
            }
            WaveformType::BrownNoise => {
                if let Some(waveform) = BrownNoise::default_simple(self.freq) {
                    let synth = Noise::new(self.sample_rate, waveform, envelope).unwrap();
                    Ok(Box::new(synth))
                } else {
                    Err(vec!["Failed to create 'BrownNoise' waveform".to_string()])
                }
            }
        }
    }
}

use crate::serde::{Description, WaveformType};

pub fn parse(jfxr: serde_json::Value) -> Result<Description, String> {
    match jfxr.get("_version") {
        Some(version) => match version.as_i64() {
            Some(1) => {}
            Some(v) => return Err(format!("JFXR version mismatch: got _version={}", v)),
            None => return Err(format!("'_version' is not a number: {}", version)),
        },
        None => todo!("Version is not provided"),
    }

    let sample_rate = match jfxr.get("sample_rate") {
        Some(sample_rate) => match sample_rate.as_u64() {
            Some(sample_rate) => sample_rate,
            None => return Err(format!("'sample_rate' is not a number: {}", sample_rate)),
        },
        None => 44100, // DEFAULT_SAMPLE_RATE
    };
    let sample_rate = sample_rate as u32;

    let attack = match jfxr.get("attack") {
        Some(attack) => match attack.as_f64() {
            Some(attack) => attack,
            None => return Err(format!("'attack' is not a number: {}", attack)),
        },
        None => 0.,
    };

    let sustain = match jfxr.get("sustain") {
        Some(sustain) => match sustain.as_f64() {
            Some(sustain) => sustain,
            None => return Err(format!("'sustain' is not a number: {}", sustain)),
        },
        None => 0.,
    };

    let decay = match jfxr.get("decay") {
        Some(decay) => match decay.as_f64() {
            Some(decay) => decay,
            None => return Err(format!("'decay' is not a number: {}", decay)),
        },
        None => 0.,
    };

    let sustain_punch = match jfxr.get("sustain_punch") {
        Some(sustain_punch) => match sustain_punch.as_f64() {
            Some(sustain_punch) => sustain_punch,
            None => {
                return Err(format!(
                    "'sustain_punch' is not a number: {}",
                    sustain_punch
                ))
            }
        },
        None => 0.,
    };
    let sustain_punch = 0.01 * sustain_punch;

    let amp = match jfxr.get("amplification") {
        Some(amp) => match amp.as_f64() {
            Some(amp) => amp,
            None => return Err(format!("'amplification' is not a number: {}", amp)),
        },
        None => 100.,
    };
    let amp = 0.01 * amp;

    let freq = match jfxr.get("frequency") {
        Some(freq) => match freq.as_f64() {
            Some(freq) => freq,
            None => return Err(format!("'frequency' is not a number: {}", freq)),
        },
        None => return Err(format!("Frequency is not set")),
    };

    let waveform = match jfxr.get("waveform") {
        Some(waveform) => match waveform.as_str() {
            Some(waveform) => waveform.to_string(),
            None => return Err(format!("'waveform' is not a string: {}", waveform)),
        },
        None => return Err(format!("Waveform is not set")),
    };
    let waveform = match waveform.as_str() {
        "sine" => WaveformType::Sine,
        "triangle" => WaveformType::Triangle,
        "sawtooth" => WaveformType::Sawtooth,
        "breaker" => WaveformType::Breaker,
        "tangent" => WaveformType::Tangent,
        "square" => {
            let square_duty = match jfxr.get("square_duty") {
                Some(square_duty) => match square_duty.as_f64() {
                    Some(square_duty) => square_duty,
                    None => return Err(format!("'squareDuty' is not a number: {}", square_duty)),
                },
                None => 0.5,
            };
            WaveformType::Square(square_duty)
        }
        "whitenoise" => WaveformType::WhiteNoise,
        "pinknoise" => WaveformType::PinkNoise,
        "brownnoise" => WaveformType::BrownNoise,
        _ => return Err(format!("Waveform type '{}' is not defined", waveform)),
    };

    let description = Description {
        sample_rate,
        attack,
        sustain,
        decay,
        sustain_punch,
        amp,
        freq,
        waveform,
    };
    let errors = description.errors();
    if errors.is_empty() {
        Ok(description)
    } else {
        Err(errors.join("\n"))
    }
}

use crate::{
    envelope::{Envelope, Interval},
    noise::{BrownNoise, Noise, PinkNoise, WhiteNoise},
    synth::Synth,
    waveform::{Breaker, Sawtooth, Sine, Square, Tangent, Triangle},
};

pub fn parse(jfxr: serde_json::Value) -> Result<Box<dyn crate::traits::Synth>, String> {
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
    if sample_rate <= 0 {
        return Err(format!("'sample_rate' must be positive: {}", sample_rate));
    }
    let sample_rate = sample_rate as u32;

    let attack = match jfxr.get("attack") {
        Some(attack) => match attack.as_f64() {
            Some(attack) => attack,
            None => return Err(format!("'attack' is not a number: {}", attack)),
        },
        None => 0.,
    };
    if attack < 0. {
        return Err(format!("'attack' must not be negative: {}", attack));
    }

    let sustain = match jfxr.get("sustain") {
        Some(sustain) => match sustain.as_f64() {
            Some(sustain) => sustain,
            None => return Err(format!("'sustain' is not a number: {}", sustain)),
        },
        None => 0.,
    };
    if sustain < 0. {
        return Err(format!("'sustain' must not be negative: {}", sustain));
    }

    let decay = match jfxr.get("decay") {
        Some(decay) => match decay.as_f64() {
            Some(decay) => decay,
            None => return Err(format!("'decay' is not a number: {}", decay)),
        },
        None => 0.,
    };
    if decay < 0. {
        return Err(format!("'decay' must not be negative: {}", decay));
    }

    if attack + sustain + decay <= 0. {
        return Err(format!(
            "Sound duration must be positive; assign 'attack', 'sustain' and/or 'decay' values"
        ));
    }

    let amp = match jfxr.get("amplification") {
        Some(amp) => match amp.as_f64() {
            Some(amp) => amp,
            None => return Err(format!("'amplification' is not a number: {}", amp)),
        },
        None => 100.,
    };
    if amp < 0. {
        return Err(format!("'amplification' must not be negative: {}", amp));
    }
    let amp = 0.01 * amp;

    let mut segments = vec![];
    if let Some(a) = Interval::new(attack, 0., amp) {
        segments.push(a);
    }
    if let Some(s) = Interval::new(sustain, amp, amp) {
        segments.push(s);
    }
    if let Some(d) = Interval::new(decay, amp, 0.) {
        segments.push(d);
    }
    let envelope = Envelope::new(segments);

    let freq = match jfxr.get("frequency") {
        Some(freq) => match freq.as_f64() {
            Some(freq) => freq,
            None => return Err(format!("'frequency' is not a number: {}", freq)),
        },
        None => return Err(format!("Frequency is not set")),
    };
    if freq < 0. {
        return Err(format!("'frequency' must not be negative: {}", freq));
    }

    let waveform = match jfxr.get("waveform") {
        Some(waveform) => match waveform.as_str() {
            Some(waveform) => waveform.to_string(),
            None => return Err(format!("'waveform' is not a string: {}", waveform)),
        },
        None => return Err(format!("Waveform is not set")),
    };
    Ok(match waveform.as_str() {
        "sine" => Box::new(Synth::new(sample_rate, Sine::new(freq).unwrap(), envelope).unwrap()),
        "triangle" => {
            Box::new(Synth::new(sample_rate, Triangle::new(freq).unwrap(), envelope).unwrap())
        }
        "sawtooth" => {
            Box::new(Synth::new(sample_rate, Sawtooth::new(freq).unwrap(), envelope).unwrap())
        }
        "breaker" => {
            Box::new(Synth::new(sample_rate, Breaker::new(freq).unwrap(), envelope).unwrap())
        }
        "tangent" => {
            // let waveform = if let Ok(cutoff) = find(&args, "tangent_cutoff") {
            //     Tangent::new(freq, cutoff.parse::<f64>().unwrap()).unwrap()
            // } else {
            //     Tangent::default(freq).unwrap()
            // };
            Box::new(Synth::new(sample_rate, Tangent::default(freq).unwrap(), envelope).unwrap())
        }
        "square" => {
            let waveform = match jfxr.get("squareDuty") {
                Some(square_duty) => match square_duty.as_f64() {
                    Some(square_duty) => {
                        if square_duty < 0. || square_duty > 100. {
                            return Err(format!(
                                "'squareDuty' must be a percentage value between 0 and 100: {}",
                                square_duty
                            ));
                        } else {
                            Square::new(freq, 0.01 * square_duty).unwrap()
                        }
                    }
                    None => return Err(format!("'squareDuty' is not a number: {}", square_duty)),
                },
                None => Square::default(freq).unwrap(),
            };
            Box::new(Synth::new(sample_rate, waveform, envelope).unwrap())
        }
        "whitenoise" => {
            Box::new(Noise::new(sample_rate, WhiteNoise::new(freq).unwrap(), envelope).unwrap())
        }
        "pinknoise" => {
            Box::new(Noise::new(sample_rate, PinkNoise::new(freq).unwrap(), envelope).unwrap())
        }
        "brownnoise" => {
            // let waveform = if let Ok(rolloff) = find(&args, "brown_noise_rolloff") {
            //     BrownNoise::new(freq, rolloff.parse::<f64>().unwrap()).unwrap()
            // } else {
            //     BrownNoise::default(freq).unwrap()
            // };
            Box::new(Noise::new(sample_rate, BrownNoise::default(freq).unwrap(), envelope).unwrap())
        }
        _ => return Err(format!("Waveform type '{}' is not defined", waveform)),
    })
}

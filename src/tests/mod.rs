use crate::serde::{Description, WaveformType};

#[test]
fn serde_json_serialize() {
    let description = Description {
        fxr_version: 1,
        fxr_name: "test".to_string(),
        sample_rate: 44100,
        attack: 0.,
        sustain: 1.,
        decay: 0.,
        sustain_punch: 0.,
        amplification: 100.,
        frequency: 200.,
        // waveform: WaveformType::Square { square_duty: 0.5 },
        waveform: WaveformType::BrownNoise,
    };
    let jfxr = serde_json::to_value::<Description>(description).unwrap();
    assert!(jfxr.is_object());
    println!("{}", jfxr);
}

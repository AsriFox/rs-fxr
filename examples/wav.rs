use rs_fxr::{
    envelope::Envelope,
    noise::{Noise, PinkNoise},
};

fn main() -> anyhow::Result<()> {
    let sample_rate = 44100;
    let waveform = PinkNoise::new(200.).unwrap();
    let envelope = Envelope::from_points(vec![(1., 1.), (2., 1.), (3., 0.)]).unwrap();
    let mut wave = Noise::new(sample_rate as f64, waveform, envelope).unwrap();

    let spec = hound::WavSpec {
        channels: 2,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("target/example.wav", spec)?;
    while let Some(sample) = wave.next() {
        let amp = i16::MAX as f64;
        writer.write_sample((sample * amp) as i16)?;
    }
    writer.finalize()?;
    Ok(())
}

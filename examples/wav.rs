use rs_fxr::{
    envelope::Envelope, passband::Filterable, traits::Duration, waveform::Square, Clock, Samples,
};

fn main() -> anyhow::Result<()> {
    let sample_rate = 8000;
    let channels = 2;

    let samples = Clock::new(sample_rate);
    let freq = Envelope::new_simple(f32::INFINITY, 200.).unwrap();
    let waveform = Square::default(samples, freq);
    let envelope = Envelope::from_duration(1., 1., 1., 1., 0., None).unwrap();
    let samples: Vec<f32> = waveform
        .take((envelope.duration() * sample_rate as f32) as usize)
        .collect();
    let wave = Samples::<f32> {
        sample_rate,
        samples,
    }
    .low_pass(400.);

    let spec = hound::WavSpec {
        channels,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("target/example.wav", spec)?;
    let mut writer = writer.get_i16_writer(wave.samples.len() as u32 * channels as u32);
    wave.samples.into_iter().for_each(|sample| {
        let sample = (sample * i16::MAX as f32) as i16;
        for _ in 0..channels {
            writer.write_sample(sample);
        }
    });
    writer.flush()?;
    Ok(())
}

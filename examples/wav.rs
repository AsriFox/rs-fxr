use rs_fxr::{
    envelope::Envelope,
    // traits::Duration,
    passband::Filterable,
    synth::Synth,
    waveform::Square,
};

fn main() -> anyhow::Result<()> {
    let sample_rate = 8000;
    let channels = 2;

    let waveform = Square::default(200.).unwrap();
    let envelope = Envelope::from_points(vec![(1., 1.), (2., 1.), (3., 0.)]).unwrap();
    let wave = Synth::new(sample_rate, waveform, envelope).unwrap();
    let wave = wave.render_32::<f32>().low_pass(400.);

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

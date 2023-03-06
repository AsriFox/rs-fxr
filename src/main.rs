mod envelope;
mod phase;
mod sound;
mod waveform;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use waveform::IntoWaveform;

fn main() -> anyhow::Result<()> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No output device found");
    let config = device.default_output_config().unwrap();

    let wave = phase::Phase::with_freq(400., config.sample_rate().0 as f64)
        .unwrap()
        .sine();

    match config.sample_format() {
        cpal::SampleFormat::I16 => run::<i16, _>(&device, &config.into(), wave),
        cpal::SampleFormat::U16 => run::<u16, _>(&device, &config.into(), wave),
        cpal::SampleFormat::F32 => run::<f32, _>(&device, &config.into(), wave),
    }
}

pub fn run<T, S: sound::Sound + 'static>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    mut wave: S,
) -> Result<(), anyhow::Error>
where
    T: cpal::Sample,
{
    // let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            for frame in data.chunks_mut(channels) {
                let value = wave.next().unwrap_or(0.) as f32;
                let value: T = cpal::Sample::from::<f32>(&value);
                for sample in frame.iter_mut() {
                    *sample = value;
                }
            }
        },
        err_fn,
    )?;
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(1000));

    Ok(())
}

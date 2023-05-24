use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rs_fxr::{bit_crush::BitCrush, envelope::Envelope, traits::Duration, waveform::Sine};

fn main() -> anyhow::Result<()> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No output device found");
    let config = device.default_output_config().unwrap();

    match config.sample_format() {
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into()),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into()),
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()),
    }
}

pub fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>
where
    T: cpal::Sample,
{
    let sample_rate = config.sample_rate.0;
    let channels = config.channels as usize;

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let samples = rs_fxr::Clock::new(sample_rate);
    let freq =
        Envelope::from_points(vec![(0., 200.), (f32::INFINITY, 200.)], Some((0.001, 10.))).unwrap();
    let waveform = Sine::new(samples, freq);
    let envelope = Envelope::from_duration(0.5, 1., 1., 1., 1., Some((0.2, 10.))).unwrap();
    let mut wave = BitCrush::B2.apply(waveform);
    let duration = wave.duration();

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            for frame in data.chunks_mut(channels) {
                let value = wave.next().unwrap_or(0.);
                let value: T = cpal::Sample::from::<f32>(&value);
                for sample in frame.iter_mut() {
                    *sample = value;
                }
            }
        },
        err_fn,
    )?;
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_secs_f32(duration));

    Ok(())
}

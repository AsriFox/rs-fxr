use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rs_fxr::traits::Synth;
use std::io::Read;

fn main() {
    let mut args = std::env::args();
    let _ = args.next().unwrap();
    let s = match args.next() {
        Some(path) => match std::fs::File::open(&path) {
            Ok(mut file) => {
                let mut buf = String::new();
                file.read_to_string(&mut buf).unwrap();
                buf
            }
            Err(err) => {
                eprintln!("Could not open file '{}': {}", path, err);
                return;
            }
        },
        None => {
            eprintln!("No JFXR file provided");
            return;
        }
    };
    let synth = match rs_fxr::serde::json::parse_str(s.as_str()) {
        Ok(description) => description.build().unwrap(),
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No output device found");
    let config = device.default_output_config().unwrap();

    if let Err(err) = match config.sample_format() {
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into(), synth),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into(), synth),
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into(), synth),
    } {
        eprintln!("{}", err);
    }
}

pub fn run<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    mut synth: Box<dyn Synth>,
) -> Result<(), anyhow::Error>
where
    T: cpal::Sample,
{
    // let sample_rate = config.sample_rate.0 as f64;
    let channels = config.channels as usize;

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let duration = synth.duration();

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            for frame in data.chunks_mut(channels) {
                let value = if let Some(value) = synth.next() {
                    value as f32
                } else {
                    0.
                };
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

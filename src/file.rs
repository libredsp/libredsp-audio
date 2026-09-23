use hound::{SampleFormat, WavReader, WavSpec, WavWriter};
use libredsp::signal::Signal;
use std::path::Path;

pub struct Audio {
    pub signal: Signal,
    pub spec: WavSpec,
}

pub fn read_file(input: &Path) -> Result<Audio, Box<dyn std::error::Error>> {
    if !input.is_file() {
        return Err(format!("input file not found: {}", input.display()).into());
    }

    let mut reader = WavReader::open(input)?;
    let spec = reader.spec();

    let channels = spec.channels as usize;

    if channels == 0 {
        return Err("WAV file has zero channels".into());
    }

    let signal: Vec<f64> = match spec.sample_format {
        SampleFormat::Int => reader
            .samples::<i32>()
            .map(|sample| sample.map(|x| x as f64))
            .collect::<Result<Vec<_>, _>>()?,

        SampleFormat::Float => reader
            .samples::<f32>()
            .map(|sample| sample.map(|x| x as f64))
            .collect::<Result<Vec<_>, _>>()?,
    };

    Ok(Audio {
        signal: Signal::new(signal),
        spec,
    })
}

pub fn write_file(
    output: &Path,
    signal: &Signal,
    spec: WavSpec,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = WavWriter::create(output, spec)?;
    let data = signal.to_vec();

    match spec.sample_format {
        SampleFormat::Int => {
            let (min, max) = match spec.bits_per_sample {
                8 => (i8::MIN as f64, i8::MAX as f64),
                16 => (i16::MIN as f64, i16::MAX as f64),
                24 => (-(1_i64 << 23) as f64, ((1_i64 << 23) - 1) as f64),
                32 => (i32::MIN as f64, i32::MAX as f64),
                bits => {
                    return Err(format!("unsupported integer WAV bit depth: {}", bits).into());
                }
            };

            for sample in data {
                let sample = sample.clamp(min, max);

                match spec.bits_per_sample {
                    8 => writer.write_sample(sample as i8)?,
                    16 => writer.write_sample(sample as i16)?,
                    24 | 32 => writer.write_sample(sample as i32)?,
                    _ => unreachable!(),
                }
            }
        }

        SampleFormat::Float => {
            for sample in data {
                writer.write_sample(sample as f32)?;
            }
        }
    }

    writer.finalize()?;

    Ok(())
}

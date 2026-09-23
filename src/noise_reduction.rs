use crate::file::{read_file, write_file};
use libredsp::noise_reduction::spectral_subtraction;
use libredsp::signal::Signal;
use std::path::Path;

pub fn fft_based_noise_removal(
    input: &Path,
    output: &Path,
    noise_start: f64,
    noise_end: f64,
    frame_len_ms: f64,
    alpha: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let audio = read_file(input)?;

    let signal = &audio.signal;
    let spec = audio.spec;

    let channels = spec.channels as usize;
    let sample_rate = spec.sample_rate as usize;

    let mut frame_size = ((frame_len_ms / 1000.0) * sample_rate as f64) as usize;

    if frame_size == 0 {
        return Err("frame length is too small".into());
    }

    if frame_size % 2 != 0 {
        frame_size += 1;
    }

    let noise_start_sample = (noise_start * sample_rate as f64) as usize;
    let noise_end_sample = (noise_end * sample_rate as f64) as usize;

    let mut channel_signals = Vec::with_capacity(channels);

    for channel in 0..channels {
        let channel_data: Vec<f64> = signal.iter().skip(channel).step_by(channels).collect();

        channel_signals.push(Signal::new(channel_data));
    }

    let mut clean_channels = Vec::with_capacity(channels);

    for channel_signal in &channel_signals {
        let clean_signal = spectral_subtraction(
            channel_signal,
            frame_size,
            noise_start_sample,
            noise_end_sample,
            alpha,
        )?;

        clean_channels.push(clean_signal);
    }

    let num_samples = clean_channels[0].len();
    let mut interleaved = Vec::with_capacity(num_samples * channels);

    for i in 0..num_samples {
        for channel in 0..channels {
            interleaved.push(clean_channels[channel][i]);
        }
    }

    let clean_signal = Signal::new(interleaved);

    write_file(output, &clean_signal, spec)?;

    Ok(())
}

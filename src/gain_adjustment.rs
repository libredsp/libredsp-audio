use crate::file::{read_file, write_file};
use libredsp::signal::Signal;

use std::path::Path;

pub fn gain_adjustment(
    input: &Path,
    output: &Path,
    gain: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let audio = read_file(input)?;

    let signal = &audio.signal;
    let spec = audio.spec;

    let processed_signal: Signal = signal.iter().map(|sample| gain * sample).collect();

    write_file(output, &processed_signal, spec)?;

    Ok(())
}

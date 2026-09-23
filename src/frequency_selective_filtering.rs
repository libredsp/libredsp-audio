use crate::file::{read_file, write_file};
use libredsp::{TransferFunction, signal::Signal};
use std::path::Path;

pub fn filter(
    input: &Path,
    output: &Path,
    filter_coef: TransferFunction,
) -> Result<(), Box<dyn std::error::Error>> {
    let audio = read_file(input)?;

    let signal = &audio.signal;
    let spec = audio.spec;

    let processed_signal: Signal = signal.filter(&filter_coef);
    write_file(output, &processed_signal, spec)?;

    Ok(())
}

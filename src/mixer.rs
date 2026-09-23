use crate::file::{read_file, write_file};

use std::path::Path;

pub fn mixer(
    input1: &Path,
    input2: &Path,
    output: &Path,
    pos_in_sec: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let audio1 = read_file(input1)?;
    let signal1 = &audio1.signal;
    let spec1 = audio1.spec;

    let audio2 = read_file(input2)?;
    let signal2 = &audio2.signal;
    let spec2 = audio2.spec;

    assert_eq!(spec1.sample_rate, spec2.sample_rate);

    /* Normally, we would do
     * let pos_in_index = (pos_in_sec * spec1.sample_rate as f64) as usize;
     * But if we have to channels, the samples for each mono channel is one after the other, like r_0, l_0, r_1, l_1, ...
     * So, we need to account for that.
     */
    let pos_in_index = (pos_in_sec * spec1.sample_rate as f64) as usize * spec1.channels as usize;

    let mut processed_signal = signal1.clone();

    for (i, sample) in signal2.iter().enumerate() {
        let index = pos_in_index + i;

        if index >= processed_signal.len() {
            break;
        }

        processed_signal[index] += sample;
    }

    write_file(output, &processed_signal, spec1)?;

    Ok(())
}

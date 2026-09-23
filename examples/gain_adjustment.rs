/* Usage: cargo run --example gain_adjustment -- <input.wav> <output.wav> <gain> */

use libredsp_audio::gain_adjustment;
use std::env;
use std::path::Path;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <input.wav> <output.wav> <gain>", args[0]);
        exit(1);
    }

    let input = Path::new(&args[1]);
    let output = Path::new(&args[2]);

    let gain: f64 = args[3].parse().unwrap_or_else(|_| {
        eprintln!("Invalid gain: {}", args[3]);
        exit(1);
    });

    let input_spec = hound::WavReader::open(input)
        .unwrap_or_else(|e| {
            eprintln!("Failed to open input file {:?}: {}", input, e);
            exit(1);
        })
        .spec();

    gain_adjustment(input, output, gain).unwrap();

    assert!(output.exists());

    let reader = hound::WavReader::open(output).unwrap();
    let output_spec = reader.spec();

    assert_eq!(output_spec.channels, input_spec.channels);
    assert_eq!(output_spec.sample_rate, input_spec.sample_rate);
}

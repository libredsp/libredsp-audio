/* Usage: cargo run --example background_noise_removal -- <input.wav> <output.wav> <noise_start> <noise_end> */

use libredsp_audio::noise_reduction::fft_based_noise_removal;
use std::env;
use std::path::Path;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!(
            "Usage: {} <input.wav> <output.wav> <noise_start> <noise_end>",
            args[0]
        );
        exit(1);
    }

    let input = Path::new(&args[1]);
    let output = Path::new(&args[2]);

    let noise_start: f64 = args[3].parse().unwrap_or_else(|_| {
        eprintln!("Invalid noise start time: {}", args[3]);
        exit(1);
    });

    let noise_end: f64 = args[4].parse().unwrap_or_else(|_| {
        eprintln!("Invalid noise end time: {}", args[4]);
        exit(1);
    });

    let input_spec = hound::WavReader::open(input)
        .unwrap_or_else(|e| {
            eprintln!("Failed to open input file {:?}: {}", input, e);
            exit(1);
        })
        .spec();

    fft_based_noise_removal(
        input,
        output,
        noise_start,
        noise_end,
        30.0, // frame length in ms
        2.0,  // alpha
    )
    .unwrap();

    assert!(output.exists());

    let reader = hound::WavReader::open(output).unwrap();

    assert_eq!(reader.spec().sample_rate, input_spec.sample_rate);
    assert_eq!(reader.spec().channels, input_spec.channels);
}

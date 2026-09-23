/* Usage: cargo run --example mixer -- <input1.wav> <input2.wav> <output.wav> <position_in_seconds> */

use libredsp_audio::mixer;
use std::env;
use std::path::Path;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!(
            "Usage: {} <input1.wav> <input2.wav> <output.wav> <position_in_seconds>",
            args[0]
        );
        exit(1);
    }

    let input1 = Path::new(&args[1]);
    let input2 = Path::new(&args[2]);
    let output = Path::new(&args[3]);

    let pos_in_sec: f64 = args[4].parse().unwrap_or_else(|_| {
        eprintln!("Invalid position: {}", args[4]);
        exit(1);
    });

    let input1_spec = hound::WavReader::open(input1)
        .unwrap_or_else(|e| {
            eprintln!("Failed to open input file {:?}: {}", input1, e);
            exit(1);
        })
        .spec();

    let input2_spec = hound::WavReader::open(input2)
        .unwrap_or_else(|e| {
            eprintln!("Failed to open input file {:?}: {}", input2, e);
            exit(1);
        })
        .spec();

    assert_eq!(input1_spec.sample_rate, input2_spec.sample_rate);
    assert_eq!(input1_spec.channels, input2_spec.channels);

    mixer(input1, input2, output, pos_in_sec).unwrap();

    assert!(output.exists());

    let reader = hound::WavReader::open(output).unwrap();

    assert_eq!(reader.spec().channels, input1_spec.channels);
    assert_eq!(reader.spec().sample_rate, input1_spec.sample_rate);
    assert_eq!(reader.spec().channels, input1_spec.channels);
}

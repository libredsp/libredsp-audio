/* Usage: cargo run --example frequency_selective_filtering -- <input.wav> <output.wav> */
use libredsp::TransferFunction;
use libredsp_audio::filter;
use std::env;
use std::path::Path;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input.wav> <output.wav>", args[0]);
        exit(1);
    }

    let input = Path::new(&args[1]);
    let output = Path::new(&args[2]);

    let num = vec![
        0.0141, 0.0244, 0.0363, 0.0491, 0.0618, 0.0731, 0.0821, 0.0879, 0.0899, 0.0879, 0.0821,
        0.0731, 0.0618, 0.0491, 0.0363, 0.0244, 0.0141,
    ];

    let den = vec![1.0];

    let filter_equation = TransferFunction::new(num, den);

    filter(input, output, filter_equation).unwrap();

    assert!(output.exists());
}

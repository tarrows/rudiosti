use hound;
use rudiosti::cli::opts;
use rudiosti::io::{read, write};

fn main() {
  let args = opts().get_matches();
  let input = args.value_of("INPUT").unwrap();
  let _ = read(input); // "data/example.wav"

  let spec = hound::WavSpec {
    channels: 1,
    sample_rate: 44100,
    bits_per_sample: 16,
    sample_format: hound::SampleFormat::Int,
  };

  write(spec).unwrap();
}

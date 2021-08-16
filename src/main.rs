use rudiosti::cli::opts;
use rudiosti::io::read;

fn main() {
  let args = opts().get_matches();
  let input = args.value_of("INPUT").unwrap();
  let _ = read(input); // "data/example.wav"
}

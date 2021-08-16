use hound;
use std::f32::consts::PI;
use std::i16;

pub fn read(file_path: &str) -> Result<(), hound::Error> {
  let mut reader = hound::WavReader::open(file_path)?;

  let sqr_sum = reader.samples::<i16>().fold(0.0, |sqr_sum, s| {
    let sample = s.unwrap() as f64;
    sqr_sum + sample * sample
  });

  println!("RMS is {}", (sqr_sum / reader.len() as f64).sqrt());

  Ok(())
}

pub fn write(spec: hound::WavSpec) -> Result<(), hound::Error> {
  let mut writer = hound::WavWriter::create("sine.wav", spec)?;

  for t in (0..spec.sample_rate).map(|x| x as f32 / spec.sample_rate as f32) {
    let sample = (t * 440.0 * 2.0 * PI).sin();
    let amplitude = i16::MAX as f32;
    writer.write_sample((sample * amplitude) as i16)?;
  }

  writer.finalize()?;

  Ok(())
}

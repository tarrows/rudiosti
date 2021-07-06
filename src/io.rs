use hound;

pub fn read(file_path: &str) -> Result<(), hound::Error> {
  let mut reader = hound::WavReader::open(file_path)?;

  let sqr_sum = reader.samples::<i16>().fold(0.0, |sqr_sum, s| {
    let sample = s.unwrap() as f64;
    sqr_sum + sample * sample
  });

  println!("RMS is {}", (sqr_sum / reader.len() as f64).sqrt());

  Ok(())
}

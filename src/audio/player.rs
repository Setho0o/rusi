use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
pub fn player(file: &str) {
  let (_stream, stream_handle) = OutputStream::try_default().unwrap();
  let sink = Sink::try_new(&stream_handle).unwrap();
  let source = Decoder::new(BufReader::new(File::open(file).unwrap())).unwrap();
  sink.append(source);
  sink.sleep_until_end();
}

mod reader;
use std::sync::Arc;

fn main() {
    let wav = include_bytes!("../resources/test.wav");
    println!("Wave length in bytes: {}", wav.len());
    let wav = wav.as_slice();
    let wav = Arc::from(wav);

    let mut reader: reader::PcmReader = Default::default();
    reader.read_bytes(wav);
}

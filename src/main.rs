mod reader;

fn main() {
    let wav = include_bytes!("../resources/test.wav");
    println!("Wave length in bytes: {}", wav.len());
    let mut reader: reader::PcmReader = Default::default();
    reader.read_bytes(wav);
}

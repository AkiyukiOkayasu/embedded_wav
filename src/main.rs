mod reader;

fn main() {
    let wav = include_bytes!("../resources/test.wav");
    println!("Wave length in bytes: {}", wav.len());

    let reader = reader::PcmReader::read_bytes(wav).unwrap();
}

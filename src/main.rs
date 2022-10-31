mod reader;

fn main() {
    let wav = include_bytes!("../resources/test.wav");
    println!("Wave length in bytes: {}", wav.len());

    let reader = reader::PcmReader::read_bytes(wav);
    for i in 0..=100 {
        println!("{}", reader.read_sample(0, i).unwrap());
    }
}

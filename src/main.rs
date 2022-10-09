use nom::bytes::complete::tag;
use nom::number::complete::le_u32;
use nom::IResult;

#[derive(Debug)]
struct RiffChunk {
    size: u32,
}

fn riff(input: &[u8]) -> IResult<&[u8], RiffChunk> {
    let (input, _) = tag(b"RIFF")(input)?;
    let (input, size) = le_u32(input)?;

    Ok((input, RiffChunk { size }))
}

fn verify_wav(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (input, wave) = tag(b"WAVE")(input)?;
    Ok((input, wave))
}

fn main() {
    let wav = include_bytes!("../resources/test.wav");
    let file_length = wav.len();
    println!("{}", wav.len());
    let (wav, riff) = riff(wav).unwrap();
    println!("{}", riff.size);
    assert_eq!((file_length - 8) as u32, riff.size);
    let (input, _) = verify_wav(wav).unwrap();
    println!("{}", input.len());
}

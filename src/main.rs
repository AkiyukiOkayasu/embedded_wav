use nom::bytes::complete::{tag, take};
use nom::number::complete::{le_u16, le_u32};
use nom::IResult;

#[derive(Debug, PartialEq)]
enum Format {
    Wave,
    Non,
}

#[derive(Debug)]
struct RiffChunk {
    size: u32,
    format: Format,
}

#[derive(Debug)]
enum AudioFormat {
    LinearPcm,
    IeeeFloat,
    ALow,
    ULow,
    Non,
}

#[derive(Debug)]
struct FmtChunk {
    audio_format: AudioFormat,
    num_channels: u16,
    sample_rate: u32,
    data_size_per_seconds: u32,
    block_size: u16,
    bit_depth: u16,
}

fn verify_riff(input: &[u8]) -> IResult<&[u8], RiffChunk> {
    let (input, _) = tag(b"RIFF")(input)?;
    let (input, size) = le_u32(input)?;
    let (input, id_str) = take(4usize)(input)?;

    let format: Format = match id_str {
        b"WAVE" => Format::Wave,
        _ => Format::Non,
    };

    Ok((input, RiffChunk { size, format }))
}

///fmtチャンクを検査します  
///
/// * 'input' - テスト
fn verify_fmt(input: &[u8]) -> IResult<&[u8], FmtChunk> {
    let (input, _) = tag(b"fmt ")(input)?;
    let (input, chunk_size) = le_u32(input)?;
    assert_eq!(chunk_size, 16);

    let (input, format) = le_u16(input)?;
    let audio_format: AudioFormat = match format {
        1 => AudioFormat::LinearPcm,
        3 => AudioFormat::IeeeFloat,
        6 => AudioFormat::ALow,
        7 => AudioFormat::ULow,
        _ => AudioFormat::Non,
    };

    let (input, num_channels) = le_u16(input)?;
    let (input, sample_rate) = le_u32(input)?;
    let (input, data_size_per_seconds) = le_u32(input)?;
    let (input, block_size) = le_u16(input)?;
    let (input, bit_depth) = le_u16(input)?;

    Ok((
        input,
        FmtChunk {
            audio_format,
            num_channels,
            sample_rate,
            data_size_per_seconds,
            block_size,
            bit_depth,
        },
    ))
}

fn main() {
    let wav = include_bytes!("../resources/test.wav");
    let file_length = wav.len();
    println!("{}", wav.len());
    let (wav, riff) = verify_riff(wav).unwrap();
    println!("{}", riff.size);
    assert_eq!(riff.format, Format::Wave);
    assert_eq!((file_length - 8) as u32, riff.size);

    let (input, fmt) = verify_fmt(wav).unwrap();
    println!("{:?}", fmt.audio_format);
}

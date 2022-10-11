use nom::bytes::complete::{tag, take};
use nom::number::complete::{le_u16, le_u32};
use nom::IResult;

#[derive(Debug, PartialEq)]
enum RiffIdentifier {
    Wave, //b"WAVE"
    Avi,  //b"AVI "
    Unknown,
}

/// RIFFチャンクの情報
///
/// * 'size' - ファイルサイズ(byte) - 8
/// * 'id' - RIFFの識別子 基本"WAVE"
#[derive(Debug)]
struct RiffChunk {
    size: u32,
    id: RiffIdentifier,
}

/// chunkの種類
///
/// * "fmt " - 必須チャンク
/// * "fact" - optional
/// * "PEAK" - optional
/// * "data" - 必須チャンク
#[derive(Debug)]
enum Chunk {
    Fmt,  // b"fmt "
    Fact, // b"fact"
    PEAK, // b"PEAK"
    Data, // b"data"
}

/// Waveの形式
/// LinearPCMとIEEE FloatとIMA ADPCMくらいしか使わないはず
/// https://github.com/tpn/winsdk-10/blob/9b69fd26ac0c7d0b83d378dba01080e93349c2ed/Include/10.0.14393.0/shared/mmreg.h#L2107-L2372
#[derive(Debug)]
enum WaveFormatTag {
    Unknown = 0x00,        //0
    LinearPcm = 0x01,      //1
    MicrosoftAdpcm = 0x02, //2
    IeeeFloat = 0x03,      //3
    ALaw = 0x06,           //6
    MuLaw = 0x07,          //7
    OkiAdpcm = 0x10,       //0x10
    ImaAdpcm = 0x11,       //0x11 aka DVI ADPCM
    YamahaAdpcm = 0x20,    //0x20
    Flac = 0xF1AC,         //0xF1AC
}

/// Fmtチャンク構造体  
///
/// 必須チャンク
/// https://www.youfit.co.jp/archives/1418
#[derive(Debug)]
struct FmtChunk {
    wave_format_tag: WaveFormatTag,
    num_channels: u16,
    sample_rate: u32,
    bytes_per_seconds: u32, //sampleRate * num_channels * (bit_depth / 8)
    block_size: u16,
    bit_depth: u16,
}

/// factチャンク
///
/// optionalのチャンク
/// https://www.g200kg.com/jp/docs/tech/wavfile.html
///
/// * 'dw_sample_length' - dataチャンクに記録されている1ch当たりのサンプル数
#[derive(Debug)]
struct FactChunk {
    dw_sample_length: u32,
}

/// PEAKチャンク
///
/// optionalのチャンク
/// https://www.g200kg.com/jp/docs/tech/wavfile.html
///
/// peak_levelとposition_peakはチャンネルごとに作られるのでtemplate的にチャンネル数の変更に対応するべきかも
#[derive(Debug)]
struct PeakChunk {
    version: u32,           // 1固定
    time_stamp: u32,        // 1970/1/1からの秒数
    peak_level_1ch: f32,    // 0dBFSを1.0とする符号付きfloat値
    position_peak_1ch: u32, // 開始位置からのサンプル数
}

/// ファイルがRIFFから始まり、識別子がWAVEであることのチェック
fn verify_riff(input: &[u8]) -> IResult<&[u8], RiffChunk> {
    let (input, _) = tag(b"RIFF")(input)?;
    let (input, size) = le_u32(input)?;
    let (input, id_str) = take(4usize)(input)?;

    let id: RiffIdentifier = match id_str {
        b"WAVE" => RiffIdentifier::Wave,
        b"AVI " => RiffIdentifier::Avi,
        _ => RiffIdentifier::Unknown,
    };

    Ok((input, RiffChunk { size, id }))
}

/// fmtチャンクを検査します  
///
/// * 'input' - テスト
fn verify_fmt(input: &[u8]) -> IResult<&[u8], FmtChunk> {
    let (input, _) = tag(b"fmt ")(input)?;
    let (input, chunk_size) = le_u32(input)?;
    assert_eq!(chunk_size, 16);

    let (input, format) = le_u16(input)?;
    let wave_format_tag: WaveFormatTag = match format {
        0 => WaveFormatTag::Unknown,
        1 => WaveFormatTag::LinearPcm,
        2 => WaveFormatTag::MicrosoftAdpcm,
        3 => WaveFormatTag::IeeeFloat,
        6 => WaveFormatTag::ALaw,
        7 => WaveFormatTag::MuLaw,
        0x10 => WaveFormatTag::OkiAdpcm,
        0x11 => WaveFormatTag::ImaAdpcm,
        0x20 => WaveFormatTag::YamahaAdpcm,
        0xF1AC => WaveFormatTag::Flac,
        _ => WaveFormatTag::Unknown,
    };

    let (input, num_channels) = le_u16(input)?;
    let (input, sample_rate) = le_u32(input)?;
    let (input, bytes_per_seconds) = le_u32(input)?;
    let (input, block_size) = le_u16(input)?;
    let (input, bit_depth) = le_u16(input)?;

    Ok((
        input,
        FmtChunk {
            wave_format_tag,
            num_channels,
            sample_rate,
            bytes_per_seconds,
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
    assert_eq!(riff.id, RiffIdentifier::Wave);
    assert_eq!((file_length - 8) as u32, riff.size);

    let (input, fmt) = verify_fmt(wav).unwrap();
    println!("{:?}", fmt);
}

use nom::{multi::many1, IResult};
use std::sync::Arc;

mod wav;

#[derive(Debug, Default)]
enum AudioFormat {
    #[default]
    Unknown,
    LinearPcmLe,
    LinearPcmBe,
    IeeeFloat,
    ALaw,
    MuLaw,
    ImaAdpcm,
}

#[derive(Default, Debug)]
pub struct PcmSpecs {
    audio_format: AudioFormat,
    num_channels: u16,
    sample_rate: u32,
    bit_depth: u16,
}

/// dataとwavとcは将来的にどれかに集約される。
/// read_bytes()のinputの型が決定したら再検討する。
#[derive(Default)]
pub struct PcmReader<'a> {
    specs: PcmSpecs,
    data: &'a [u8],
    wav: Arc<&'a [u8]>,
    c: wav::Chunk<'a>,
}

impl<'a> PcmReader<'a> {
    fn parse_aiff(&mut self, input: &'a [u8]) -> IResult<&[u8], &[u8]> {
        todo!(); // Ok((input, input))
    }

    fn parse_wav(&mut self, input: &'a [u8]) -> IResult<&[u8], &[u8]> {
        let (_input, v) = many1(wav::parse_chunk)(input)?;
        for e in v {
            match e.id {
                wav::ChunkId::Fmt => {
                    println!("fmt");
                    let (_, spec) = wav::parse_fmt(e.data)?;
                    // println!("{:?}", spec);
                    self.specs = spec;
                }
                wav::ChunkId::Data => {
                    println!("Data");
                    self.data = e.data;
                    self.c = e;
                }
                wav::ChunkId::Fact => println!("fact"),
                wav::ChunkId::IDv3 => println!("IDv3"),
                wav::ChunkId::JUNK => println!("JUNK"),
                wav::ChunkId::LIST => println!("LIST"),
                wav::ChunkId::PEAK => println!("PEAK"),
                wav::ChunkId::Unknown => println!("Unknown"),
            }
        }

        return Ok((&[], &[]));
    }

    /// WAVのByte配列をパースし、再生できるように準備する。
    /// inputを
    /// * Arc<&[u8]>
    /// * Arc<[u8; size]>
    /// * &[u8]
    /// のどれにするか検討中。
    /// PcmReaderがいつ破棄されるかは再生時にしか決められない場合があるのでArcを使うべきだと思うが、スライスだと結局ライフタイムの問題がある。
    /// 配列だと長さがコンパイル時に決められない。どう書くのがRust的に良いかを探っている。
    /// これをPcmReaderのnew()相当の初期化関数とするべきかもしれない。
    pub fn read_bytes(&mut self, input: Arc<&'a [u8]>) -> IResult<&[u8], &[u8]> {
        let file_length = input.len();
        self.wav = input;

        //TODO WAVかAIFFか判定
        if let Ok((input, riff)) = wav::parse_riff_header(*self.wav) {
            println!("Riff size: {}", riff.size);
            assert_eq!(riff.id, wav::RiffIdentifier::Wave);
            assert_eq!((file_length - 8) as u32, riff.size);
            self.parse_wav(input)?;
        };

        todo!();
        //Ok((input, PcmReader {}))
    }
}

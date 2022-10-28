use nom::{multi::many1, IResult};

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
}

impl<'a> PcmReader<'a> {
    fn parse_aiff(&mut self, input: &'a [u8]) -> IResult<&[u8], &[u8]> {
        todo!(); // Ok((input, input))
    }

    fn parse_wav(&mut self, input: &'a [u8]) -> IResult<&[u8], &[u8]> {
        //many1はallocが実装されていないと使えない。no_stdで使うなら逐次的に実行するべき。
        let (_input, v) = many1(wav::parse_chunk)(input)?;

        for e in v {
            match e.id {
                wav::ChunkId::Fmt => {
                    println!("fmt");
                    let (_, spec) = wav::parse_fmt(e.data)?;
                    println!("{:?}", spec);
                    self.specs = spec;
                }
                wav::ChunkId::Data => {
                    println!("Data");
                    self.data = e.data;
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
    /// 少なくともinputとPcmReaderのlifetimeの長さがinput>PcmReaderであればよい。    
    /// http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch19-02-advanced-lifetimes.html
    /// また、配列だと長さがコンパイル時に決められない。ジェネリクスで書くのか、どう書くのがRust的に良いかを探っている。
    /// これをPcmReaderのnew()相当の初期化関数とするべきかもしれない。
    pub fn read_bytes(input: &'a [u8]) -> Self {
        let file_length = input.len();

        let mut reader: PcmReader = Default::default();

        //WAVの場合
        if let Ok((input, riff)) = wav::parse_riff_header(input) {
            println!("Riff length in bytes: {}", riff.size);
            assert_eq!(riff.id, wav::RiffIdentifier::Wave);
            assert_eq!((file_length - 8) as u32, riff.size);
            if let Ok((_, _)) = reader.parse_wav(input) {
                return reader;
            }
        };

        //AIFFの場合
        todo!();

        //WAVでもAIFFでもなかった場合
        panic!();
    }
}

use nom::IResult;
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
        let (input, chunk) = wav::parse_chunk(input)?;
        match chunk.id {
            wav::ChunkId::Fmt => {
                let (_, spec) = wav::parse_fmt(chunk.data)?;
                println!("{:?}", spec);
                self.specs = spec;
            }
            wav::ChunkId::Data => {
                self.data = chunk.data;
                self.c = chunk;
            }
            wav::ChunkId::Fact => println!("fact"),
            wav::ChunkId::IDv3 => println!("IDv3"),
            wav::ChunkId::JUNK => println!("JUNK"),
            wav::ChunkId::LIST => println!("LIST"),
            wav::ChunkId::PEAK => println!("PEAK"),
            wav::ChunkId::Unknown => println!("Unknown"),
        }
        // println!("{:?}", chunk);

        todo!();
    }

    pub fn read_bytes(&mut self, input: Arc<&'a [u8]>) -> IResult<&[u8], &[u8]> {
        let file_length = input.len();
        self.wav = input;

        //TODO WAVかAIFFか判定
        if let Ok((input, riff)) = wav::parse_riff_header(*self.wav) {
            println!("Riff size: {}", riff.size);
            assert_eq!(riff.id, wav::RiffIdentifier::Wave);
            assert_eq!((file_length - 8) as u32, riff.size);
            self.parse_wav(input);
        };

        todo!();
        //Ok((input, PcmReader {}))
    }
}

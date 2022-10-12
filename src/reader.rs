use nom::IResult;

mod wav;

#[derive(Default)]
pub struct PcmSpecs {
    num_channels: u16,
    sample_rate: u32,
    bit_depth: u16,
}

#[derive(Default)]
pub struct PcmReader {
    specs: PcmSpecs,
}

impl PcmReader {
    fn parse_aiff(input: &[u8]) -> IResult<&[u8], &[u8]> {
        Ok((input, input))
    }

    fn parse_wav(input: &[u8]) -> IResult<&[u8], &[u8]> {
        let (input, chunk) = wav::parse_chunk(input)?;
        match chunk.id {
            wav::ChunkId::Fmt => {
                wav::parse_fmt(chunk.data);
            }
            wav::ChunkId::Data => println!("data"),
            wav::ChunkId::Fact => println!("fact"),
            wav::ChunkId::IDv3 => println!("IDv3"),
            wav::ChunkId::JUNK => println!("JUNK"),
            wav::ChunkId::LIST => println!("LIST"),
            wav::ChunkId::PEAK => println!("PEAK"),
            wav::ChunkId::Unknown => println!("Unknown"),
        }
        println!("{:?}", chunk);

        todo!();
    }

    pub fn read_bytes(input: &[u8]) -> IResult<&[u8], PcmReader> {
        let file_length = input.len();

        //TODO WAVかAIFFか判定
        if let Ok((input, riff)) = wav::parse_riff_header(input) {
            println!("{}", riff.size);
            assert_eq!(riff.id, wav::RiffIdentifier::Wave);
            assert_eq!((file_length - 8) as u32, riff.size);
            PcmReader::parse_wav(input);
        };

        todo!();
        //Ok((input, PcmReader {}))
    }
}

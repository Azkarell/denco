use rayon::prelude::*;
use crate::{BaseAsciiConverter, Config, Decoder, Encoder};

pub struct ParallelBase64<C: BaseAsciiConverter> {
    pub config: Config,
    pub converter: C,
}

impl<C: BaseAsciiConverter + Send + Sync> Encoder for ParallelBase64<C> {
    fn encode(&self, input: &[u8]) -> String {
        let mut len = input.len() * 4 / 3;
        while len % 4 != 0 {
            len += 1;
        }
        let mut buffer: Vec<u8> = vec![0; len];
        buffer.par_chunks_mut(4).zip(input.par_chunks(3))
            .for_each(|(i, chunk)| {
                let mask1 = 0b0000_0011;
                let mask2 = 0b0000_1111;
                self.converter.base_to_ascii_with_index((chunk[0]) >> 2, i, &self.config, 0);

                if chunk.len() == 1 {
                    self.converter.base_to_ascii_with_index((chunk[0] & mask1) << 4, i, &self.config, 1);
                    i[2] = self.config.fill_marker[0];
                    i[3] = self.config.fill_marker[0];
                    return;
                }
                self.converter.base_to_ascii_with_index(((chunk[0] & mask1) << 4) | (chunk[1] >> 4), i, &self.config, 1);

                if chunk.len() == 2 {
                    self.converter.base_to_ascii_with_index((chunk[1] & mask2) << 2, i, &self.config, 2);
                    i[3] = self.config.fill_marker[0];
                    return;
                }
                self.converter.base_to_ascii_with_index(((chunk[1] & mask2) << 2) | (chunk[2] >> 6), i, &self.config, 2);
                self.converter.base_to_ascii_with_index(chunk[2] & 0b0011_1111, i, &self.config, 3);
            });


        unsafe {
            String::from_utf8_unchecked(buffer)
        }
    }
}

impl<C: BaseAsciiConverter + Send + Sync> Decoder for ParallelBase64<C> {
    fn decode(&self, input: &str) -> Vec<u8> {
        let bytes = input.as_bytes();
        let mut len = bytes.len() * 3 / 4;

        while len % 3 != 0 {
            len += 1;
        }


        let mut buffer = vec![u8::MAX; len];

        buffer.par_chunks_mut(3).zip(bytes.par_chunks(4)).for_each(
            |(dest, src)| {
                if src[1] == self.config.fill_marker[0] {

                }
                dest[0] = (self.converter.ascii_to_base_non_failing(&src[0], &self.config) << 2 |
                    self.converter.ascii_to_base_non_failing(&src[1], &self.config) >> 4) as u8;
                dest[1] = (self.converter.ascii_to_base_non_failing(&src[1], &self.config) << 4 |
                    self.converter.ascii_to_base_non_failing(&src[2], &self.config) >> 2) as u8;
                dest[2] = (self.converter.ascii_to_base_non_failing(&src[2], &self.config) << 6 |
                    self.converter.ascii_to_base_non_failing(&src[3], &self.config)) as u8;
            }
        );

        match self.converter.num_unexpected_chars() {
            0 => buffer,
            1 => {
                buffer.pop();
                buffer
            }
            2 => {
                buffer.pop();
                buffer
            }
            3 => {
                buffer.pop();
                buffer.pop();
                buffer
            }
            _ => panic!("Unexpected number of unexpected chars"),
        }
    }
}

impl<C: BaseAsciiConverter + Send + Sync> ParallelBase64<C> {
    pub fn new(config: Config) -> Self {
        let c = C::from_config(&config);
        Self {
            config,
            converter: c,
        }
    }
}

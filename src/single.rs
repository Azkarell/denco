use crate::{BaseAsciiConverter, Config, Encoder, Decoder};

pub struct Base64<C: BaseAsciiConverter> {
    config: Config,
    converter: C,
}

impl<C: BaseAsciiConverter> Encoder for Base64<C> {
    fn encode(&self, input: &[u8]) -> String {
        let mut buffer = Vec::with_capacity(f32::ceil(input.len() as f32 * 4.0 / 3.0) as usize);
        let mut input_split_index = 0;
        let mut input_index = 0;
        let mut paddings = 0u8;
        while input_index < input.len() {
            input_index += 1;
            if (input_index - input_split_index) == 3 {
                self.encode_triplet(&input[input_split_index..input_index], paddings, &mut buffer);
                input_split_index = input_index;
            }
        }
        let left = (input_index % 3) as usize;
        if left != 0 {
            paddings = (3 - left) as u8;
            let mut data = vec![0u8; 3];
            for i in 0..left as usize {
                data[i] = input[input_split_index + i];
            }
            self.encode_triplet(&data, paddings, &mut buffer);
        }

        unsafe {
            String::from_utf8_unchecked(buffer)
        }
    }


}

impl<C: BaseAsciiConverter> Decoder for Base64<C> {

    fn decode(&self, input: &str) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(f32::ceil(input.len() as f32 / 3.0 * 4.0) as usize);
        let input_bytes = input.as_bytes();

        let mut input_split_index = 0;
        let mut input_index = 0;
        while input_index < input_bytes.len() {
            input_index += 1;
            if (input_index - input_split_index) == 4 {
                self.decode_quad(&input_bytes[input_split_index..input_index], &mut buffer);
                input_split_index = input_index;
            }
        }
        if input_index % 4 != 0 {
            let mut data = vec![u8::MAX; 4];
            data[0..(input_index % 4)].copy_from_slice(&input_bytes[input_split_index..input_index]);
            self.decode_quad(&data, &mut buffer);
        }

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


impl<C: BaseAsciiConverter> Base64<C> {
    pub fn new(config: Config) -> Self {
        let conv = C::from_config(&config);
        Base64 {
            config,
            converter: conv,
        }
    }

    #[inline]
    fn encode_triplet(&self, val: &[u8], paddings: u8, buffer: &mut Vec<u8>) {
        let mask1 = 0b0000_0011;
        let mask2 = 0b0000_1111;
        self.converter.base_to_ascii((val[0]) >> 2, buffer, &self.config);
        self.converter.base_to_ascii(((val[0] & mask1) << 4) | (val[1] >> 4), buffer, &self.config);
        if paddings == 2u8 { buffer.append(self.config.fill_marker.to_vec().as_mut()) } else { self.converter.base_to_ascii(((val[1] & mask2) << 2) | (val[2] >> 6), buffer, &self.config) };
        if paddings == 2u8 || paddings == 1u8 { buffer.append(self.config.fill_marker.to_vec().as_mut()) } else { self.converter.base_to_ascii(val[2] & 0b0011_1111, buffer, &self.config) };
    }


    #[inline]
    fn decode_quad(&self, input: &[u8], buffer: &mut Vec<u8>) {
        buffer.push(  self.converter.ascii_to_base_non_failing(&input[0], &self.config) << 2 | self.converter.ascii_to_base_non_failing(&input[1], &self.config) >> 4);
        buffer.push(self.converter.ascii_to_base_non_failing(&input[1], &self.config) << 4 | self.converter.ascii_to_base_non_failing(&input[2], &self.config) >> 2);
        buffer.push(self.converter.ascii_to_base_non_failing(&input[2], &self.config) << 6 | self.converter.ascii_to_base_non_failing(&input[3], &self.config));
    }

}

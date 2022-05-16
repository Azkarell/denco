#![allow(dead_code)]

use crate::fast::FastConverter;
use crate::parallel::ParallelBase64;
use crate::single::Base64;

pub mod memory_efficient;
pub mod fast;
pub mod avx;
pub mod single;
pub mod parallel;

pub trait Encoder {
    fn encode(&self, input: &[u8]) -> String;
}

pub trait Decoder {
    fn decode(&self, input: &str) -> Vec<u8>;
}

pub trait FromConfig {
    fn from_config(config: &Config) -> Self;
}


pub fn parallel_encode(input: &[u8], config: Config) -> String {
    let encoder = ParallelBase64::<FastConverter>::new(config);
    encoder.encode(input)
}

pub fn parallel_decode(input: &str, config: Config) -> Vec<u8> {
    let decoder = ParallelBase64::<FastConverter>::new(config);
    decoder.decode(input)
}

pub fn decode(input: &str) -> Vec<u8> {
    let decoder = Base64::<FastConverter>::new(Config::DEFAULT);
    decoder.decode(input)
}

pub fn decode_with_config(input: &str, config: Config) -> Vec<u8> {
    let decoder = Base64::<FastConverter>::new(config);
    decoder.decode(input)
}

pub fn url_decode(input: &str) -> Vec<u8> {
    let decoder = Base64::<FastConverter>::new(Config::URL_SAFE);
    decoder.decode(input)
}

pub fn encode(input: &[u8]) -> String {
    let encoder = Base64::<FastConverter>::new(Config::DEFAULT);
    encoder.encode(input)
}

pub fn url_encode(input: &[u8]) -> String {
    let encoder = Base64::<FastConverter>::new(Config::URL_SAFE);
    encoder.encode(input)
}

pub fn encode_with_config(input: &[u8], config: Config) -> String {
    let encoder = Base64::<FastConverter>::new(config);
    encoder.encode(input)
}


pub struct Config {
    pub sixty_two: u8,
    pub sixty_three: u8,
    pub fill_marker: &'static [u8],
}

impl Config {
    pub const DEFAULT: Config = Config {
        sixty_two: b'+',
        sixty_three: b'/',
        fill_marker: b"=",
    };
    pub const URL_SAFE: Config = Config {
        sixty_two: b'-',
        sixty_three: b'_',
        fill_marker: b"",
    };
}

pub trait BaseAsciiConverter: FromConfig {
    /// should append the converted value to buffer
    fn base_to_ascii(&self, val: u8, buffer: &mut Vec<u8>, config: &Config);
    fn base_to_ascii_with_index(&self, val: u8, buffer: &mut [u8], config: &Config, index: usize);
    fn ascii_to_base(&self, val: &u8, config: &Config) -> Option<u8>;
    fn ascii_to_base_non_failing(&self, val: &u8, config: &Config) -> u8;
    fn num_unexpected_chars(&self) -> usize;
}






#[cfg(test)]
mod tests {
    use crate::{Config, decode, Decoder, encode, Encoder, FastConverter, url_decode, url_encode};
    use crate::parallel::ParallelBase64;

    #[test]
    fn encode_works() {
        let input = "4".as_bytes();
        let encoded = encode(input);
        assert_eq!("NA==", encoded);
    }

    #[test]
    fn decode_works() {
        let input = "dGVzdDE=";
        let decoded = decode(input);
        assert_eq!("test1", String::from_utf8(decoded).unwrap());
    }


    #[test]
    fn url_encode_works() {
        let input = "Hello I am a test string 1".as_bytes();
        let encoded = url_encode(input);
        assert_eq!("SGVsbG8gSSBhbSBhIHRlc3Qgc3RyaW5nIDE", encoded);
    }

    #[test]
    fn url_decode_works() {
        let input = "dGVzdA";
        let decoded = url_decode(input);
        assert_eq!("test", String::from_utf8(decoded).unwrap());
    }

    #[test]
    fn parallel_encode_works() {
        let input = "Hello I am a test string 1".as_bytes();
        let encoder = ParallelBase64 {
            config: Config::DEFAULT,
            converter: FastConverter::default(),
        };
        let res = encoder.encode(input);
        assert_eq!("SGVsbG8gSSBhbSBhIHRlc3Qgc3RyaW5nIDE=", res);
    }

    #[test]
    fn parallel_decode_works(){
        let input = "dGVzdA==";
        let decoder = ParallelBase64 {
            config: Config::DEFAULT,
            converter: FastConverter::default(),
        };
        let res = decoder.decode(input);
        assert_eq!("test".as_bytes(), res.as_slice());
    }
}


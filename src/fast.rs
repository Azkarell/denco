use std::sync::atomic::{AtomicUsize, Ordering};
use crate::{BaseAsciiConverter, Config, FromConfig};

#[derive(Debug)]
pub struct FastConverter {
    base_ascii: [u8; 64],
    ascii_base: [u8; u8::MAX as usize + 1],
    unexpected_char: AtomicUsize
}

impl FromConfig for FastConverter {
    fn from_config(config: &Config) -> Self {
        let mut base_ascii = [0u8; 64];
        let mut ascii_base = [u8::MAX; u8::MAX as usize + 1];
        for i in 0..64 {
            match i {
                0..=25 => {
                    base_ascii[i] = i as u8 + 65;
                    ascii_base[i + 65] = i as u8;
                }
                26..=51 => {
                    base_ascii[i] = i as u8 + 71;
                    ascii_base[i + 71] = i as u8;
                }
                52..=61 => {
                    base_ascii[i] = i as u8 - 4;
                    ascii_base[i - 4] = i as u8;
                }
                62 => {
                    base_ascii[i] = config.sixty_two;
                    ascii_base[config.sixty_two as usize] = i as u8;
                }
                63 => {
                    base_ascii[i] = config.sixty_three;
                    ascii_base[config.sixty_three as usize] = i as u8;
                }
                _ => unreachable!()
            }
        }

        FastConverter {
            base_ascii,
            ascii_base,
            unexpected_char: AtomicUsize::new(0)
        }
    }
}

impl Default for FastConverter {
    fn default() -> Self {
        FastConverter::from_config(&Config::DEFAULT)
    }
}

impl BaseAsciiConverter for FastConverter {
    #[inline]
    fn base_to_ascii(&self, val: u8, buffer: &mut Vec<u8>, _: &Config) {
        buffer.push(self.base_ascii[val as usize]);
    }
    #[inline]
    fn base_to_ascii_with_index(&self, val: u8, buffer: &mut [u8], _: &Config, index: usize) {
        buffer[index] = self.base_ascii[val as usize];
    }
    #[inline]
    fn ascii_to_base(&self, val: &u8, _: &Config) -> Option<u8> {
        let v = self.ascii_base[*val as usize];
        if v == u8::MAX {
            None
        } else {
            Some(v)
        }
    }
    #[inline]
    fn ascii_to_base_non_failing(&self, val: &u8, _: &Config) -> u8 {

        let val = self.ascii_base[*val as usize];
        if val == u8::MAX {
            self.unexpected_char.fetch_add(1, Ordering::Relaxed);
            0
        } else {
            val
        }
    }

    fn num_unexpected_chars(&self) -> usize {
        self.unexpected_char.load(Ordering::Relaxed)
    }
}
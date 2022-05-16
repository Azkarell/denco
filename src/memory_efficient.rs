use std::sync::atomic::AtomicUsize;
use crate::{BaseAsciiConverter, Config, FromConfig};

pub struct MemoryEfficientBaseAsciiConverter {
    unexpected_chars: AtomicUsize,
}

impl Default for MemoryEfficientBaseAsciiConverter {
    fn default() -> Self {
        Self {
            unexpected_chars: AtomicUsize::new(0),
        }
    }
}

impl FromConfig for MemoryEfficientBaseAsciiConverter {
    fn from_config(_: &Config) -> Self {
        Self{
            unexpected_chars: AtomicUsize::new(0),
        }
    }
}

impl BaseAsciiConverter for MemoryEfficientBaseAsciiConverter {
    fn base_to_ascii(&self, val: u8, bytes: &mut Vec<u8>, config: &Config) {
        match val {
            0..=25 => bytes.push(val + 65),
            26..=51 => bytes.push(val + 71),
            52..=61 => bytes.push(val - 4),
            62 => bytes.push(config.sixty_two),
            63 => bytes.push(config.sixty_three),
            _ => panic!("Invalid ascii value"),
        }
    }

    fn base_to_ascii_with_index(&self, val: u8, buffer: &mut [u8], config: &Config, index: usize) {
        match val {
            0..=25 => buffer[index] = val + 65,
            26..=51 => buffer[index] = val + 71,
            52..=61 => buffer[index] = val - 4,
            62 => buffer[index] = config.sixty_two,
            63 => buffer[index] = config.sixty_three,
            _ => panic!("Invalid ascii value"),
        }
    }

    fn ascii_to_base(&self, val: &u8, config: &Config) -> Option<u8> {
        if *val == config.sixty_two {
            return Some(62);
        }
        if *val == config.sixty_three {
            return Some(63);
        }

        match val {
            b'A'..=b'Z' => Some(val - 65),
            b'a'..=b'z' => Some(val - 71),
            b'0'..=b'9' => Some(val + 4),
            _ => {
                self.unexpected_chars.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                None
            },
        }
    }

    fn ascii_to_base_non_failing(&self, val: &u8, config: &Config) -> u8 {
        if *val == config.sixty_two {
            return 62;
        }
        if *val == config.sixty_three {
            return 63;
        }

        match val {
            b'A'..=b'Z' => val - 65,
            b'a'..=b'z' => val - 71,
            b'0'..=b'9' => val + 4,
            _ => {
                self.unexpected_chars.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                u8::MAX
            },
        }
    }

    fn num_unexpected_chars(&self) -> usize {
        return self.unexpected_chars.load(std::sync::atomic::Ordering::Relaxed);
    }
}

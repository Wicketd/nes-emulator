use std::ops::Shl;
use rust_embed::RustEmbed;

pub type Result<T = ()> = anyhow::Result<T>;

pub trait BitRead: Sized + Shl {
    fn read_bit(&self, n: u8) -> u8;
    fn is_bit_set(&self, n: u8) -> bool;
    fn is_bit_clear(&self, n: u8) -> bool;
}

impl BitRead for u8 {
    fn read_bit(&self, n: u8) -> u8 {
        assert!(n < 8);
        self & (1 << n)
    }

    fn is_bit_set(&self, n: u8) -> bool {
        self.read_bit(n) != 0
    }

    fn is_bit_clear(&self, n: u8) -> bool {
        self.read_bit(n) == 0
    }
}

#[derive(RustEmbed)]
#[folder = "assets/"]
pub struct Asset;

mod bits_len;
mod u64_rz;

pub const MAX_LEN_64: usize = 10;

/// `Rizzler` defines the codec's API.
pub trait Rizzler {
    fn put_u64rz(buf: &mut [u8], x: u64) -> Result<usize, &'static str>;
    fn u64rz(buf: &[u8]) -> Result<(u64, usize), &'static str>;
    fn size_u64rz(x: u64) -> usize;
}

pub struct Rizz64;

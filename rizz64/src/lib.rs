use std::fmt::{Display, Error as fmtError, Formatter};

mod tests;

pub const MAX_LEN_64: usize = 10;

#[derive(Debug)]
pub enum Error {
    BufferOverflow,
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmtError> {
        match *self {
            Error::BufferOverflow => {
                write!(f, "The number being read is larger than can be represented")
            }
        }
    }
}

pub struct Rizz64;
impl Rizz64 {
    #[inline]
    pub fn write_u64(buf: &mut [u8], mut x: u64) -> Result<usize, Error> {
        if Self::size_u64(x) > buf.len() {
            return Err(Error::BufferOverflow);
        }

        for i in 0..MAX_LEN_64 {
            let byte = (x & 0x7F) as u8;
            x >>= 7;

            let more = (x != 0) as u8;
            buf[i] = byte | (more << 7);

            if more == 0 {
                return Ok(i + 1);
            }
        }

        Err(Error::BufferOverflow)
    }

    #[inline]
    pub fn write_max_u64(buf: &mut [u8], x: u64) -> Result<usize, Error> {
        let mut read = Self::write_u64(buf, x)?;

        for i in read..MAX_LEN_64 {
            buf[i] = 0x80;
            read += 1;
        }

        Ok(read)
    }

    #[inline]
    pub fn read_u64(buf: &[u8]) -> Result<(u64, usize), Error> {
        let mut x = 0u64;
        let mut s: usize = 0;

        for (i, &b) in buf.iter().enumerate() {
            if i == MAX_LEN_64 {
                return Err(Error::BufferOverflow);
            }

            if b < 0x80 {
                if i == MAX_LEN_64 - 1 && b > 1 {
                    return Err(Error::BufferOverflow);
                }

                return Ok((x | (b as u64) << s, i + 1));
            }

            x |= ((b & 0x7f) as u64) << s;
            s += 7
        }

        return Ok((0, 0));
    }

    #[inline]
    pub fn size_u64(x: u64) -> usize {
        let bits = x.max(1).ilog2() + 1;
        ((9 * bits + 64) / 64) as usize
    }

    #[inline]
    pub fn write_i64(buf: &mut [u8], x: i64) -> Result<usize, Error> {
        Self::write_u64(buf, Self::zig_zag(x))
    }

    #[inline]
    pub fn read_i64(buf: &[u8]) -> Result<(i64, usize), Error> {
        let (ux, n) = Self::read_u64(buf)?;
        Ok((Self::decode_zig_zag(ux), n))
    }

    #[inline]
    pub fn write_max_i64(buf: &mut [u8], x: i64) -> Result<usize, Error> {
        Self::write_max_u64(buf, Self::zig_zag(x))
    }

    #[inline]
    pub fn size_i64(x: i64) -> usize {
        Self::size_u64(Self::zig_zag(x))
    }

    #[inline]
    fn zig_zag(x: i64) -> u64 {
        ((x << 1) ^ (x >> 63)) as u64
    }

    #[inline]
    fn decode_zig_zag(x: u64) -> i64 {
        ((x >> 1) as i64) ^ -((x & 1) as i64)
    }
}

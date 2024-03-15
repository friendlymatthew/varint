use std::fmt::{Display, Error as fmtError, Formatter};

mod tests;

pub const MAX_LEN_64: usize = 10;

#[derive(Debug)]
pub enum Error {
    BufferOverflow
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmtError> {
        match *self {
            Error::BufferOverflow=> {
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

        let mut i = 0;
        loop {
            let byte = (x & 0x7F) as u8;
            x >>= 7;
            let more = x != 0;

            unsafe {
                *buf.get_unchecked_mut(i) = byte | if more { 0x80 } else { 0 };
            }

            i += 1;
            if !more { return Ok(i); }
        }
    }

    /// `pack_u64` creates maximal-encoded variable integers. It does the following:
    ///
    ///  reserves maximum space, writes value, fills the rest with continuation bit
    #[inline]
    pub fn pack_u64(buf: &mut[u8], x: u64) -> Result<usize, Error> {
        let mut read = Self::write_u64(buf, x)?;

        while read < MAX_LEN_64 {
            unsafe {
                *buf.get_unchecked_mut(read) = 0x80;
            }

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
        return if x == 0 { 1 } else {
            ((64 - x.leading_zeros() as usize) + 6) / 7
        }
    }

    #[inline]
    pub fn write_i64(buf: &mut [u8], x: i64) -> Result<usize, Error> {
        Self::write_u64(buf, Self::zig_zag(x))
    }

    #[inline]
    pub fn read_i64(buf: &[u8]) -> Result<(i64, usize), Error> {
        let (ux, n) = Self::read_u64(buf)?;

        let mut x = (ux >> 1) as i64;
        if ux & 1 != 0 {
            x = !x;
        }

        Ok((x, n))
    }

    #[inline]
    pub fn pack_i64(buf: &mut[u8], x: i64) -> Result<usize, Error>{
        Self::pack_u64(buf, Self::zig_zag(x))
    }

    #[inline]
    pub fn size_i64(x: i64) -> usize {
        Self::size_u64(Self::zig_zag(x))
    }

    #[inline]
    fn zig_zag(x: i64) -> u64 {
        ((x << 1) ^ (x >> 63)) as u64
    }
}
